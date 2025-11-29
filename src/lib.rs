
#![no_std]
mod state;

use core::iter::Sum;

use asr::{emulator::gba::Emulator, future::next_tick, timer::{self, TimerState}, print_message};
use state::{LocationPair, GameState};

asr::async_main!(stable);
asr::panic_handler!();


pub struct CustomVars {
    shard_split_mask: [bool; 8],
    has_golem_split: bool,
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
   

    loop {
        let process = Emulator::wait_attach().await;
        process
            .until_closes(async {
                let mut vars = startup();
                let mut state = GameState::default();
                let mut previousTimerState = TimerState::NotRunning;
                loop {
                    update_loop(&process, &mut state);
                    let timer_state = timer::state();
                    
                    if timer_state == TimerState::Running {
                        if previousTimerState == TimerState::NotRunning {
                            

                        }
                        if split(&mut state, &mut vars){
                            timer::split();
                        }
                    }
                    else if timer_state == TimerState::NotRunning {
                        if previousTimerState == TimerState::Running {
                            vars.shard_split_mask = core::array::from_fn(|_| false);
                            vars.has_golem_split = false;
                        }
                        
                    }
                    previousTimerState = timer_state;
                    next_tick().await;
                }
                }).await;
    }
}


pub fn startup() -> CustomVars {
    print_message("KatAM Autosplitter Loaded");
    CustomVars {
        shard_split_mask: core::array::from_fn(|_| false),
        has_golem_split: false,
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

fn update_loop(game: &Emulator, state: &mut GameState){
    let curr_room = game.read::<u16>(0x2020FE6).unwrap_or_default();
    let curr_shards = game.read::<u8>(0x2038970).unwrap_or_default();
    state.room.update_infallible(curr_room);
    state.shards.update_infallible(curr_shards);
}



fn split(state: &mut GameState, vars: &mut CustomVars) -> bool {
    let room_pair = state.room.pair.unwrap_or_default();
    let shard_pair = state.shards.pair.unwrap_or_default();
    if room_pair.changed_from_to(&vars.dm4_loc.old_room, &vars.dm4_loc.new_room){
        return true;
    }
    if room_pair.changed_from_to(&vars.rr_warpstar_loc.old_room, &vars.rr_warpstar_loc.new_room){
        return true;
    }
    if shard_pair.increased() {
        let current_mask = mask_to_array(shard_pair.current);
        let old_mask = mask_to_array(shard_pair.old);
        let old_bits_set : i32 = old_mask.iter().sum();
        let bits_set : i32 = current_mask.iter().sum();
        if shard_pair.old == 0 {
            if vars.has_golem_split {
                return false;
            }
            if bits_set > 1 {
                return false;
            }
            return true;
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