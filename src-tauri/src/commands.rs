use tauri_plugin_store::StoreExt;

#[tauri::command]
pub fn get_task_state(td: u32, exercice: u32) {
    println!("Hello world!");
}
