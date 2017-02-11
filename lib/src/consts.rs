// FEngineLoop::Tick(FEngineLoop *__hidden this)
// _ZN11FEngineLoop4TickEv
#[cfg(unix)]
pub const FENGINELOOP_TICK_AFTER_UPDATETIME: usize = 0x1905D98;
#[cfg(windows)]
pub const FENGINELOOP_TICK_AFTER_UPDATETIME: usize = 0xe8bcc;

#[cfg(unix)]
pub const GMALLOC: usize = 0x58d8e20;

// FApp::DeltaTime
// static variable inside the binary
#[cfg(unix)]
pub const APP_DELTATIME: usize = 0x57F0F40;
#[cfg(windows)]
pub const APP_DELTATIME: usize = 0x1e39a68;
// FSlateApplication::Tick(FSlateApplication *__hidden this)
// _ZN17FSlateApplication4TickEv
#[cfg(unix)]
pub const FSLATEAPPLICATION_TICK: usize = 0x1cd1390;
#[cfg(windows)]
pub const FSLATEAPPLICATION_TICK: usize = 0x330be0;
// AMyCharacter::ForcedUnCrouch(AMyCharacter *__hidden this)
// _ZN12AMyCharacter14ForcedUnCrouchEv
#[cfg(unix)]
pub const AMYCHARACTER_EXECFORCEDUNCROUCH: usize = 0x192b040;
#[cfg(windows)]
// on Windows we hook before the return
pub const AMYCHARACTER_EXECFORCEDUNCROUCH: usize = 0x1007d9;
// FSlateApplication::OnKeyDown(FSlateApplication *this, unsigned int, unsigned int, bool)
// _ZN17FSlateApplication9OnKeyDownEijb
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONKEYDOWN: usize = 0x1ce0ab0;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONKEYDOWN: usize = 0x321750;
// FSlateApplication::OnKeyUp(FSlateApplication *this, unsigned int, unsigned int, bool)
// _ZN17FSlateApplication7OnKeyUpEijb
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONKEYUP: usize = 0x1ce1be0;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONKEYUP: usize = 0x3218f0;
// FSlateApplication::OnRawMouseMove(FSlateApplication *this, int, int)
// _ZN17FSlateApplication14OnRawMouseMoveEii
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0x1ce9c70;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0x322610;
