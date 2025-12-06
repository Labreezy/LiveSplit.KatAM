
#![no_std]
mod state;
mod settings;

use core::iter::Sum;

use asr::{Process, emulator::gba::Emulator, future::next_tick, print_limited, print_message, settings::Gui, timer::{self, TimerState}};
use state::{LocationPair, GameState};
use settings::Settings;

use crate::state::{CURR_ROOM_ADDR, DM6_HP_ADDR, SHARD_FLAG_ADDR, SPRAY_FLAG_ADDR, SWITCH_STATE_ARR_ADDR};
asr::async_main!(stable);
asr::panic_handler!();


pub struct CustomVars {
    shard_split_mask: [bool; 8],
    spray_split_mask: [bool; 14],
    switch_split_mask: [bool; 15],
    has_switch_split: [bool; 15],
    has_golem_split: bool,
    dm6_has_split: bool,
    dmk_final_loc: LocationPair,
    dm4_loc: LocationPair,
    rr_warpstar_loc: LocationPair,
    
    
}





async fn main() {
    asr::print_message("Starting KATAM Autosplitter");
    let mut settings = settings::Settings::register();
    loop {
        let process = Emulator::wait_attach().await;
        process
            .until_closes(async {
                let mut vars = startup();
                let mut state = GameState::default();
                let mut previous_timer_state = TimerState::NotRunning;
                loop {
                    settings.update();
                    update_loop(&process, &mut state, &settings);
                    let timer_state = timer::state();
                    
                    if timer_state == TimerState::Running {
                        if previous_timer_state == TimerState::NotRunning {
                            vars.has_golem_split = false;  
                            print_message("start");
                            start(&mut state, &mut vars, &settings);

                        }
                        if split(&mut state, &mut vars, &settings){
                            timer::split();
                        }
                    }
                    
                    previous_timer_state = timer_state;
                    next_tick().await;
                }
                }).await;
    }
}




pub fn start( state: &mut GameState, vars: &mut CustomVars, settings: &Settings) {
    vars.has_golem_split = false;
    vars.shard_split_mask = core::array::from_fn(|_| false);
    vars.spray_split_mask = core::array::from_fn(|_| false);
    vars.switch_split_mask = core::array::from_fn(|_| false);
    vars.has_switch_split = core::array::from_fn(|_| false);
    vars.dmk_final_loc.enabled = settings.dark_mk;
    vars.dm4_loc.enabled = settings.dm4_dm5;
    vars.rr_warpstar_loc.enabled = settings.rr_mm_warp;
    vars.dm4_loc.has_split = false;
    vars.dmk_final_loc.has_split = false;
    vars.rr_warpstar_loc.has_split = false;
    vars.dm6_has_split = false;

    macro_rules! set_switch_mask_if_setting {
        ( $setting:ident, $number:expr ) => {
            if settings.$setting {
                vars.switch_split_mask[$number] = true
            }
        };
    }

    set_switch_mask_if_setting!(moonlight_mansion_switch, 0);
    set_switch_mask_if_setting!(rainbow_route_switch, 1);
    set_switch_mask_if_setting!(olive_ocean_switch, 2);
    set_switch_mask_if_setting!(deep_olive_ocean_switch, 3);
    set_switch_mask_if_setting!(deep_cabbage_cavern_switch, 4);
    set_switch_mask_if_setting!(cabbage_cavern_switch, 5);
    set_switch_mask_if_setting!(radish_ruins_switch, 6);
    set_switch_mask_if_setting!(deep_radish_ruins_switch, 7);
    set_switch_mask_if_setting!(carrot_castle_switch, 8);
    set_switch_mask_if_setting!(deep_carrot_castle_switch, 9);
    set_switch_mask_if_setting!(peppermint_palace_switch, 10);
    set_switch_mask_if_setting!(deep_peppermint_palace_switch, 11);
    set_switch_mask_if_setting!(mustard_mountain_switch, 12);
    set_switch_mask_if_setting!(candy_constellation_switch, 13);
    set_switch_mask_if_setting!(deep_mustard_mountain_switch, 14);
}

pub fn startup() -> CustomVars {
    print_message("KatAM Autosplitter Loaded");
    CustomVars {
        shard_split_mask: core::array::from_fn(|_| false),
        spray_split_mask: core::array::from_fn(|_| false),
        switch_split_mask: core::array::from_fn(|_| false),
        has_switch_split: core::array::from_fn(|_| false),
        has_golem_split: false,
        dmk_final_loc: LocationPair { enabled: true, has_split: false, old_room: 909, new_room: 910 },
        dm4_loc: LocationPair {
            enabled: true,
            has_split: false,
            old_room: 916,
            new_room: 918
        },
        rr_warpstar_loc: LocationPair {
            enabled: true,
            has_split: false,
            old_room: 142,
            new_room: 195,
        },
        dm6_has_split: false,

    }
}

