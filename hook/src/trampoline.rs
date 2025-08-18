use std::collections::{HashMap, HashSet};
use std::ops::Range;
use iced_x86::{ConditionCode, FlowControl, Formatter, Instruction, IntelFormatter, MemorySizeOptions, Mnemonic, OpKind};
use iced_x86::code_asm::{CodeAssembler, CodeLabel, get_gpr32, get_gpr64};
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
            // `call 0; pop <reg>` is a common way on x86 to get the IP
            // -> also include the `pop <reg>` for easier rewriting
            if inst.is_call_near() && inst.near_branch_target() == inst.next_ip() {
                let inst = decoder.decode();
                if inst.mnemonic() == Mnemonic::Pop {
                    instructions.push(inst);
                    total_bytes += inst.len();
                }
            }
            break;
        }
    }

    let free_reg = get_free_register::<IA>(&instructions);

    print_instructions(&instructions, orig_addr as u64, 0);
    let rewriter = TrampolineRewriter::<IA>::new(instructions, free_reg);
    let mut a = rewriter.rewrite_relative_instructions();
    IA::create_mov_reg_addr(&mut a, free_reg, orig_addr + total_bytes).unwrap();
    IA::create_jmp_reg(&mut a, free_reg).unwrap();

    Trampoline { instructions: a.take_instructions() }
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

struct TrampolineRewriter<IA: IsaAbi> {
    orig_instructions: Vec<Instruction>,
    orig_addr_range: Range<u64>,
    free_reg: IA::AsmRegister,
    a: CodeAssembler,
    /// one label per original code instruction
    labels: HashMap<u64, CodeLabel>,
}

impl<IA: IsaAbi> TrampolineRewriter<IA> {
    pub fn new(orig_instructions: Vec<Instruction>, free_reg: IA::AsmRegister) -> Self {
        let orig_addr_range = orig_instructions.first().unwrap().ip()..orig_instructions.last().unwrap().next_ip();
        let mut a = CodeAssembler::new(IA::BITNESS).unwrap();
        let mut labels: HashMap<_, _> = orig_instructions.iter()
            .map(|i| (i.ip(), a.create_label()))
            .collect();
        // label to jump back to original function
        labels.insert(orig_instructions.last().unwrap().next_ip(), a.create_label());

        Self {
            orig_instructions,
            orig_addr_range,
            free_reg,
            a,
            labels,
        }
    }

    pub fn rewrite_relative_instructions(mut self) -> CodeAssembler {
        for i in 0..self.orig_instructions.len() {
            let instruction = self.orig_instructions[i];
            self.a.set_label(self.labels.get_mut(&instruction.ip()).unwrap()).unwrap();

            println!("rewriting `{}`", instruction.nasm());
            let before = self.a.instructions().len();
            let replacement = self.rewrite_memory_access(instruction)
                .or_else(|| self.rewrite_jump(instruction))
                .or_else(|| self.rewrite_call(instruction, i));
            if let Some(()) = replacement {
                println!("replaced");
                print_instructions(&[instruction], instruction.ip(), 4);
                println!("with");
                let added_instructions = &self.a.instructions()[before..];
                assert!(!added_instructions.is_empty());
                print_instructions(added_instructions, instruction.ip(), 4);
            } else {
                self.a.add_instruction(instruction).unwrap();
                println!("no replacement needed for");
                print_instructions(&[*self.a.instructions().last().unwrap()], instruction.ip(), 4);
            }
        }
        // set label to jump back to original function
        self.a.set_label(self.labels.get_mut(&self.orig_instructions.last().unwrap().next_ip()).unwrap()).unwrap();

        self.a
    }

