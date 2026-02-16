use crate::commands::get_task_state;
use tauri::{generate_context, generate_handler, Manager};
use tauri_plugin_opener::init;
use tauri_plugin_store::{Builder, StoreExt};

mod commands;
mod structure;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }
    builder
        .plugin(init())
        .plugin(Builder::default().build())
        .setup(|app| {
            // Create a new store or load the existing one
            // this also put the store in the app's resource table
            // so your following `store` calls (from both Rust and JS)
            // will reuse the same store.

            let store = app.store("store.json")?;

            Ok(())
        })
        .invoke_handler(generate_handler![get_task_state])
        .run(generate_context!())
        .expect("error while running tauri application");
}
