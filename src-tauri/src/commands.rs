use crate::structure::TD;

#[tauri::command]
pub fn get_task_state(_td: u32, _exercice: u32) {
    println!("Hello world!");
}

#[tauri::command]
pub fn test_command() -> String {
    let td = TD {
        id: 0,
        name: "test".to_string(),
        lvl1: 1,
        lvl2: 1,
        lvl3: 1,
        lvl1o: 1,
        lvl2o: 1,
        lvl3o: 1,
    };

    let json = serde_json::to_string_pretty(&td).expect("Failed to serialize");

    json
}

fn save_td(td: TD) {
    let mut td_list
}
