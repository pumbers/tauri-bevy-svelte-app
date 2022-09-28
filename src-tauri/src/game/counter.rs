use crate::game::TauriBridge;
use bevy::prelude::*;

#[derive(Default)]
pub struct CounterValue(u32);

// Bevy system to increment the counter resource
pub fn increment_counter(mut state: ResMut<CounterValue>) {
  state.0 = (state.0 + 1) % 1_000_000u32;
}

// Bevy system to send the counter resource value to the Tauri bridge (sender)
pub fn send_counter(tauri_bridge: ResMut<TauriBridge>, counter: Res<CounterValue>) {
  tauri_bridge
    .0
    .send(counter.0)
    .expect("Failed to send on channel");
}
