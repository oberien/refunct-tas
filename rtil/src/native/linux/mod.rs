use std::env;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use libc::{self, c_void, PROT_READ, PROT_WRITE, PROT_EXEC};

// Shoutout to https://github.com/geofft/redhook/blob/master/src/ld_preload.rs#L18
// Rust doesn't directly expose __attribute__((constructor)), but this
// is how GNU implements it.
#[link_section=".init_array"]
pub static INITIALIZE_CTOR: extern "C" fn() = crate::initialize;

// extern "C" {
//     fn dlinfo(handle: *mut c_void, request: c_int, info: *mut c_void) -> c_int;
// }
// const RTLD_DI_LINKMAP: c_int = 2;
//
// pub fn base_address() -> usize {
//     #[derive(Debug)]
//     #[repr(C)]
//     struct LinkMap {
//         addr: isize,
//         name: *mut c_char,
//         l_ld: usize,
//         l_next: *mut LinkMap,
//         l_prev: *mut LinkMap,
//     }
//     let base_offset = unsafe {
//         let handle = libc::dlopen(ptr::null(), libc::RTLD_LAZY);
//         let mut ptr: *mut LinkMap = ptr::null_mut();
//         let ret = dlinfo(handle, RTLD_DI_LINKMAP, (&mut ptr) as *mut _ as *mut c_void);
//         assert_eq!(ret, 0);
//         (*ptr).addr
//     };
//     let current_exe = env::current_exe().unwrap();
//     let data = fs::read(current_exe).unwrap();
//     let elf_object = object::File::parse(&*data).unwrap();
//     // get first LOAD header
//     let elf_base_address = elf_object.segments().next().unwrap().address();
//
//     (elf_base_address as isize + base_offset) as usize
// }

macro_rules! find {
    ($($name:ident, $symbol:expr,)*) => {
        $(
            pub(in crate::native) static $name: AtomicUsize = AtomicUsize::new(0);
        )*
        const NAMES: &[&str] = &[
            $(
                $symbol,
            )*
        ];

        pub(in crate::native) fn init() {
            let addrs: HashMap<_, _> = dynsym::iter(env::current_exe().unwrap()).into_iter()
                .filter_map(|(name, addr)| NAMES.iter()
                    .find(|&&pattern| {
                        if let Some(pattern) = pattern.strip_prefix('^') {
                            name.starts_with(pattern)
                        } else {
                            name.contains(pattern)
                        }
                    })
                    .map(|&name| (name, addr)))
                .collect();
            log!("{:?}", addrs);
            let mut i = 0;
            $(
                $name.store(*addrs.get(NAMES[i]).unwrap(), Ordering::SeqCst);
                log!("found {}: {:#x}", NAMES[i], $name.load(Ordering::SeqCst));
                #[allow(unused_assignments)]
                { i += 1 };
            )*
        }
    }
}

find! {
    FSLATEAPPLICATION_TICK, "^FSlateApplication::Tick()",
    FSLATEAPPLICATION_ONKEYDOWN, "^FSlateApplication::OnKeyDown(int, unsigned int, bool)",
    FSLATEAPPLICATION_ONKEYUP, "^FSlateApplication::OnKeyUp(int, unsigned int, bool)",
    FSLATEAPPLICATION_ONRAWMOUSEMOVE, "^FSlateApplication::OnRawMouseMove(int, int)",
    FPLATFORMMISC_PUMPMESSAGES, "^FLinuxPlatformMisc::PumpMessages(bool)",
    UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE, "^UEngine::UpdateTimeAndHandleMaxTickRate()",
    AMYCHARACTER_TICK, "^AMyCharacter::Tick(float)",
    AMYCHARACTER_FORCEDUNCROUCH, "^AMyCharacter::ForcedUnCrouch()",
    FAPP_DELTATIME, "^FApp::DeltaTime",
    FMEMORY_MALLOC, "^FMemory::Malloc(unsigned long, unsigned int)",
    FMEMORY_FREE, "^FMemory::Free(void*)",
    FNAME_FNAME, "^FName::FName(wchar_t const*, EFindName)",
    AMYHUD_DRAWHUD, "^AMyHUD::DrawHUD()",
    AHUD_DRAWLINE, "^AHUD::DrawLine(float, float, float, float, FLinearColor, float)",
    AHUD_DRAWTEXT, "^AHUD::DrawText(FString const&, FLinearColor, float, float, UFont*, float, bool)",
    AHUD_PROJECT, "^AHUD::Project(FVector)",
    AHUD_GETTEXTSIZE, "^AHUD::GetTextSize(FString const&, float&, float&, UFont*, float)",
    GWORLD, "^GWorld",
    UWORLD_SPAWNACTOR, "^UWorld::SpawnActor(UClass*, FVector const*, FRotator const*, FActorSpawnParameters const&)",
    UWORLD_DESTROYACTOR, "^UWorld::DestroyActor(AActor*, bool, bool)",
    AMYCHARACTER_STATICCLASS, "^AMyCharacter::StaticClass()",
    APAWN_SPAWNDEFAULTCONTROLLER, "^APawn::SpawnDefaultController()",
    AACTOR_SETACTORENABLECOLLISION, "^AActor::SetActorEnableCollision(bool)",
    UGAMEPLAYSTATICS_GETACCURATEREALTIME, "^UGameplayStatics::GetAccurateRealTime(UObject const*, int&, float&)",
}

pub(in crate::native) fn make_rw(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    unsafe { libc::mprotect(page, 0x1000, PROT_READ | PROT_WRITE); }
}

pub(in crate::native) fn make_rx(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    unsafe { libc::mprotect(page, 0x1000, PROT_READ | PROT_EXEC); }
}
