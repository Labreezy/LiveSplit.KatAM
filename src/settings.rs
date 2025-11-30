
use asr::settings::Gui;
use asr::settings::gui::Title;


#[derive(Gui)]
pub struct Settings {
    /// Dark Meta Knight (Final)
    #[default = false]
    pub dark_mk: bool,
    /// Dark Mind Phase 4 -> 5
    #[default = true]
    pub dm4_dm5: bool,
    /// Dark Mind 6 (Any% End)
    #[default = true]
    pub dm6_end: bool,

    ///Warp Stars
    _warpstars: Title,

    /// Rainbow -> Moonlit Warp Star
    #[default = false]
    pub rr_mm_warp: bool
    
}