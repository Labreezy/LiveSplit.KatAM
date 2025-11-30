

use asr::{
    watcher::{Pair, Watcher},
    Address,
};

macro_rules! define_address {
    ($name:ident, $addr:expr) => {
        pub const $name: Address = Address::new($addr);
    };
}



#[derive(Default, Clone, Copy)]
pub struct LocationPair {
    pub enabled: bool,
    pub has_split: bool,
    pub old_room: u16,
    pub new_room: u16,
}

define_address!(CURR_ROOM_ADDR,0x2020FE6);
define_address!(SHARD_FLAG_ADDR,0x2038970);
define_address!(CHEST_START_ADDR,0x2038960);

#[derive(Default)]
pub struct GameState {
    pub shards: Watcher<u8>,
    pub chests: Watcher<[u8; 11]>,
    pub room: Watcher<u16>,
    pub dm6_hp: Watcher<u8>
}