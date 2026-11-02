// inline: pselUiSetAllowUserCreation
#[inline(always)]
pub fn pselUiSetAllowUserCreation(ui: &mut PselUiSettings, flag: bool) {
    if ui.settings.mode == PselUiMode_UserSelector {
        ui.settings.is_permitted = flag as u8;
    }
}
// inline: pselUiSetNetworkServiceRequired
#[inline(always)]
pub fn pselUiSetNetworkServiceRequired(ui: &mut PselUiSettings, flag: bool) {
    ui.settings.is_network_service_account_required = flag as u8;
}
// inline: pselUiSetSkipButtonEnabled
#[inline(always)]
pub fn pselUiSetSkipButtonEnabled(ui: &mut PselUiSettings, flag: bool) {
    ui.settings.show_skip_button = flag as u8;
}
