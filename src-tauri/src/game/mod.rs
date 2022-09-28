pub mod counter;

use crossbeam_channel::{Receiver, Sender};

pub struct TauriBridge(pub Sender<u32>);

pub struct BevyBridge(pub Receiver<u32>);

// Tauri command for the front end to retrieve the couinter value from the Bevy bridge (receiver)
#[tauri::command]
pub fn get_state(state: tauri::State<BevyBridge>) -> u32 {
  state.0.try_iter().last().unwrap_or(0)
}
