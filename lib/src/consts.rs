// FEngineLoop::Tick(FEngineLoop *__hidden this)
// _ZN11FEngineLoop4TickEv
#[cfg(windows)]
pub const FENGINELOOP_TICK_AFTER_UPDATETIME: usize =  0xe903c;

#[cfg(windows)]
pub const AMYCHARACTER_TICK: usize = 0xefbd0;

// FApp::DeltaTime
// _ZN4FApp9DeltaTimeE
// static variable inside the binary
#[cfg(windows)]
pub const APP_DELTATIME: usize = 0x1e20b08;

// FSlateApplication::Tick(FSlateApplication *__hidden this)
// _ZN17FSlateApplication4TickEv
#[cfg(windows)]
pub const FSLATEAPPLICATION_TICK: usize = 0x339170;

// AMyCharacter::ForcedUnCrouch(AMyCharacter *__hidden this)
// _ZN12AMyCharacter14ForcedUnCrouchEv
#[cfg(windows)]
// on Windows we hook 7 bytes before the return
pub const AMYCHARACTER_EXECFORCEDUNCROUCH_END: usize = 0x109e29;

// FSlateApplication::OnKeyDown(FSlateApplication *this, unsigned int, unsigned int, bool)
// _ZN17FSlateApplication9OnKeyDownEijb
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONKEYDOWN: usize = 0x329c50;

// FSlateApplication::OnKeyUp(FSlateApplication *this, unsigned int, unsigned int, bool)
// _ZN17FSlateApplication7OnKeyUpEijb
//#[cfg(unix)]
//pub const FSLATEAPPLICATION_ONKEYUP: [u8; 33] = [0x48, 0x8b, 0x07, 0xff, 0x50, 0x58, 0x66, 0x89,
//    0x44, 0x24, 0x08, 0xc7, 0x44, 0x24, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x44, 0x88, 0x64, 0x24, 0x10,
//    0x48, 0xc7, 0x44, 0x24, 0x18, 0x00, 0x00, 0x00, 0x00];
//#[cfg(unix)]
//pub const FSLATEAPPLICATION_ONKEYUP_OFFSET: usize = 76;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONKEYUP: usize = 0x329df0;

// FSlateApplication::OnRawMouseMove(FSlateApplication *this, int, int)
// _ZN17FSlateApplication14OnRawMouseMoveEii
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0x32ab10;

// AController::GetControlRotation(AController* this)
// _ZNK11AController18GetControlRotationEv
#[cfg(windows)]
pub const ACONTROLLER_GETCONTROLROTATION: usize = 0xba1e40;
