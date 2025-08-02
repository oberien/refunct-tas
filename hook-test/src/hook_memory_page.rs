use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use memmap2::MmapMut;
use crate::{assemble, Hook, Interceptor};
use crate::isa_abi::IsaAbi;
use crate::trampoline::Trampoline;

#[must_use]
pub struct HookMemoryPageBuilder<IA: IsaAbi> {
    map: MmapMut,
    _marker: PhantomData<IA>,
}

impl<IA: IsaAbi> HookMemoryPageBuilder<IA> {
    pub fn new() -> Self {
        let mut map = MmapMut::map_anon(8192).unwrap();

        let hook_struct = Hook::default();
        unsafe {
            // make sure the map is Hook-aligned
            assert_eq!(map.as_ptr() as usize % align_of::<Hook<IA>>(), 0);
            let hook_struct_ptr = map.as_mut_ptr() as *mut Hook<IA>;
            // SAFETY:
            // * `dst` is valid for writes: we have MmapMut
            // * `dst` is properly aligned: see above alignment check
            *hook_struct_ptr = hook_struct;
        }

        Self {
            map,
            _marker: PhantomData,
        }
    }

    pub fn trampoline(mut self, trampoline: Trampoline) -> HookMemoryPageBuilderWithTrampoline<IA> {
        let addr = self.trampoline_addr();
        let code = assemble::<IA>(&trampoline.instructions, addr as u64).unwrap();
        self.map[addr..][..code.len()].copy_from_slice(&code);
        HookMemoryPageBuilderWithTrampoline {
            builder: self,
            trampoline_len: code.len(),
        }
    }

    pub fn set_hook_struct(&mut self, hook_struct: Hook<IA>) {
        unsafe {
            let ptr = self.map.as_mut_ptr() as *mut Hook<IA>;
            // SAFETY: the struct was initialized properly at that address in Self::new
            *ptr = hook_struct
        }
    }
    pub fn page_addr(&self) -> usize {
        self.map.as_ptr() as usize
    }
    pub fn hook_struct_offset(&self) -> usize {
        0
    }
    #[expect(unused)]
    pub fn hook_struct_addr(&self) -> usize {
        self.page_addr() + self.hook_struct_offset()
    }
    pub fn trampoline_offset(&self) -> usize {
        ((self.hook_struct_offset() + size_of::<Hook<IA>>() + 15) / 16) * 16
    }
    pub fn trampoline_addr(&self) -> usize {
        self.page_addr() + self.trampoline_offset()
    }
}

#[must_use]
pub struct HookMemoryPageBuilderWithTrampoline<IA: IsaAbi> {
    builder: HookMemoryPageBuilder<IA>,
    trampoline_len: usize,
}
impl<IA: IsaAbi> Deref for HookMemoryPageBuilderWithTrampoline<IA> {
    type Target = HookMemoryPageBuilder<IA>;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}
impl<IA: IsaAbi> DerefMut for HookMemoryPageBuilderWithTrampoline<IA> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}

impl<IA: IsaAbi> HookMemoryPageBuilderWithTrampoline<IA> {
    pub fn interceptor(mut self, interceptor: Interceptor) -> HookMemoryPageBuilderFinished<IA> {
        let addr = self.interceptor_addr();
        let code = assemble::<IA>(&interceptor.instructions, addr as u64).unwrap();
        self.builder.map[addr..][..code.len()].copy_from_slice(&code);
        HookMemoryPageBuilderFinished {
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

#[must_use]
pub struct HookMemoryPageBuilderFinished<IA: IsaAbi> {
    builder: HookMemoryPageBuilderWithTrampoline<IA>,
    interceptor_len: usize,
}
impl<IA: IsaAbi> Deref for HookMemoryPageBuilderFinished<IA> {
    type Target = HookMemoryPageBuilderWithTrampoline<IA>;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}
impl<IA: IsaAbi> DerefMut for HookMemoryPageBuilderFinished<IA> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}

impl<IA: IsaAbi> HookMemoryPageBuilderFinished<IA> {
    #[expect(unused)]
    pub fn interceptor_len(&self) -> usize {
        self.interceptor_len
    }
    pub fn finalize(self) -> &'static Hook<IA> {
        let map = self.builder.builder.map.make_exec().unwrap();
        unsafe {
            let ptr = map.as_ptr() as *const Hook<IA>;
            // SAFETY:
            // * the struct was initialized properly at that address in HookMemoryPageBuilder::new
            // * we consume all references to the Mmap / MmapMut, leaking it and making the
            //   memory unmodifiable and static
            &*ptr
        }
    }
}