    fn rewrite_memory_access(&mut self, mut instruction: Instruction) -> Option<()> {
        for op_num in 0..instruction.op_count() {
            match instruction.op_kind(op_num) {
                OpKind::Register if instruction.op_register(op_num).is_ip() => todo!(),
                OpKind::MemorySegSI | OpKind::MemorySegESI | OpKind::MemorySegRSI | OpKind::MemorySegDI
                | OpKind::MemorySegEDI | OpKind::MemorySegRDI | OpKind::MemoryESDI | OpKind::MemoryESEDI
                | OpKind::MemoryESRDI => panic!("segmented memory not supported"),
                OpKind::Memory if instruction.memory_base().is_ip() || instruction.memory_index().is_ip() => {
                    if instruction.memory_base().is_ip() {
                        instruction.set_memory_base(self.free_reg.into());
                        instruction.set_memory_displacement64(instruction.memory_displacement64().wrapping_sub(instruction.next_ip()));
                    }
                    if instruction.memory_index().is_ip() {
                        instruction.set_memory_index(self.free_reg.into());
                    }
                    IA::create_mov_reg_addr(&mut self.a, self.free_reg, instruction.next_ip().try_into().unwrap()).unwrap();
                    self.a.add_instruction(instruction).unwrap();
                    return Some(());
                }
                _ => (),
            }
        }
        None
    }

