use std::{
    fs::{read_to_string, write},
    path::Path,
};

use crate::structure::Level::{Lvl1, Lvl2, Lvl3};
use crate::structure::{TDExercice, TDList, TD};

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
    }; */

    td_setup();

    let td_list = get_td_list();
    match serde_json::to_string_pretty(&td_list) {
        Ok(json) => json,
        Err(e) => e.to_string(),
    }
}

fn td_setup() {
    /* new_td(
        2,
        "Dérivation, Intégration".to_string(),
        (12, 25, 32, 43, 61, 68),
    );
    new_td(4, "Relations binaires".to_string(), (6, 14, 16, 22, 35, 41));
    new_td(5, "Arithmétique".to_string(), (7, 14, 17, 26, 38, 43));
    new_td(6, "Les réels".to_string(), (6, 14, 15, 21, 26, 28));
    new_td(
        7,
        "Applications, lois internes".to_string(),
        (7, 16, 19, 24, 36, 39),
    );
    new_td(
        8,
        "Dénombrement, sommes finies".to_string(),
        (6, 17, 24, 29, 43, 53),
    );
    new_td(9, "Les complexes".to_string(), (10, 22, 24, 32, 44, 48));
    new_td(10, "Groupes".to_string(), (7, 15, 22, 29, 39, 46));
    new_td(11, "Anneaux".to_string(), (5, 14, 18, 23, 31, 36));
    new_td(
        12,
        "Espaces vectoriels".to_string(),
        (11, 23, 26, 31, 40, 43),
    );
    new_td(13, "Normes et suites".to_string(), (8, 15, 17, 20, 31, 35));
    new_td(15, "Topologie".to_string(), (3, 13, 18, 19, 31, 34));
    new_td(16, "Continuité".to_string(), (6, 17, 25, 28, 39, 51));
    new_td(
        17,
        "Calcul asymptotique".to_string(),
        (15, 27, 29, 43, 59, 65),
    ); */
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

fn new_td(id: u32, name: String, bounds: (u32, u32, u32, u32, u32, u32)) {
    let mut lvl1: Vec<TDExercice> = (1..bounds.0)
        .map(|i: u32| TDExercice {
            id: i,
            lvl: Lvl1,
            done: false,
        })
        .collect();
    let mut lvl1_opt: Vec<TDExercice> = (bounds.2..bounds.3)
        .map(|i: u32| TDExercice {
            id: i,
            lvl: Lvl1,
            done: false,
        })
        .collect();
    lvl1.append(&mut lvl1_opt);

    let mut lvl2: Vec<TDExercice> = (bounds.0..bounds.1)
        .map(|i: u32| TDExercice {
            id: i,
            lvl: Lvl2,
            done: false,
        })
        .collect();
    let mut lvl2_opt: Vec<TDExercice> = (bounds.3..bounds.4)
        .map(|i: u32| TDExercice {
            id: i,
            lvl: Lvl2,
            done: false,
        })
        .collect();
    lvl2.append(&mut lvl2_opt);

    let mut lvl3: Vec<TDExercice> = (bounds.1..bounds.2)
        .map(|i: u32| TDExercice {
            id: i,
            lvl: Lvl3,
            done: false,
        })
        .collect();
    let mut lvl3_opt: Vec<TDExercice> = (bounds.4..bounds.5)
        .map(|i: u32| TDExercice {
            id: i,
            lvl: Lvl3,
            done: false,
        })
        .collect();
    lvl3.append(&mut lvl3_opt);

    let td = TD {
        id,
        name,
        lvl1,
        lvl2,
        lvl3,
    };

    add_td(td);
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
