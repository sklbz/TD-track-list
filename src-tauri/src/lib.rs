use crate::commands::get_td_list_json;
use crate::commands::get_title;
use crate::commands::set_task_state;
use tauri::Builder;
use tauri::{generate_context, generate_handler, Manager};
use tauri_plugin_opener::init;

mod commands;
mod structure;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = Builder::default();
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }
    builder
        .plugin(init())
        .invoke_handler(generate_handler![
            get_td_list_json,
            set_task_state,
            get_title
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
