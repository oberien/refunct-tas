use std::env;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    FSLATEAPPLICATION_ONMOUSEMOVE, "^FSlateApplication::OnMouseMove()",
    FSLATEAPPLICATION_ONMOUSEDOWN, "^FSlateApplication::OnMouseDown(TSharedPtr<FGenericWindow, (ESPMode)0> const&, EMouseButtons::Type, FVector2D)",
    FSLATEAPPLICATION_ONMOUSEDOUBLECLICK, "^FSlateApplication::OnMouseDoubleClick(TSharedPtr<FGenericWindow, (ESPMode)0> const&, EMouseButtons::Type, FVector2D)",
    FSLATEAPPLICATION_ONMOUSEUP, "^FSlateApplication::OnMouseUp(EMouseButtons::Type, FVector2D)",
    FSLATEAPPLICATION_ONMOUSEWHEEL, "^FSlateApplication::OnMouseWheel(float, FVector2D)",
    FPLATFORMMISC_PUMPMESSAGES, "^FLinuxPlatformMisc::PumpMessages(bool)",
    UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE, "^UEngine::UpdateTimeAndHandleMaxTickRate()",
    AMYCHARACTER_TICK, "^AMyCharacter::Tick(float)",
    AMYCHARACTER_FORCEDUNCROUCH, "^AMyCharacter::ForcedUnCrouch()",
    FAPP_DELTATIME, "^FApp::DeltaTime",
    FMEMORY_MALLOC, "^FMemory::Malloc(unsigned long, unsigned int)",
    FMEMORY_FREE, "^FMemory::Free(void*)",
    FNAME_FNAME, "^FName::FName(wchar_t const*, EFindName)",
    FNAME_APPENDSTRING, "^FName::AppendString(FString&)",
    AMYHUD_DRAWHUD, "^AMyHUD::DrawHUD()",
    AHUD_DRAWLINE, "^AHUD::DrawLine(float, float, float, float, FLinearColor, float)",
    AHUD_DRAWTEXT, "^AHUD::DrawText(FString const&, FLinearColor, float, float, UFont*, float, bool)",
    AHUD_DRAWTEXTURESIMPLE, "^AHUD::DrawTextureSimple(UTexture*, float, float, float, bool)",
    AHUD_DRAWTEXTURE, "^AHUD::DrawTexture(UTexture*, float, float, float, float, float, float, float, float, FLinearColor, EBlendMode, float, bool, float, FVector2D)",
    AHUD_DRAWMATERIALSIMPLE, "^AHUD::DrawMaterialSimple(UMaterialInterface*, float, float, float, float, float, bool)",
    AHUD_DRAWRECT, "^AHUD::DrawRect(FLinearColor, float, float, float, float)",
    AHUD_PROJECT, "^AHUD::Project(FVector)",
    AHUD_GETTEXTSIZE, "^AHUD::GetTextSize(FString const&, float&, float&, UFont*, float)",
    GWORLD, "^GWorld",
    GUOBJECTARRAY, "^GUObjectArray",
    FUOBJECTARRAY_ALLOCATESERIALNUMBER, "^FUObjectArray::AllocateSerialNumber(int)",
    UWORLD_SPAWNACTOR, "^UWorld::SpawnActor(UClass*, FVector const*, FRotator const*, FActorSpawnParameters const&)",
    UWORLD_DESTROYACTOR, "^UWorld::DestroyActor(AActor*, bool, bool)",
    AMYCHARACTER_STATICCLASS, "^AMyCharacter::StaticClass()",
    APLAYERCONTROLLER_GETVIEWPORTSIZE, "^APlayerController::GetViewportSize(int&, int&)",
    APAWN_SPAWNDEFAULTCONTROLLER, "^APawn::SpawnDefaultController()",
    AACTOR_SETACTORENABLECOLLISION, "^AActor::SetActorEnableCollision(bool)",
    UOBJECT_PROCESSEVENT, "^UObject::ProcessEvent(UFunction*, void*)",
    UGAMEPLAYSTATICS_GETACCURATEREALTIME, "^UGameplayStatics::GetAccurateRealTime(UObject const*, int&, float&)",
    UTEXTURE2D_CREATETRANSIENT, "^UTexture2D::CreateTransient(int, int, EPixelFormat)",
    UTEXTURE2D_GETRUNNINGPLATFORMDATA, "^UTexture2D::GetRunningPlatformData()",
    UTEXTURE2D_UPDATERESOURCE, "^UTexture2D::UpdateResource()",
    FUNTYPEDBULKDATA_LOCK, "^FUntypedBulkData::Lock(unsigned int)",
    FUNTYPEDBULKDATA_UNLOCK, "^FUntypedBulkData::Unlock()",
    UGAMEUSERSETTINGS_APPLYRESOLUTIONSETTINGS, "^UGameUserSettings::ApplyResolutionSettings(bool)",
    UUSERWIDGET_ADDTOSCREEN, "^UUserWidget::AddToScreen(ULocalPlayer*, int)",
    UKISMETSYSTEMLIBRARY_LINETRACESINGLE, "^UKismetSystemLibrary::LineTraceSingle_NEW(UObject*, FVector, FVector, ETraceTypeQuery, bool, TArray<AActor*, FDefaultAllocator> const&, EDrawDebugTrace::Type, FHitResult&, bool, FLinearColor, FLinearColor, float)",
    FROTATOR_VECTOR, "^FRotator::Vector()",
    ALIFTBASE_ADDBASEDCHARACTER, "^ALiftBase::AddBasedCharacter(AMyCharacter*)",
    ALIFTBASE_REMOVEBASEDCHARACTER, "^ALiftBase::RemoveBasedCharacter(AMyCharacter*)",
    AMYCHARACTER_UNDERWATERCHANGED, "^AMyCharacter::UnderwaterChanged(bool)",
    UMATERIALINSTANCEDYNAMIC_SETSCALARPARAMETERVALUE, "^UMaterialInstanceDynamic::SetScalarParameterValue(FName, float)",
    UFONTBULKDATA_INITIALIZE, "^UFontBulkData::Initialize(void const*, int)",
    FVIEWPORT_SETGAMERENDERINGENABLED, "^FViewport::SetGameRenderingEnabled(bool, int)",
    UWIDGETBLUEPRINTLIBRARY_SETINPUTMODE_GAMEONLY, "^UWidgetBlueprintLibrary::SetInputMode_GameOnly(APlayerController*)",
    UWIDGETBLUEPRINTLIBRARY_SETINPUTMODE_UIONLYEX, "^UWidgetBlueprintLibrary::SetInputMode_UIOnlyEx(APlayerController*, UWidget*, EMouseLockMode)",
    APLAYERCONTROLLER_FLUSHPRESSEDKEYS, "^APlayerController::FlushPressedKeys()",
    APLAYERCONTROLLER_GETMOUSEPOSITION, "^APlayerController::GetMousePosition(float&, float&)",
}
