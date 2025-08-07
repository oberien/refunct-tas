use std::collections::HashSet;
use iced_x86::{Formatter, Instruction, IntelFormatter, MasmFormatter, NasmFormatter, OpKind, Register};
use iced_x86::code_asm::{rax, CodeAssembler};
use crate::{assemble, print, test_function};
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

    instructions.push(IA::create_mov_reg_addr(free_reg, orig_addr + total_bytes));
    instructions.push(IA::create_jmp_reg(free_reg));

    print_instructions(&instructions, orig_addr as u64);
    let instructions = rewrite_relative_instructions::<IA>(instructions, free_reg);

    Trampoline { instructions }
}

fn get_free_register<IA: IsaAbi>(instructions: &[Instruction]) -> IA::AsmRegister {
    let mut used_registers = HashSet::new();
    for instruction in instructions.iter().cloned() {
        for op_num in 0..instruction.op_count() {
            let op_kind = instruction.op_kind(op_num);
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
        let replacement = rewrite_memory_access::<IA>(instruction, free_reg)
            .or_else(|| rewrite_jump::<IA>(instruction, free_reg))
            .or_else(|| rewrite_call::<IA>(instruction, free_reg));
        if let Some(instructions) = replacement {
            print!("replaced\n    ");
        }
        
        if let Some(instructions) = rewrite_memory_access::<IA>(instruction, free_reg) {
            new_instructions.extend(instructions);
        } else if let Some(instructions) = rewrite_jump::<IA>(instruction, free_reg) {
            new_instructions.extend(instructions);
        } else if let Some(instructions) = rewrite_call::<IA>(instruction, free_reg) {
            new_instructions.extend(instructions);
        } else {
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
    // TODO
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

fn print_instructions(instructions: &[Instruction], mut ip: u64) {
    for instruction in instructions {
        ip = instruction.print(ip);
    }
}
fn debug_instructions(instructions: &[Instruction], mut ip: u64){
    for instruction in instructions {
        ip = instruction.debug(ip);
    }
}

trait InstructionFormat {
    fn nasm(&self, ip: u64) -> String;
    fn bytes(&self, ip: u64) -> (u64, String);
    fn print(&self, ip: u64) -> u64;
    fn debug(&self, ip: u64) -> u64;
}
impl InstructionFormat for Instruction {
    fn nasm(&self, ip: u64) -> String {
        println!("{:#x}", ip);
        let mut s = String::new();
        let mut formatter = IntelFormatter::new();
        formatter.options_mut().set_rip_relative_addresses(true);
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

    fn print(&self, ip: u64) -> u64 {
        let (new_ip, bytes) = self.bytes(ip);
        println!("{bytes:<36}    {}", self.nasm(ip));
        new_ip
    }
    fn debug(&self, ip: u64) -> u64 {
        let (new_ip, bytes) = self.bytes(ip);
        println!("{bytes:<36}    {}: {:#x?}", self.nasm(ip), self);
        new_ip
    }
}
