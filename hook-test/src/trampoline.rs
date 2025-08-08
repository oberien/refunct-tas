use std::collections::HashSet;
use iced_x86::{ConditionCode, FlowControl, Formatter, Instruction, IntelFormatter, MemorySizeOptions, OpKind};
use iced_x86::code_asm::CodeAssembler;
use crate::assemble;
use crate::function_decoder::FunctionDecoder;
use crate::isa_abi::{Array, IsaAbi, X86_64_SystemV};

pub struct Trampoline {
    pub instructions: Vec<Instruction>,
}

pub unsafe fn create_trampoline<IA: IsaAbi>(orig_addr: usize) -> Trampoline {
    let mut decoder = unsafe { FunctionDecoder::<IA>::new(orig_addr) };
    let mut instructions = Vec::new();
    let mut total_bytes = 0;
    loop {
        let inst = decoder.decode();
        total_bytes += inst.len();
        instructions.push(inst);
        if total_bytes >= IA::JmpInterceptorBytesArray::LEN {
            break;
        }
    }

    let free_reg = get_free_register::<IA>(&instructions);

    print_instructions(&instructions, orig_addr as u64, 0);
    let mut instructions = rewrite_relative_instructions::<IA>(instructions, free_reg);
    instructions.push(IA::create_mov_reg_addr(free_reg, orig_addr + total_bytes));
    instructions.push(IA::create_jmp_reg(free_reg));

    Trampoline { instructions }
}

fn get_free_register<IA: IsaAbi>(instructions: &[Instruction]) -> IA::AsmRegister {
    let mut used_registers = HashSet::new();
    for instruction in instructions.iter().cloned() {
        for op_num in 0..instruction.op_count() {
            match instruction.op_kind(op_num) {
                OpKind::Register => { used_registers.insert(instruction.op_register(op_num).full_register()); },
                OpKind::Memory => {
                    used_registers.insert(instruction.memory_base().full_register());
                    used_registers.insert(instruction.memory_index().full_register());
                }
                _ => (),
            }
        }
    }
    println!("Used Registers: {used_registers:#?}");

    IA::free_registers().iter().copied()
        .map(|reg| (reg, reg.into().full_register()))
        .filter(|(_, reg)| !used_registers.contains(reg))
        .map(|(asmreg, _)| asmreg)
        .next()
        .expect("no free usable register found")
}

// +--------------------------+---------------------------+
// | RIP-Relative Instruction | Absolute Equivalent       |
// +--------------------------+---------------------------+
// | mov rax, [rip + disp]    | mov rax, <absolute_addr>  |
// |                          | mov rax, [rax]            |
// +--------------------------+---------------------------+
// | lea rax, [rip + disp]    | mov rax, <absolute_addr>  |
// +--------------------------+---------------------------+
// | jmp [rip + disp]         | mov rax, <absolute_addr>  |
// | or jmp rel32             | jmp rax                   |
// +--------------------------+---------------------------+
// | call [rip + disp]        | mov rax, <absolute_addr>  |
// | or call rel32            | call rax                  |
// +--------------------------+---------------------------+
// | mov [rip + disp], rax    | mov rbx, <absolute_addr>  |
// |                          | mov [rbx], rax            |
// +--------------------------+---------------------------+
// | cmp rax, [rip + disp]    | mov rbx, <absolute_addr>  |
// |                          | cmp rax, [rbx]            |
// +--------------------------+---------------------------+
// | add rax, [rip + disp]    | mov rbx, <absolute_addr>  |
// |                          | add rax, [rbx]            |
// +--------------------------+---------------------------+
// | sub rax, [rip + disp]    | mov rbx, <absolute_addr>  |
// |                          | sub rax, [rbx]            |
// +--------------------------+---------------------------+
// | push [rip + disp]        | mov rax, <absolute_addr>  |
// |                          | push qword [rax]          |
// +--------------------------+---------------------------+
// | pop [rip + disp]         | mov rax, <absolute_addr>  |
// |                          | pop qword [rax]           |
// +--------------------------+---------------------------+
// | je <rel32>               | jne skip                  |
// |                          | mov rax, <absolute_addr>  |
// |                          | jmp rax                   |
// |                          | skip:                     |
// +--------------------------+---------------------------+

