#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::plugin::hr::HrPlugin;
use tauri::Runtime;

mod plugin;

#[tauri::command]
async fn toggle_decorations<R: Runtime>(window: tauri::Window<R>) -> Result<(), ()> {
  let was_decorated = window.is_decorated().unwrap();

  window.set_decorations(!was_decorated).unwrap();
  window.set_always_on_top(was_decorated).unwrap();

  Ok(())
}

fn main() {
  env_logger::init();

  tauri::Builder::default()
      .plugin(HrPlugin::new())
      .invoke_handler(tauri::generate_handler![toggle_decorations])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
