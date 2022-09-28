#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod game;

use crate::game::counter::{increment_counter, send_counter, CounterValue};
use crate::game::{get_state, BevyBridge, TauriBridge};
use bevy::{app::ScheduleRunnerSettings, prelude::*, utils::Duration};
use crossbeam_channel::bounded;
use std::thread;
use tauri::api::shell;
use tauri::{
  AboutMetadata, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, WindowBuilder,
  WindowUrl,
};

fn main() {
  /*
   * Start Bevy in a separate thread with a mpsc bridge between Bevy & Tauri
   */
  let (tx, rx) = bounded::<u32>(1000);
  thread::spawn(move || {
    App::new()
      .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0,
      )))
      .add_plugins(MinimalPlugins)
      .insert_resource(TauriBridge(tx))
      .insert_resource(CounterValue::default())
      .add_system(increment_counter)
      .add_system(send_counter)
      .run()
  });

  /*
   * Start Tauri as usual
   */
  let ctx = tauri::generate_context!();
  tauri::Builder::default()
    .manage(BevyBridge(rx))
    .invoke_handler(tauri::generate_handler![get_state])
    .setup(|app| {
      let _window = WindowBuilder::new(app, "main", WindowUrl::default())
        .title("Tauri Svelte App")
        .inner_size(800.0, 550.0)
        .min_inner_size(400.0, 200.0)
        .build()
        .expect("Unable to create window");
      Ok(())
    })
    .menu(Menu::with_items([
      #[cfg(target_os = "macos")]
      MenuEntry::Submenu(Submenu::new(
        &ctx.package_info().name,
        Menu::with_items([
          MenuItem::About(ctx.package_info().name.clone(), AboutMetadata::default()).into(),
          MenuItem::Separator.into(),
          CustomMenuItem::new("settings".to_string(), "Settings").into(),
          MenuItem::Separator.into(),
          MenuItem::Services.into(),
          MenuItem::Separator.into(),
          MenuItem::Hide.into(),
          MenuItem::HideOthers.into(),
          MenuItem::ShowAll.into(),
          MenuItem::Separator.into(),
          MenuItem::Quit.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "File",
        Menu::with_items([MenuItem::CloseWindow.into()]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Edit",
        Menu::with_items([
          MenuItem::Undo.into(),
          MenuItem::Redo.into(),
          MenuItem::Separator.into(),
          MenuItem::Cut.into(),
          MenuItem::Copy.into(),
          MenuItem::Paste.into(),
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator.into(),
          MenuItem::SelectAll.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "View",
        Menu::with_items([MenuItem::EnterFullScreen.into()]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Window",
        Menu::with_items([MenuItem::Minimize.into(), MenuItem::Zoom.into()]),
      )),
      // You should always have a Help menu on macOS because it will automatically
      // show a menu search field
      MenuEntry::Submenu(Submenu::new(
        "Help",
        Menu::with_items([CustomMenuItem::new("learn_more", "Learn More").into()]),
      )),
    ]))
    .on_menu_event(|event| match event.menu_item_id() {
      "settings" => {
        let handle = event.window().app_handle();
        let position = event.window().outer_position().unwrap();
        tauri::WindowBuilder::new(
          &handle,
          "settings", /* the unique window label */
          tauri::WindowUrl::App("windows/settings/settings.html".into()),
        )
        .title("Settings")
        .inner_size(640.0, 480.0)
        .resizable(false)
        .position(position.x as f64 + 20.00, position.y as f64 + 20.00)
        .build()
        .expect("failed to build settings window");
      }
      "learn_more" => {
        let url = "https://github.com/probablykasper/tauri-template".to_string();
        shell::open(&event.window().shell_scope(), url, None).unwrap();
      }
      _ => {}
    })
    .run(ctx)
    .expect("error while running tauri application");
}
