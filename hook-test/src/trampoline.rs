use iced_x86::Instruction;
use crate::function_decoder::FunctionDecoder;
use crate::isa_abi::IsaAbi;

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
        if total_bytes >= IA::JMP_INTERCEPTOR_BYTE_LEN {
            break;
        }
    }

    let instructions = rewrite_relative_instructions(instructions);

    Trampoline { instructions }
}

fn rewrite_relative_instructions(instructions: Vec<Instruction>) -> Vec<Instruction> {
    // TODO
    instructions
}
