use crate::game::{BevyBridge, TauriBridge};
use bevy::prelude::*;

#[derive(Default)]
pub struct CounterValue(u32);

pub fn increment_counter(mut state: ResMut<CounterValue>) {
  state.0 = (state.0 + 1) % 1_000_000u32;
}

pub fn send_counter(tauri_bridge: ResMut<TauriBridge>, mut counter: ResMut<CounterValue>) {
  tauri_bridge
    .0
    .send(counter.0)
    .expect("Failed to send on channel");

  match tauri_bridge.1.try_recv() {
    Ok(_) => counter.0 = 0,
    _ => {}
  }
}

#[tauri::command]
pub fn reset_counter(state: tauri::State<BevyBridge>) {
  state
    .0
    .send(())
    .expect("Unable to send reset message to bevy");
}