fn rewrite_relative_instructions<IA: IsaAbi>(instructions: Vec<Instruction>, free_reg: IA::AsmRegister) -> Vec<Instruction> {
    // println!("{}", unsafe { FunctionDecoder::new(thiscall_function as usize) }.stack_argument_size());
    // println!("{}", unsafe { FunctionDecoder::<X86_64_SystemV>::new(print as usize) }.stack_argument_size());

    let mut new_instructions = Vec::new();

    for instruction in instructions {
        println!("rewriting `{}`", instruction.nasm());
        let replacement = rewrite_memory_access::<IA>(instruction, free_reg)
            .or_else(|| rewrite_jump::<IA>(instruction, free_reg))
            .or_else(|| rewrite_call::<IA>(instruction, free_reg));
        if let Some(instructions) = replacement {
            println!("replaced");
            print_instructions(&[instruction], instruction.ip(), 4);
            println!("with");
            print_instructions(&instructions, instruction.ip(), 4);
            new_instructions.extend(instructions);
        } else {
            println!("no replacement needed for");
            print_instructions(&[instruction], instruction.ip(), 4);
            new_instructions.push(instruction);
        }
    }

    new_instructions
}

fn rewrite_memory_access<IA: IsaAbi>(mut instruction: Instruction, free_reg: IA::AsmRegister) -> Option<Vec<Instruction>> {
    for op_num in 0..instruction.op_count() {
        match instruction.op_kind(op_num) {
            OpKind::Register if instruction.op_register(op_num).is_ip() => todo!(),
            OpKind::MemorySegSI | OpKind::MemorySegESI | OpKind::MemorySegRSI | OpKind::MemorySegDI
            | OpKind::MemorySegEDI | OpKind::MemorySegRDI | OpKind::MemoryESDI | OpKind::MemoryESEDI
            | OpKind::MemoryESRDI => panic!("segmented memory not supported"),
            OpKind::Memory if instruction.memory_base().is_ip() || instruction.memory_index().is_ip() => {
                if instruction.memory_base().is_ip() {
                    instruction.set_memory_base(free_reg.into());
                    instruction.set_memory_displacement64(instruction.memory_displacement64().wrapping_sub(instruction.next_ip()));
                }
                if instruction.memory_index().is_ip() {
                    instruction.set_memory_index(free_reg.into());
                }
                return Some(vec![
                    IA::create_mov_reg_addr(free_reg, instruction.next_ip().try_into().unwrap()),
                    instruction,
                ])
            }
            _ => (),
        }
    }
    None
}
fn rewrite_jump<IA: IsaAbi>(instruction: Instruction, free_reg: IA::AsmRegister) -> Option<Vec<Instruction>> {
    match instruction.flow_control() {
        // no branch
        FlowControl::Next
        | FlowControl::Return
        // branch to register or memory; rip-relative memory handled in rewrite_memory_access
        | FlowControl::IndirectBranch
        // handled by rewrite_call
        | FlowControl::Call
        | FlowControl::IndirectCall
            => {}
        FlowControl::UnconditionalBranch | FlowControl::ConditionalBranch => {
            assert_eq!(instruction.op_count(), 1);
            match instruction.op0_kind() {
                OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => {
                    // TODO: check if the branch-target is also moved to the trampoline

                    // `jmp <label>` becomes `mov rax, <label>; jmp rax`
                    if instruction.flow_control() == FlowControl::UnconditionalBranch {
                        return Some(vec![
                            IA::create_mov_reg_addr(free_reg, instruction.near_branch_target() as usize),
                            IA::create_jmp_reg(free_reg),
                        ])
                    }

                    // `je <label>` becomes `jne skip; mov rax, <label>; jmp rax; skip:
                    let mut a = CodeAssembler::new(IA::BITNESS).unwrap();
                    let mut skip_label = a.create_label();
                    match instruction.condition_code() {
                        ConditionCode::None => unreachable!("we're checking ConditionalBranch, there must be a cc"),
                        ConditionCode::o => a.jno(skip_label).unwrap(),
                        ConditionCode::no => a.jo(skip_label).unwrap(),
                        ConditionCode::b => a.jae(skip_label).unwrap(),
                        ConditionCode::ae => a.jb(skip_label).unwrap(),
                        ConditionCode::e => a.jne(skip_label).unwrap(),
                        ConditionCode::ne => a.je(skip_label).unwrap(),
                        ConditionCode::be => a.ja(skip_label).unwrap(),
                        ConditionCode::a => a.jbe(skip_label).unwrap(),
                        ConditionCode::s => a.jns(skip_label).unwrap(),
                        ConditionCode::ns => a.js(skip_label).unwrap(),
                        ConditionCode::p => a.jnp(skip_label).unwrap(),
                        ConditionCode::np => a.jp(skip_label).unwrap(),
                        ConditionCode::l => a.jge(skip_label).unwrap(),
                        ConditionCode::ge => a.jl(skip_label).unwrap(),
                        ConditionCode::le => a.jg(skip_label).unwrap(),
                        ConditionCode::g => a.jle(skip_label).unwrap(),
                    };
                    a.add_instruction(IA::create_mov_reg_addr(free_reg, instruction.near_branch_target() as usize)).unwrap();
                    a.add_instruction(IA::create_jmp_reg(free_reg)).unwrap();
                    a.set_label(&mut skip_label).unwrap();
                    return Some(a.take_instructions())
                }
                // far branch jumps to a different segment -> not IP-relative
                OpKind::FarBranch16 | OpKind::FarBranch32 => {}
                kind => unreachable!("(un)conditional branch to {:?}", kind),
            }
        }
        // unsupported
        FlowControl::Interrupt | FlowControl::XbeginXabortXend | FlowControl::Exception => {
            unimplemented!("no support for rewriting {:?}", instruction.flow_control())
        }
    }
    None
}
fn rewrite_call<IA: IsaAbi>(instruction: Instruction, free_reg: IA::AsmRegister) -> Option<Vec<Instruction>> {
    // TODO
    None
}
// Original Function:
// 0:  48 8b 3d 04 00 00 00    mov    rdi, [rip+0x4]  # b <0xb>
// 7:  74 05                   je     [rip+0x5]       # e <end>
// 9:  e8 00 00 00 00          call   [rip]           # e <end>
// 000000000000000e <end>:
// e:  c3                      ret
//
// Overwritten Function:
// ; jump to our interceptor
// 0:  48 b8 ef cd ab 89 67    movabs rax,0x123456789abcdef <interceptor_address>
// 7:  45 23 01
// a:  ff e0                   jmp    rax
// ; keep the rest of the last instruction we overwrote
// c:  00 00
// 00000000000000 e <end>:
// e:  c3                      ret
//
// Trampoline:
// ; mov rdi, [rip+0x4]
// 0:  48 b8 0b 00 00 00 00    movabs rax,0xb
// 7:  00 00 00
// a:  48 8b 38                mov    rdi,QWORD PTR [rax]
// ; je e
// d:  75 0c                   jne    1b <skipped>
// f:  48 b8 0e 00 00 00 00    movabs rax,0xe
// 16: 00 00 00
// 19: ff e0                   jmp    rax
// 000000000000001b <skipped>:
// ; call e
// 1b: 48 b8 0e 00 00 00 00    movabs rax,0xe
// 22: 00 00 00
// 25: ff d0                   call   rax
// ; jump to original function
// 27: 48 b8 0e 00 00 00 00    movabs rax,0xe
// 2e: 00 00 00
// 31: ff e0                   jmp    rax

