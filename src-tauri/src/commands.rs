use std::{
    fs::{read_to_string, write},
    path::Path,
};

use crate::structure::{Level::*, TDExercice, TDList, TD};

/* #[tauri::command]
pub fn get_task_state(_td: u32, _exercice: u32) {
    println!("Hello world!");
} */

#[tauri::command]
pub fn get_td_list_json() -> String {
    let td = TD {
        id: 16,
        name: "Continuité".to_string(),
        lvl1: vec![TDExercice { id: 1, lvl: Lvl1 }],
        lvl2: vec![TDExercice { id: 2, lvl: Lvl2 }],
        lvl3: vec![TDExercice { id: 3, lvl: Lvl3 }],
    };

    let td_list = get_td_list();
    match serde_json::to_string_pretty(&td_list) {
        Ok(json) => json,
        Err(e) => e.to_string(),
    }
}

fn save_td(td: TD) {
    let mut td_list: TDList = get_td_list();
    td_list.tds.push(td);
    let json = match serde_json::to_string_pretty(&td_list) {
        Ok(json) => json,
        Err(e) => e.to_string(),
    };

    let path = Path::new("/home/sklbz/.config/td-track/td.json");
    match write(path, json) {
        Ok(_) => println!("Successfully wrote to file"),
        Err(e) => println!("Failed to write to file: {}", e),
    };
}

fn get_td_list() -> TDList {
    let path = Path::new("/home/sklbz/.config/td-track/td.json");
    let raw_data = match read_to_string(path) {
        Ok(data) => data,
        Err(e) => e.to_string(),
    };
    let td_list: TDList = match serde_json::from_str(&raw_data) {
        Ok(data) => data,
        Err(_) => TDList { tds: Vec::new() },
    };

    td_list
}
