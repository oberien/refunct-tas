use std::marker::PhantomData;
use std::slice;
use iced_x86::{Decoder, DecoderOptions, FlowControl, Instruction, OpKind};
use crate::isa_abi::IsaAbi;

pub struct FunctionDecoder<IA: IsaAbi> {
    addr: usize,
    ip: u64,
    read: usize,
    _marker: PhantomData<IA>,
}
impl<IA: IsaAbi> Clone for FunctionDecoder<IA> {
    fn clone(&self) -> Self {
        Self {
            addr: self.addr,
            ip: self.ip,
            read: self.read,
            _marker: self._marker,
        }
    }
}
impl<IA: IsaAbi> FunctionDecoder<IA> {
    /// # Safety
    /// * `addr` must point to a valid address
    /// * `addr` must live at least as long as this struct
    /// * `addr` must have at least as many bytes after it as needed for all
    ///    calls to `decode`, i.e., the memory region can't end prematurely
    pub unsafe fn new(addr: usize) -> Self {
        Self {
            addr,
            ip: addr as u64,
            read: 0,
            _marker: PhantomData,
        }
    }
    /// for Safety see Self::new
    #[allow(unused)]
    pub unsafe fn with_ip(addr: usize, ip: u64) -> Self {
        Self {
            addr,
            ip,
            read: 0,
            _marker: PhantomData,
        }
    }

    unsafe fn decoder(&self) -> Decoder<'static> {
        // non-contrived x86_64 instructions are max 15 bytes
        let slice = unsafe { slice::from_raw_parts(self.addr as *const u8, 15) };
        Decoder::with_ip(IA::BITNESS, slice, self.ip, DecoderOptions::NONE)
    }

    pub fn decode(&mut self) -> Instruction {
        let instruction = unsafe { self.decoder() }.decode();
        if instruction.is_invalid() {
            panic!("decoded invalid instruction: {instruction:?}");
        }
        self.addr += instruction.len();
        self.ip += instruction.len() as u64;
        self.read += instruction.len();
        instruction
    }

    /// Number of bytes of arguments passed via the stack
    ///
    /// Gotten by decoding until the first `ret` instruction and taking its immediate
    /// if it exists, e.g. `retn 16`, or `0` if there isn't any, e.g. `ret`.
    /// Resets the Decoder afterwards.
    pub fn stack_argument_size(&self) -> u16 {
        let mut decoder = (*self).clone();
        loop {
            let instruction = decoder.decode();
            if instruction.flow_control() != FlowControl::Return {
                continue;
            }
            if instruction.op_count() == 0 {
                return 0;
            }
            assert_eq!(instruction.op_count(), 1);
            assert_eq!(instruction.op0_kind(), OpKind::Immediate16);
            return instruction.immediate16();
        }
    }
}