fn print_instructions(instructions: &[Instruction], mut ip: u64, indent: u8) {
    for instruction in instructions {
        ip = instruction.print(ip, indent);
    }
}
fn debug_instructions(instructions: &[Instruction], mut ip: u64){
    for instruction in instructions {
        ip = instruction.debug(ip);
    }
}

trait InstructionFormat {
    fn nasm(&self) -> String;
    fn bytes(&self, ip: u64) -> (u64, String);
    fn print(&self, ip: u64, indent: u8) -> u64;
    fn debug(&self, ip: u64) -> u64;
}
impl InstructionFormat for Instruction {
    fn nasm(&self) -> String {
        let mut s = String::new();
        let mut formatter = IntelFormatter::new();
        formatter.options_mut().set_rip_relative_addresses(true);
        formatter.options_mut().set_hex_prefix("0x");
        formatter.options_mut().set_hex_suffix("");
        formatter.options_mut().set_uppercase_hex(false);
        formatter.options_mut().set_space_after_operand_separator(true);
        formatter.options_mut().set_memory_size_options(MemorySizeOptions::Never);
        formatter.format(self, &mut s);
        s
    }

    fn bytes(&self,ip: u64) -> (u64, String) {
        match assemble::<X86_64_SystemV>(&[self.clone()], ip) {
            Ok(bytes) => {
                let mut s = String::new();
                for &byte in &bytes {
                    s.push_str(&format!("{byte:02x} "));
                }
                s.pop();
                (ip + bytes.len() as u64, s)
            }
            Err(e) => (ip, format!("invalid opcode: {e}")),
        }
    }

