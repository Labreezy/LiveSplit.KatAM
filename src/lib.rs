
#![no_std]
mod state;
mod settings;

use core::iter::Sum;

use asr::{Process, emulator::gba::Emulator, future::next_tick, print_limited, print_message, settings::Gui, timer::{self, TimerState}};
use state::{LocationPair, GameState};
use settings::Settings;
asr::async_main!(stable);
asr::panic_handler!();


pub struct CustomVars {
    shard_split_mask: [bool; 8],
    has_golem_split: bool,
    dmk_final_loc: LocationPair,
    dm4_loc: LocationPair,
    rr_warpstar_loc: LocationPair,
    
}


    pub fn mask_to_array(val: u8) -> [i32; 8] {
        let mut out_arr: [i32;8] = core::array::from_fn(|_| 0);
    for i in 0..8{
        let mask = 1 << i;
        if (mask & val) > 0{
            out_arr[i] = 1;
        }
    }
    out_arr
}




async fn main() {
    // TODO: Set up some general state and settings.
    //let mut settings = Settings::register();

    asr::print_message("Starting KATAM Autosplitter");
    let mut settings = settings::Settings::register();
    loop {
        let process = Emulator::wait_attach().await;
        process
            .until_closes(async {
                let mut vars = startup();
                let mut state = GameState::default();
                let mut previousTimerState = TimerState::NotRunning;
                loop {
                    update_loop(&process, &mut state, &settings);
                    let timer_state = timer::state();
                    
                    if timer_state == TimerState::Running {
                        if previousTimerState == TimerState::NotRunning {
                            vars.has_golem_split = false;   

                        }
                        if split(&mut state, &mut vars, &settings){
                            timer::split();
                        }
                    }
                    else if timer_state == TimerState::NotRunning {
                        if previousTimerState == TimerState::Running {
                            start( &mut state, &mut vars, &settings);
                        }
                        
                    }
                    previousTimerState = timer_state;
                    next_tick().await;
                }
                }).await;
    }
}


pub fn start( state: &mut GameState, vars: &mut CustomVars, settings: &Settings) {
    vars.has_golem_split = false;
    vars.shard_split_mask = core::array::from_fn(|_| false);

    vars.dmk_final_loc.enabled = settings.dark_mk;
    vars.dm4_loc.enabled = settings.dm4_dm5;
    vars.rr_warpstar_loc.enabled = settings.rr_mm_warp;
    vars.dm4_loc.has_split = false;
    vars.dmk_final_loc.has_split = false;
    vars.rr_warpstar_loc.has_split = false;

}

pub fn startup() -> CustomVars {
    print_message("KatAM Autosplitter Loaded");
    CustomVars {
        shard_split_mask: core::array::from_fn(|_| false),
        has_golem_split: false,
        dmk_final_loc: LocationPair { enabled: true, has_split: false, old_room: 909, new_room: 910 },
        dm4_loc: LocationPair{
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
        }
    }
}

fn update_loop(game: &Emulator, state: &mut GameState, settings: &Settings){
    let curr_room = game.read::<u16>(0x2020FE6).unwrap_or_default();
    let curr_shards = game.read::<u8>(0x2038970).unwrap_or_default();
    let dm6_hp = game.read::<u8>(0x2000088).unwrap_or_default();
    state.room.update_infallible(curr_room);
    state.shards.update_infallible(curr_shards);
    state.dm6_hp.update_infallible(dm6_hp);
}





fn split(state: &mut GameState, vars: &mut CustomVars, settings: &Settings) -> bool {
    let room_pair = state.room.pair.unwrap_or_default();
    let shard_pair = state.shards.pair.unwrap_or_default();


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

    if room_pair.current == 919 && settings.dm6_end {
        let dm6_pair = state.dm6_hp.pair.unwrap_or_default();
        if dm6_pair.changed() {
            print_limited::<64>(&format_args!("DM6 HP {} -> {}", dm6_pair.old, dm6_pair.current));
        }
        if dm6_pair.old >= 1 && dm6_pair.current == 1 {
            return true;
        }
    }

    if shard_pair.increased() {
        let current_mask = mask_to_array(shard_pair.current);
        let old_mask = mask_to_array(shard_pair.old);
        let old_bits_set : i32 = old_mask.iter().sum();
        let bits_set : i32 = current_mask.iter().sum();
        print_limited::<64>(&format_args!("Shards: {} -> {}", old_bits_set, bits_set));
        if shard_pair.old == 0 {
            if vars.has_golem_split {
                return false;
            }
            if bits_set == 1 {
                return true;
            }
            return false;
        } else {
            if bits_set - old_bits_set != 1 {
                return false;
            }
            if bits_set == 1 {
                vars.has_golem_split = true;
            }
            return true;
        }
    }

    return false;
}