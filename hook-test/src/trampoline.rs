use iced_x86::{Formatter, Instruction, NasmFormatter, Register};
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
    let mut a = CodeAssembler::new(IA::BITNESS).unwrap();
    a.mov(rax, (orig_addr + total_bytes) as u64).unwrap();
    a.jmp(rax).unwrap();
    instructions.extend(a.take_instructions());

    let instructions = rewrite_relative_instructions(instructions);

    Trampoline { instructions }
}

fn rewrite_relative_instructions(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut decoder = unsafe { FunctionDecoder::<X86_64_SystemV>::new(test_function as usize) };
    let push = decoder.decode();
    let mut mov = decoder.decode();

    println!("fn-addr: {:#x?}", test_function as usize);
    push.print();
    mov.print();
    mov.set_memory_displ_size(X86_64_SystemV::DISPL_SIZE);
    mov.print();
    mov.set_memory_base(Register::None);
    mov.print();

    // println!("{}", unsafe { FunctionDecoder::new(thiscall_function as usize) }.stack_argument_size());
    println!("{}", unsafe { FunctionDecoder::<X86_64_SystemV>::new(print as usize) }.stack_argument_size());

    // Instruction::with1(mov.code(), MemoryOperand::new(
    //     Register::None,
    //     Register::None,
    //     0,
    //     offset,
    //
    // ))

    instructions
}

trait InstructionFormat {
    fn nasm(&self) -> String;
    fn bytes(&self) -> String;
    fn print(&self);
}
impl InstructionFormat for Instruction {
    fn nasm(&self) -> String {
        let mut s = String::new();
        let mut formatter = NasmFormatter::new();
        formatter.format(self, &mut s);
        s
    }

    fn bytes(&self) -> String {
        match assemble::<X86_64_SystemV>(&[self.clone()], self.ip()) {
            Ok(bytes) => {
                let mut s = String::new();
                for byte in bytes {
                    s.push_str(&format!("{byte:02x} "));
                }
                s.pop();
                s
            }
            Err(e) => format!("invalid opcode: {e}"),
        }
    }

    fn print(&self) {
        println!("{:<36}    {}: {:#?}", self.bytes(), self.nasm(), self);
    }
}