    fn print(&self, ip: u64, indent: u8) -> u64 {
        let (new_ip, bytes) = self.bytes(ip);
        println!("{:}{bytes:<36}    {}", " ".repeat(indent as usize), self.nasm());
        new_ip
    }
    fn debug(&self, ip: u64) -> u64 {
        let (new_ip, bytes) = self.bytes(ip);
        println!("{bytes:<36}    {}: {:#x?}", self.nasm(), self);
        new_ip
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::arch::naked_asm;
    use iced_x86::Code;

    fn clean_asm_string(s: &str) -> String {
        let mut res = String::new();
        for line in s.lines() {
            if line.trim().is_empty() {
                continue;
            }
            res.push_str(line.trim());
            res.push('\n');
        }
        res
    }

    macro_rules! test {
        ($isaabi:ty, $orig:literal, $res:literal,) => {
            #[unsafe(naked)]
            extern "C" fn naked_function_to_test() {
                naked_asm!(
                    $orig,
                    "ud2",
                )
            }

            let mut decoder = unsafe { FunctionDecoder::<$isaabi>::with_ip(naked_function_to_test as usize, 0x1000) };
            let mut instructions = Vec::new();
            let mut ip = 0x1000;
            loop {
                let mut instruction = decoder.decode();
                if instruction.code() == Code::Ud2 {
                    break;
                }
                instruction.set_ip(ip);
                ip += instruction.len() as u64;
                instructions.push(instruction);
            }
            let instructions = rewrite_relative_instructions::<$isaabi>(instructions, <$isaabi>::free_registers()[0]);
            let mut result = String::new();
            for instruction in instructions {
                result.push_str(&instruction.nasm());
                result.push('\n');
            }
            assert_eq!(clean_asm_string(&result), clean_asm_string($res));
        };
    }

    #[test]
    fn test_mov_reg_addr() {
        test!(X86_64_SystemV,
            r#"
                mov rdi, [rip+0xc]
                mov rdi, 0x1337
            "#,
            r#"
                mov rax, 0x1007
                mov rdi, [rax+0xc]
                mov rdi, 0x1337
            "#,
        );
    }

    #[test]
    fn test_mov_addr_reg() {
        test!(X86_64_SystemV,
            r#"
                mov [rip+0xc], rdi
                mov rdi, rsi
            "#,
            r#"
                mov rax, 0x1007
                mov [rax+0xc], rdi
                mov rdi, rsi
            "#,
        );
    }

    #[test]
    fn test_lea() {
        test!(X86_64_SystemV,
            r#"
                lea rdi, [rip+0xc]
                lea rdi, [0x1337]
            "#,
            r#"
                mov rax, 0x1007
                lea rdi, [rax+0xc]
                lea rdi, [0x1337]
            "#,
        );
    }

    #[test]
    fn test_cmp() {
        test!(X86_64_SystemV,
            r#"
                cmp rdi, [rip+0xc]
                cmp [rip+0xc], rdi
                cmp rdi, rsi
            "#,
            r#"
                mov rax, 0x1007
                cmp rdi, [rax+0xc]
                mov rax, 0x100e
                cmp [rax+0xc], rdi
                cmp rdi, rsi
            "#,
        );
    }

    #[test]
    fn test_add() {
        test!(X86_64_SystemV,
            r#"
                add rdi, [rip+0xc]
                add [rip+0xc], rdi
            "#,
            r#"
                mov rax, 0x1007
                add rdi, [rax+0xc]
                mov rax, 0x100e
                add [rax+0xc], rdi
            "#,
        );
    }

    #[test]
    fn test_push() {
        test!(X86_64_SystemV,
            r#"
                push [rip+0xc]
                push rdi
            "#,
            r#"
                mov rax, 0x1006
                push [rax+0xc]
                push rdi
            "#,
        );
    }

    #[test]
    fn test_pop() {
        test!(X86_64_SystemV,
            r#"
                pop [rip+0xc]
                pop rdi
            "#,
            r#"
                mov rax, 0x1006
                pop [rax+0xc]
                pop rdi
            "#,
        );
    }

    #[test]
    fn test_jmp_riprel() {
        test!(X86_64_SystemV,
            r#"
                jmp [rip+0xc]
            "#,
            r#"
                mov rax, 0x1006
                jmp [rax+0xc]
            "#,
        );
    }

    #[test]
    fn test_jmp_rel_outside_trampoline() {
        test!(X86_64_SystemV,
            r#"
                jmp 2f
                mov rdi, 0x1234567890abc
                mov rdi, 0x1234567890abc
                mov rdi, 0x1234567890abc
                2:
            "#,
            r#"
                mov rax, 0x1020
                jmp rax
                mov rdi, 0x1234567890abc
                mov rdi, 0x1234567890abc
                mov rdi, 0x1234567890abc
            "#,
        );
    }
    #[test]
    fn test_jmp_rel_inside_trampoline() {
        test!(X86_64_SystemV,
            r#"
                jmp 2f
                mov rdi, 0x1234567890abc
                2:
                mov rdi, 0x1234567890abc
                mov rdi, 0x1234567890abc
            "#,
            r#"
                jmp short 0x00000000000200c
                mov rdi, 0x1234567890abc
                mov rdi, 0x1234567890abc
                mov rdi, 0x1234567890abc
            "#,
        );
    }

// +--------------------------+---------------------------+
// | jmp [rip + disp]         | mov rax, <absolute_addr>  |
// | or jmp rel32             | jmp rax                   |
// +--------------------------+---------------------------+
// | je <rel32>               | jne skip                  |
// |                          | mov rax, <absolute_addr>  |
// |                          | jmp rax                   |
// |                          | skip:                     |
// +--------------------------+---------------------------+
// | call [rip + disp]        | mov rax, <absolute_addr>  |
// | or call rel32            | call rax                  |
// +--------------------------+---------------------------+

}
