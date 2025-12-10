
use core::default;
use asr::settings::Gui;
use asr::settings::gui::Title;


#[derive(Gui)]
pub struct Settings {
    ///General
    _general: Title,

    #[default=false]
    pub automatically_start: bool,
    ///Split On Mirror Shard
    #[default=true]
    pub shard_split: bool,
    /// Dark Meta Knight (Final)
    #[default = false]
    pub dark_mk: bool,
    /// Dark Mind Phase 4 -> 5
    #[default = true]
    pub dm4_dm5: bool,
    /// Dark Mind 6 (Any% End)
    #[default = true]
    pub dm6_end: bool,

    #[default = false]
    ///Split On Warp back to Central Circle
    pub l_warp: bool,
    ///Warp Stars
    _warpstars: Title,

    /// Rainbow -> Moonlit Warp Star
    #[default = false]
    pub rr_mm_warp: bool,

    ///Collectibles
    _collectibles: Title,
    ///Split On Any Spray Collection
    #[default=false]
    pub spray_collect: bool,

    ///Switches
    _switches: Title,

    #[default=false]
    pub moonlight_mansion_switch: bool,
    #[default=false]
    pub rainbow_route_switch: bool,
    #[default=false]
    pub olive_ocean_switch: bool,
    #[default=false]
    pub deep_olive_ocean_switch: bool,
    #[default=false]
    pub deep_cabbage_cavern_switch: bool,
    #[default=false]
    pub cabbage_cavern_switch: bool,
    #[default=false]
    pub radish_ruins_switch: bool,
    #[default=false]
    pub deep_radish_ruins_switch: bool,
    #[default=false]
    pub carrot_castle_switch: bool,
    #[default=false]
    pub deep_carrot_castle_switch: bool,
    #[default=false]
    pub peppermint_palace_switch: bool,
    #[default=false]
    pub deep_peppermint_palace_switch: bool,
    #[default=false]
    pub mustard_mountain_switch: bool,
    #[default=false]
    pub candy_constellation_switch: bool,
    #[default=false]
    pub deep_mustard_mountain_switch: bool, 
    
}