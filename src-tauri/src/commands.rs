use std::{
    fs::{read_to_string, write},
    path::Path,
};

use crate::structure::{TDList, TD};

/* #[tauri::command]
pub fn get_task_state(_td: u32, _exercice: u32) {
    println!("Hello world!");
} */

#[tauri::command]
pub fn get_td_list_json() -> String {
    /* let td = TD {
        id: 16,
        name: "Continuité".to_string(),
        lvl1: vec![TDExercice {
            id: 1,
            lvl: Lvl1,
            done: false,
        }],
        lvl2: vec![TDExercice {
            id: 2,
            lvl: Lvl2,
            done: false,
        }],
        lvl3: vec![TDExercice {
            id: 3,
            lvl: Lvl3,
            done: false,
        }],
    };

    add_td(td); */

    let td_list = get_td_list();
    match serde_json::to_string_pretty(&td_list) {
        Ok(json) => json,
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
pub fn set_task_state(td: u32, exercice: u32, state: bool) {
    println!("Hello task!");
    let mut td_list: TDList = get_td_list();
    td_list.set_task_state(td, exercice, state);
    rewrite_td_list(td_list);
}

fn rewrite_td_list(td_list: TDList) {
    println!("rewrite");
    match serde_json::to_string_pretty(&td_list) {
        Ok(json) => save_td_list_json(json),
        Err(e) => println!("{}", e),
    };
}

fn save_td_list_json(json: String) {
    println!("save");
    let path = Path::new("/home/sklbz/.config/td-track/td.json");
    match write(path, json) {
        Ok(_) => println!("Successfully wrote to file"),
        Err(e) => println!("Failed to write to file: {}", e),
    };
}

fn add_td(td: TD) {
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
