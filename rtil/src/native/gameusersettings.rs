use crate::native::Args;

#[rtil_derive::hook_before(UGameUserSettings::ApplyResolutionSettings)]
fn apply_resolution_settings(_args: &mut Args) {
    crate::threads::ue::apply_resolution_settings();
}