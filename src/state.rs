

use asr::{
    watcher::{Watcher},

};



#[derive(Default, Clone, Copy)]
pub struct LocationPair {
    pub enabled: bool,
    pub has_split: bool,
    pub old_room: u16,
    pub new_room: u16,
}

pub const CURR_ROOM_ADDR : u32 = 0x2020FE6;
pub const SHARD_FLAG_ADDR : u32 = 0x2038970;
pub const DM6_HP_ADDR : u32 = 0x2000088;
pub const SPRAY_FLAG_ADDR : u32 = 0x2038974;
pub const SWITCH_STATE_ARR_ADDR : u32 = 0x2028C14;

pub const MENU_FN_PTR : u32 = 0x30043F0;

pub const MENU_FN_START_VAL : u32 = 0x814238D;

pub static SPRAY_NAMES : [&str; 14] = [&"Pink",&"Yellow",&"Red",&"Green",&"Snow",&"Carbon",&"Ocean",&"Sapphire",&"Grape",&"Emerald",&"Orange",&"Chocolate",&"Cherry",&"Chalk"];

#[derive(Default)]
pub struct GameState {
    pub shards: Watcher<u8>,
    pub sprays: Watcher<[u8;2]>,
    pub room: Watcher<u16>,
    pub dm6_hp: Watcher<u8>,
    pub switches: Watcher<[u32; 15]>,
    pub menu_val: Watcher<u32>,
}