    fn rewrite_jump(&mut self, instruction: Instruction) -> Option<()> {
        match instruction.flow_control() {
            // no branch
            FlowControl::Next
            | FlowControl::Return
            // branch to register or memory; rip-relative memory handled in rewrite_memory_access
            | FlowControl::IndirectBranch
            | FlowControl::IndirectCall
            // handled by rewrite_call
            | FlowControl::Call
            => {}
            FlowControl::UnconditionalBranch | FlowControl::ConditionalBranch => {
                assert_eq!(instruction.op_count(), 1);
                match instruction.op0_kind() {
                    // handled by rewrite_memory_access
                    OpKind::Memory | OpKind::Register => return None,
                    OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => {
                        let jmp_into_trampoline = self.orig_addr_range.contains(&instruction.near_branch_target());

                        match (instruction.flow_control(), jmp_into_trampoline) {
                            // branch targets code copied into the trampoline
                            (FlowControl::UnconditionalBranch, true) => {
                                let label = *self.labels.get(&instruction.near_branch_target())
                                    .expect("jump to inside an instruction in the trampoline");
                                self.a.jmp(label).unwrap();
                                return Some(());
                            }
                            // `jmp <label>` becomes `mov rax, <label>; jmp rax`
                            (FlowControl::UnconditionalBranch, false) => {
                                IA::create_mov_reg_addr(&mut self.a, self.free_reg, instruction.near_branch_target() as usize).unwrap();
                                IA::create_jmp_reg(&mut self.a, self.free_reg).unwrap();
                                return Some(());
                            }
                            _ => (),
                        }

                        // `je <label>` becomes `jne skip; mov rax, <label>; jmp rax; skip:
                        let skip_label = self.labels[&instruction.next_ip()];
                        // invert condition
                        match instruction.condition_code() {
                            ConditionCode::None => unreachable!("we're checking ConditionalBranch, there must be a cc"),
                            ConditionCode::o => self.a.jno(skip_label).unwrap(),
                            ConditionCode::no => self.a.jo(skip_label).unwrap(),
                            ConditionCode::b => self.a.jae(skip_label).unwrap(),
                            ConditionCode::ae => self.a.jb(skip_label).unwrap(),
                            ConditionCode::e => self.a.jne(skip_label).unwrap(),
                            ConditionCode::ne => self.a.je(skip_label).unwrap(),
                            ConditionCode::be => self.a.ja(skip_label).unwrap(),
                            ConditionCode::a => self.a.jbe(skip_label).unwrap(),
                            ConditionCode::s => self.a.jns(skip_label).unwrap(),
                            ConditionCode::ns => self.a.js(skip_label).unwrap(),
                            ConditionCode::p => self.a.jnp(skip_label).unwrap(),
                            ConditionCode::np => self.a.jp(skip_label).unwrap(),
                            ConditionCode::l => self.a.jge(skip_label).unwrap(),
                            ConditionCode::ge => self.a.jl(skip_label).unwrap(),
                            ConditionCode::le => self.a.jg(skip_label).unwrap(),
                            ConditionCode::g => self.a.jle(skip_label).unwrap(),
                        };
                        if jmp_into_trampoline {
                            let label = *self.labels.get(&instruction.near_branch_target())
                                .expect("jump to inside an instruction in the trampoline");
                            self.a.jmp(label).unwrap();
                            return Some(());
                        } else {
                            IA::create_mov_reg_addr(&mut self.a, self.free_reg, instruction.near_branch_target() as usize).unwrap();
                            IA::create_jmp_reg(&mut self.a, self.free_reg).unwrap();
                        }
                        return Some(())
                    }
                    OpKind::FarBranch16 | OpKind::FarBranch32 => unimplemented!("for branch not supported: {instruction:?}"),
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
    fn rewrite_call(&mut self, instruction: Instruction, orig_instruction_num: usize) -> Option<()> {
        match instruction.flow_control() {
            // no branch
            FlowControl::Next
            | FlowControl::Return
            // branch to register or memory; rip-relative memory handled in rewrite_memory_access
            | FlowControl::IndirectBranch
            | FlowControl::IndirectCall
            // handled by rewrite_jump
            | FlowControl::UnconditionalBranch
            | FlowControl::ConditionalBranch
                => return None,
            | FlowControl::Call => (),
            // unsupported
            FlowControl::Interrupt | FlowControl::XbeginXabortXend | FlowControl::Exception => {
                unimplemented!("no support for rewriting {:?}", instruction.flow_control())
            }
        }
        assert_eq!(instruction.op_count(), 1);
        match instruction.op0_kind() {
            OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => (),
            // handled by rewrite_memory_access
            OpKind::Memory | OpKind::Register => return None,
            OpKind::FarBranch16 | OpKind::FarBranch32 => unimplemented!("for call not supported: {instruction:?}"),
            kind => unreachable!("call to {:?}", kind),
        }

        let call_into_trampoline = self.orig_addr_range.contains(&instruction.near_branch_target());

        if call_into_trampoline {
            // the `call`-instruction targets code which was copied into the trampoline

            // special case: `call 0; pop <reg>` is a common pattern on x86 to get the IP
            let next_instruction_is_pop = self.orig_instructions.get(orig_instruction_num+1)
                .map(|i| i.mnemonic() == Mnemonic::Pop)
                .unwrap_or_default();
            if instruction.near_branch_target() == instruction.next_ip() && next_instruction_is_pop {
                let next_instruction = self.orig_instructions[orig_instruction_num+1];
                assert_eq!(next_instruction.op0_kind(), OpKind::Register);
                let reg = next_instruction.op0_register();
                if let Some(reg) = get_gpr64(reg) {
                    self.a.mov(reg, instruction.next_ip()).unwrap();
                } else if let Some(reg) = get_gpr32(reg) {
                    self.a.mov(reg, instruction.next_ip() as u32).unwrap();
                } else {
                    unimplemented!("found `call 0; pop <reg>` with unsupported register {:?}", reg);
                }
                return Some(())
            }

            // otherwise call into the copied code
            let label = *self.labels.get(&instruction.near_branch_target())
                .expect("call to inside an instruction in the trampoline");
            self.a.call(label).unwrap();
        } else {
            IA::create_mov_reg_addr(&mut self.a, self.free_reg, instruction.near_branch_target() as usize).unwrap();
            IA::create_call_reg(&mut self.a, self.free_reg).unwrap();
        }
        Some(())
    }
}

#[allow(unused)]
fn print_instructions(instructions: &[Instruction], mut ip: u64, indent: u8) {
    for instruction in instructions {
        ip = instruction.print(ip, indent);
    }
}
#[allow(unused)]
fn debug_instructions(instructions: &[Instruction], mut ip: u64){
    for instruction in instructions {
        ip = instruction.debug(ip);
    }
}

trait InstructionFormat {
    #[allow(unused)]
    fn nasm(&self) -> String;
    #[allow(unused)]
    fn bytes(&self, ip: u64) -> (u64, String);
    #[allow(unused)]
    fn print(&self, ip: u64, indent: u8) -> u64;
    #[allow(unused)]
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
        println!("{}{:16x}: {bytes:<36}    {}", " ".repeat(indent as usize), self.ip(), self.nasm());
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
            let mut a = TrampolineRewriter::<$isaabi>::new(instructions, <$isaabi>::free_registers()[0])
                .rewrite_relative_instructions();
            let instructions = a.take_instructions();
            let mut result = String::new();
            for instruction in instructions {
                result.push_str(&format!("{:x}: ", instruction.ip()));
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
               1: mov rax, 0x1007
            1000: mov rdi, [rax+0xc]
               2: mov rdi, 0x1337
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
               1: mov rax, 0x1007
            1000: mov [rax+0xc], rdi
               2: mov rdi, rsi
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
               1: mov rax, 0x1007
            1000: lea rdi, [rax+0xc]
               2: lea rdi, [0x1337]
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
               1: mov rax, 0x1007
            1000: cmp rdi, [rax+0xc]
               2: mov rax, 0x100e
            1007: cmp [rax+0xc], rdi
               3: cmp rdi, rsi
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
               1: mov rax, 0x1007
            1000: add rdi, [rax+0xc]
               2: mov rax, 0x100e
            1007: add [rax+0xc], rdi
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
               1: mov rax, 0x1006
            1000: push [rax+0xc]
               2: push rdi
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
               1: mov rax, 0x1006
            1000: pop [rax+0xc]
               2: pop rdi
            "#,
        );
    }

    #[test]
    fn test_call_riprel() {
        test!(X86_64_SystemV,
            r#"
                call [rip+0xc]
                call rdi
            "#,
            r#"
               1: mov rax, 0x1006
            1000: call [rax+0xc]
               2: call rdi
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
               1: mov rax, 0x1006
            1000: jmp [rax+0xc]
            "#,
        );
    }

    #[test]
    fn test_jmp_rel_outside_trampoline() {
        test!(X86_64_SystemV,
            r#"
                jmp 2f
                mov rdi, 0x42
                2:
            "#,
            r#"
               1: mov rax, 0x1009
               0: jmp rax
               2: mov rdi, 0x42
            "#,
        );
    }
    #[test]
    fn test_jmp_rel_inside_trampoline() {
        test!(X86_64_SystemV,
            r#"
                jmp 2f
                mov rdi, 0x42
                2:
                mov rdi, 0x1337
            "#,
            r#"
               1: jmp short 3
               2: mov rdi, 0x42
               3: mov rdi, 0x1337
            "#,
        );
    }

    #[test]
    fn test_jcc_rel_outside_trampoline() {
        test!(X86_64_SystemV,
            r#"
                jb 2f
                mov rdi, 0x42
                2:
            "#,
            r#"
               1: jae short 2
               0: mov rax, 0x1009
               0: jmp rax
               2: mov rdi, 0x42
            "#,
        );
    }
    #[test]
    fn test_jcc_rel_inside_trampoline() {
        test!(X86_64_SystemV,
            r#"
                jg 2f
                mov rdi, 0x42
                2:
                mov rdi, 0x1337
            "#,
            r#"
               1: jle short 2
               0: jmp short 3
               2: mov rdi, 0x42
               3: mov rdi, 0x1337
            "#,
        );
    }

    #[test]
    fn test_call_rel_outside_trampoline() {
        test!(X86_64_SystemV,
            r#"
                call 2f
                mov rdi, 0x42
                2:
            "#,
            r#"
               1: mov rax, 0x100c
               0: call rax
               2: mov rdi, 0x42
            "#,
        );
    }
    #[test]
    fn test_call_rel_inside_trampoline() {
        test!(X86_64_SystemV,
            r#"
                call 2f
                mov rdi, 0x42
                2:
                mov rdi, 0x1337
            "#,
            r#"
               1: call 3
               2: mov rdi, 0x42
               3: mov rdi, 0x1337
            "#,
        );
    }
}
