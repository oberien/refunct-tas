// FEngineLoop::Tick(FEngineLoop *__hidden this)
// _ZN11FEngineLoop4TickEv
//#[cfg(unix)]
//pub const FENGINELOOP_TICK_AFTER_UPDATETIME: usize = 0x16f2208;
#[cfg(windows)]
pub const FENGINELOOP_TICK_AFTER_UPDATETIME: usize =  0xe8e6c;

// UEngine::UpdateTimeAndHandleMaxTickRate()
// _ZN7UEngine30UpdateTimeAndHandleMaxTickRateEv
#[cfg(unix)]
pub const UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE: [u8; 20] = [0xbf, 0x50, 0x73, 0x20, 0x04, 0xbe,
    0xdc, 0x04, 0x00, 0x00, 0xba, 0x10, 0x19, 0x20, 0x04, 0xb0, 0x02, 0x41, 0x89, 0xe8];
#[cfg(unix)]
pub const UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE_OFFSET: usize = 374;

//#[cfg(unix)]
//pub const GENGINE: usize = 0x505fd40;

// FApp::DeltaTime
// static variable inside the binary
#[cfg(unix)]
pub const APP_DELTATIME: usize = 0x4d7a9c0;
#[cfg(windows)]
pub const APP_DELTATIME: usize = 0x1e1eb08;

// FSlateApplication::Tick(FSlateApplication *__hidden this)
// _ZN17FSlateApplication4TickEv
#[cfg(unix)]
pub const FSLATEAPPLICATION_TICK: [u8; 35] = [0xf2, 0x0f, 0x10, 0x83, 0x70, 0x04, 0x00, 0x00, 0xf2,
    0x0f, 0x5c, 0x83, 0x78, 0x04, 0x00, 0x00, 0xf2, 0x0f, 0x5a, 0xc0, 0x83, 0xbb, 0x78, 0x01, 0x00,
    0x00, 0x00, 0xf3, 0x0f, 0x11, 0x44, 0x24, 0x04, 0x7e, 0x70];
#[cfg(unix)]
pub const FSLATEAPPLICATION_TICK_OFFSET: usize = 22;
#[cfg(windows)]
pub const FSLATEAPPLICATION_TICK: usize = 0x338d60;

// AMyCharacter::ForcedUnCrouch(AMyCharacter *__hidden this)
// _ZN12AMyCharacter14ForcedUnCrouchEv
#[cfg(unix)]
pub const AMYCHARACTER_EXECFORCEDUNCROUCH: [u8; 21] = [0x48, 0x8b, 0xbf, 0x68, 0x08, 0x00, 0x00, 0x48,
    0x8b, 0x07, 0x48, 0x8b, 0x80, 0xf0, 0x06, 0x00, 0x00, 0x31, 0xf6, 0xff, 0xe0];
#[cfg(unix)]
pub const AMYCHARACTER_EXECFORCEDUNCROUCH_OFFSET: usize = 0;
#[cfg(windows)]
// on Windows we hook 7 bytes before the return
pub const AMYCHARACTER_EXECFORCEDUNCROUCH_END: usize = 0x109309;

// FSlateApplication::OnKeyDown(FSlateApplication *this, unsigned int, unsigned int, bool)
// _ZN17FSlateApplication9OnKeyDownEijb
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONKEYDOWN: [u8; 18] = [0x66, 0x89, 0x44, 0x24, 0x08, 0xc7, 0x44, 0x24,
    0x0c, 0x00, 0x00, 0x00, 0x00, 0x44, 0x88, 0x64, 0x24, 0x10];
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONKEYDOWN_OFFSET: usize = 82;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONKEYDOWN: usize = 0x329680;

// FSlateApplication::OnKeyUp(FSlateApplication *this, unsigned int, unsigned int, bool)
// _ZN17FSlateApplication7OnKeyUpEijb
//#[cfg(unix)]
//pub const FSLATEAPPLICATION_ONKEYUP: [u8; 33] = [0x48, 0x8b, 0x07, 0xff, 0x50, 0x58, 0x66, 0x89,
//    0x44, 0x24, 0x08, 0xc7, 0x44, 0x24, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x44, 0x88, 0x64, 0x24, 0x10,
//    0x48, 0xc7, 0x44, 0x24, 0x18, 0x00, 0x00, 0x00, 0x00];
//#[cfg(unix)]
//pub const FSLATEAPPLICATION_ONKEYUP_OFFSET: usize = 76;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONKEYUP: usize = 0x329820;

// FSlateApplication::OnRawMouseMove(FSlateApplication *this, int, int)
// _ZN17FSlateApplication14OnRawMouseMoveEii
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONRAWMOUSEMOVE: [u8; 33] = [0x48, 0x8b, 0x07, 0xff, 0x50, 0x58, 0x66,
    0x89, 0x44, 0x24, 0x10, 0xc7, 0x44, 0x24, 0x14, 0x00, 0x00, 0x00, 0x00, 0xc6, 0x44, 0x24, 0x18,
    0x00, 0x48, 0xc7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00];
pub const FSLATEAPPLICATION_ONRAWMOUSEMOVE_OFFSET: usize = 110;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0x32a540;

// ACharacter::CheckJumpInput(ACharacter* this, float)
// _ZN10ACharacter14CheckJumpInputEf

// AController::GetControlRotation(AController* this)
// _ZNK11AController18GetControlRotationEv
#[cfg(unix)]
pub const ACONTROLLER_GETCONTROLROTATION: [u8; 17] = [0xf3, 0x0f, 0x7e, 0x87, 0xb8, 0x03, 0x00,
    0x00, 0xf3, 0x0f, 0x10, 0x8f, 0xc0, 0x03, 0x00, 0x00, 0xc3];
#[cfg(unix)]
pub const ACONTROLLER_GETCONTROLROTATION_OFFSET: usize = 0;
#[cfg(windows)]
pub const ACONTROLLER_GETCONTROLROTATION: usize = 0xba1120;
