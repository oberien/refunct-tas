// FEngineLoop::Tick(FEngineLoop *__hidden this)
// _ZN11FEngineLoop4TickEv
#[cfg(unix)]
pub const FENGINELOOP_TICK_AFTER_UPDATETIME: usize = 0x16f2208;
#[cfg(windows)]
pub const FENGINELOOP_TICK_AFTER_UPDATETIME: usize =  0xe8e67;

#[cfg(unix)]
pub const GMALLOC: usize = 0x5a17ea0;

// FApp::DeltaTime
// static variable inside the binary
#[cfg(unix)]
pub const APP_DELTATIME: usize = 0x4d7a9c0;
#[cfg(windows)]
pub const APP_DELTATIME: usize = 0x1e1eb08;
// FSlateApplication::Tick(FSlateApplication *__hidden this)
// _ZN17FSlateApplication4TickEv
#[cfg(unix)]
pub const FSLATEAPPLICATION_TICK: usize = 0x1ac7e20;
#[cfg(windows)]
pub const FSLATEAPPLICATION_TICK: usize = 0x338d60;
// AMyCharacter::ForcedUnCrouch(AMyCharacter *__hidden this)
// _ZN12AMyCharacter14ForcedUnCrouchEv
#[cfg(unix)]
pub const AMYCHARACTER_EXECFORCEDUNCROUCH: usize = 0x1705090;
#[cfg(windows)]
// on Windows we hook before the return
pub const AMYCHARACTER_EXECFORCEDUNCROUCH: usize = 0x1092f0;
// FSlateApplication::OnKeyDown(FSlateApplication *this, unsigned int, unsigned int, bool)
// _ZN17FSlateApplication9OnKeyDownEijb
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONKEYDOWN: usize = 0x1ad7540;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONKEYDOWN: usize = 0x329680;
// FSlateApplication::OnKeyUp(FSlateApplication *this, unsigned int, unsigned int, bool)
// _ZN17FSlateApplication7OnKeyUpEijb
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONKEYUP: usize = 0x1ad8670;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONKEYUP: usize = 0x329820;
// FSlateApplication::OnRawMouseMove(FSlateApplication *this, int, int)
// _ZN17FSlateApplication14OnRawMouseMoveEii
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0x1ae0700;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0x32a540;

// ACharacter::CheckJumpInput(ACharacter* this, float)
// _ZN10ACharacter14CheckJumpInputEf

// AController::GetControlRotation(AController* this)
// _ZNK11AController18GetControlRotationEv
#[cfg(unix)]
pub const ACONTROLLER_GETCONTROLROTATION: usize = 0x2903fa0;
#[cfg(windows)]
pub const ACONTROLLER_GETCONTROLROTATION: usize = 0xba1120;
