use std::marker::PhantomData;
use std::{mem, ptr};
use std::ops::Deref;
use memmap2::MmapMut;
use crate::{assemble, CallTrampoline, RawHook, Interceptor};
use crate::isa_abi::IsaAbi;
use crate::trampoline::Trampoline;

#[must_use]
pub struct HookMemoryPageBuilder<IA: IsaAbi, T> {
    map: MmapMut,
    _marker: PhantomData<(IA, T)>,
}
impl<IA: IsaAbi, T: 'static> HookMemoryPageBuilder<IA, T> {
    pub fn new() -> Self {
        Self {
            map: MmapMut::map_anon(8192).unwrap(),
            _marker: PhantomData,
        }
    }

    pub fn trampoline(mut self, trampoline: Trampoline) -> HookMemoryPageBuilderWithTrampoline<IA, T> {
        let addr = self.trampoline_addr();
        let offset = self.trampoline_offset();
        let code = assemble::<IA>(&trampoline.instructions, addr as u64).unwrap();
        self.map[offset..][..code.len()].copy_from_slice(&code);
        HookMemoryPageBuilderWithTrampoline {
            builder: self,
            trampoline_len: code.len(),
        }
    }

    pub fn page_addr(&self) -> usize {
        self.map.as_ptr().addr()
    }
    pub fn hook_struct_offset(&self) -> usize {
        0
    }
    pub fn hook_struct_addr(&self) -> usize {
        self.page_addr() + self.hook_struct_offset()
    }
    pub fn trampoline_offset(&self) -> usize {
        ((self.hook_struct_offset() + size_of::<RawHook<IA, T>>() + 15) / 16) * 16
    }
    pub fn trampoline_addr(&self) -> usize {
        self.page_addr() + self.trampoline_offset()
    }
}

// --------------------------------

#[must_use]
pub struct HookMemoryPageBuilderWithTrampoline<IA: IsaAbi, T> {
    builder: HookMemoryPageBuilder<IA, T>,
    trampoline_len: usize,
}
impl<IA: IsaAbi, T: 'static> Deref for HookMemoryPageBuilderWithTrampoline<IA, T> {
    type Target = HookMemoryPageBuilder<IA, T>;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}
impl<IA: IsaAbi, T: 'static> HookMemoryPageBuilderWithTrampoline<IA, T> {
    pub fn interceptor(mut self, interceptor: Interceptor) -> HookMemoryPageBuilderWithInterceptor<IA, T> {
        let addr = self.interceptor_addr();
        let offset = self.interceptor_offset();
        let code = assemble::<IA>(&interceptor.instructions, addr as u64).unwrap();
        self.builder.map[offset..][..code.len()].copy_from_slice(&code);
        HookMemoryPageBuilderWithInterceptor {
            builder: self,
            interceptor_len: code.len(),
        }
    }

    #[expect(unused)]
    pub fn trampoline_len(&self) -> usize {
        self.trampoline_len
    }
    pub fn interceptor_offset(&self) -> usize {
        ((self.trampoline_offset() + self.trampoline_len + 15) / 16) * 16
    }
    pub fn interceptor_addr(&self) -> usize {
        self.page_addr() + self.interceptor_offset()
    }
}

// --------------------------------

#[must_use]
pub struct HookMemoryPageBuilderWithInterceptor<IA: IsaAbi, T> {
    builder: HookMemoryPageBuilderWithTrampoline<IA, T>,
    interceptor_len: usize,
}
impl<IA: IsaAbi, T> Deref for HookMemoryPageBuilderWithInterceptor<IA, T> {
    type Target = HookMemoryPageBuilderWithTrampoline<IA, T>;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}
impl<IA: IsaAbi, T: 'static> HookMemoryPageBuilderWithInterceptor<IA, T> {
    pub fn call_trampoline(mut self, call_trampoline: CallTrampoline) -> HookMemoryPageBuilderFinished<IA, T> {
        let addr = self.call_trampoline_addr();
        let offset = self.call_trampoline_offset();
        let code = assemble::<IA>(&call_trampoline.instructions, addr as u64).unwrap();
        self.builder.builder.map[offset..][..code.len()].copy_from_slice(&code);
        HookMemoryPageBuilderFinished {
            builder: self,
            call_trampoline_len: code.len(),
        }
    }

    #[expect(unused)]
    pub fn interceptor_len(&self) -> usize {
        self.interceptor_len
    }
    pub fn call_trampoline_offset(&self) -> usize {
        ((self.interceptor_offset() + self.interceptor_len + 15) / 16) * 16
    }
    pub fn call_trampoline_addr(&self) -> usize {
        self.page_addr() + self.call_trampoline_offset()
    }
}

// --------------------------------

#[must_use]
pub struct HookMemoryPageBuilderFinished<IA: IsaAbi, T> {
    builder: HookMemoryPageBuilderWithInterceptor<IA, T>,
    call_trampoline_len: usize,
}
impl<IA: IsaAbi, T> Deref for HookMemoryPageBuilderFinished<IA, T> {
    type Target = HookMemoryPageBuilderWithInterceptor<IA, T>;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl<IA: IsaAbi, T> HookMemoryPageBuilderFinished<IA, T> {
    #[expect(unused)]
    pub fn call_trampoline_len(&self) -> usize {
        self.call_trampoline_len
    }
    pub fn finalize(mut self, hook_struct: RawHook<IA, T>) -> &'static RawHook<IA, T> {
        unsafe {
            let ptr = self.builder.builder.builder.map.as_mut_ptr();
            // make sure the map is Hook-aligned
            assert_eq!(ptr.addr() % align_of::<RawHook<IA, T>>(), 0);
            let hook_struct_ptr = ptr as *mut RawHook<IA, T>;
            // SAFETY:
            // * `dst` is valid for writes: we have MmapMut
            // * `dst` is properly aligned: see previous alignment check
            // * `dst` is currently uninitialized and thus doesn't need to be dropped
            ptr::write(hook_struct_ptr, hook_struct)
        }
        let map = self.builder.builder.builder.map.make_exec().unwrap();
        unsafe {
            let ptr = map.as_ptr() as *const RawHook<IA, T>;
            mem::forget(map);
            // SAFETY:
            // * the struct was just initialized properly at that address
            // * we leak the Mmap, making the memory static
            // * we converted the MmapMut into an Mmap, making the memory unmodifiable
            &*ptr
        }
    }
}
