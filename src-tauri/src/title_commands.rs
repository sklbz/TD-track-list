use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::fs::{read_to_string, write};
use std::path::Path;
use std::sync::RwLock;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct TitleMap {
    #[serde_as(as = "Vec<(_, _)>")]
    title_map: HashMap<(u32, u32), String>,
}

const PATH: &str = "/home/sklbz/.config/td-track/titles.json";

/// 🔥 Global cached map (thread-safe)
static TITLE_MAP: Lazy<RwLock<HashMap<(u32, u32), String>>> = Lazy::new(|| {
    let path = Path::new(PATH);
    let raw_data = read_to_string(path).unwrap_or_default();

    // Désérialise comme Vec de tuples, comme serde_with le sauvegarde
    let pairs: Vec<((u32, u32), String)> = serde_json::from_str(&raw_data).unwrap_or_default();
    let map: HashMap<(u32, u32), String> = pairs.into_iter().collect();

    RwLock::new(map)
});

#[tauri::command]
pub fn get_title(td_id: u32, exercice_id: u32) -> String {
    use std::io::Write;
    println!("get_title called");
    std::io::stdout().flush().ok();

    let map = TITLE_MAP.read().unwrap();

    match map.get(&(td_id, exercice_id)) {
        Some(title) => {
            println!("Title: {}", title);
            title.clone()
        }
        None => {
            println!("Title not found");
            "".to_string()
        }
    }
}

#[tauri::command]
pub fn get_all_titles() -> String {
    let path = Path::new(PATH);
    let raw_data = read_to_string(path).unwrap_or_default();
    println!("raw file content: {}", raw_data);

    let map = TITLE_MAP.read().unwrap();
    println!("map size: {}", map.len());
    for (k, v) in map.iter() {
        println!("  {:?} => {}", k, v);
    }

    let pairs: Vec<((u32, u32), String)> = map.iter().map(|(k, v)| (*k, v.clone())).collect();
    let result = serde_json::to_string(&pairs).unwrap_or_default();
    println!("returning: {}", result);
    result
}

pub fn save_title_json(json: String) {
    println!("save title");

    let path = Path::new(PATH);

    match write(path, &json) {
        Ok(_) => println!("Successfully wrote titles to file"),
        Err(e) => println!("Failed to write to file: {}", e),
    };

    // 🔥 IMPORTANT: update the in-memory cache
    let new_map: HashMap<(u32, u32), String> = serde_json::from_str(&json).unwrap_or_default();

    let mut map = TITLE_MAP.write().unwrap();
    *map = new_map;
}