fn update_loop(game: &Emulator, state: &mut GameState, settings: &Settings){
    
    let curr_room = game.read::<u16>(CURR_ROOM_ADDR).unwrap_or_default();
    let curr_shards = game.read::<u8>(SHARD_FLAG_ADDR).unwrap_or_default();
    let dm6_hp = game.read::<u8>(DM6_HP_ADDR).unwrap_or_default();
    let spray_bytes = game.read::<[u8;2]>(SPRAY_FLAG_ADDR).unwrap_or_default();
    let switch_states = game.read::<[u32;15]>(SWITCH_STATE_ARR_ADDR).unwrap_or_default();

    state.room.update_infallible(curr_room);
    state.shards.update_infallible(curr_shards);
    state.dm6_hp.update_infallible(dm6_hp);
    state.sprays.update_infallible(spray_bytes);
    state.switches.update_infallible(switch_states);
}





fn split(state: &mut GameState, vars: &mut CustomVars, settings: &Settings) -> bool {
    let room_pair = state.room.pair.unwrap_or_default();
    let shard_pair = state.shards.pair.unwrap_or_default();
    let spray_pair = state.sprays.pair.unwrap_or_default();
    let switch_pair = state.switches.pair.unwrap_or_default();
    
    //Mirror Shard Collection
    if shard_pair.increased() {

        let num_old_flags: usize = vars.shard_split_mask.iter().filter(|&b| *b).count();
        let num_current_flags = (0..8).filter(|i: &i32| shard_pair.current & (1 << i) > 0).count();
        print_limited::<64>(&format_args!("{} -> {}", num_old_flags, num_current_flags));
        if num_current_flags - num_old_flags == 1 {
            vars.shard_split_mask = core::array::from_fn(|i| shard_pair.current & (1 << i) > 0);
            return true;
        }
        
    }

    macro_rules! split_loc_pair {
        ($name:ident) => {
            if (vars.$name.enabled && !vars.$name.has_split){
                if (room_pair.changed_from_to(&vars.$name.old_room, &vars.$name.new_room)) {
                    vars.$name.has_split = true;
                    return true;
                }
            }
        }
    }

    split_loc_pair!(dmk_final_loc);
    split_loc_pair!(dm4_loc);
    split_loc_pair!(rr_warpstar_loc);

    //Dark Mind 6 (Any% ending)
    if room_pair.current == 919 && settings.dm6_end && !vars.dm6_has_split {
        let dm6_pair: asr::watcher::Pair<u8> = state.dm6_hp.pair.unwrap_or_default();
        if dm6_pair.changed() {
            print_limited::<64>(&format_args!("DM6 HP {} -> {}", dm6_pair.old, dm6_pair.current));
            if dm6_pair.old > 1 && dm6_pair.current == 1 {
                vars.dm6_has_split = true;
                return true;
            }
        }
        
    }
    //Spray Pickup
    if settings.spray_collect && spray_pair.bytes_changed() {
        if spray_pair.current[0] >= spray_pair.old[0] && spray_pair.current[1] >= spray_pair.old[1] {
            let num_old_flags = vars.spray_split_mask.iter().filter(|&b| *b).count();
            let mut num_new_flags = (0..8).filter(|i| spray_pair.current[0] & (1 << i) > 0).count();
            num_new_flags += (0..6).filter(|i| spray_pair.current[1] & (1 << i) > 0).count();

            if num_new_flags - num_old_flags == 1 {
                for i in 0..8 {
                    vars.spray_split_mask[i] = (spray_pair[0] & 1 << i) > 0;
                }
                for i in 8..14 {
                    vars.spray_split_mask[i] = (spray_pair[1] & 1 << (i-8)) > 0;
                }
                print_message("COLLECTED SPRAY");
                return true;
            }
        }
        
    }

    //Switches
    let switches_on = vars.switch_split_mask.iter().filter(|&b| *b).count() > 0;
    if switches_on && switch_pair.bytes_changed() {
        let old_n_switches = vars.has_switch_split.iter().filter(|&b| *b).count();
        let new_n_switches = switch_pair.current.iter().filter(|&v| *v == 1).count();
        print_limited::<64>(&format_args!("{} -> {} switches", old_n_switches, new_n_switches));
        if new_n_switches > old_n_switches && new_n_switches  - old_n_switches == 1 {
            for i in 0..15 {
                
                if !vars.has_switch_split[i] && switch_pair.current[i] > 0 {
                    vars.has_switch_split[i] = true;
                    print_limited::<64>(&format_args!("SWITCH {} HIT ({})", i, switch_pair.current[i]));
                    if vars.switch_split_mask[i] {
                        return true;
                    }
                }
                
            }
        }
    }
    
    return false;
}