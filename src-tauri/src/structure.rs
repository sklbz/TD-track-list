use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Level {
    Lvl1,
    Lvl2,
    Lvl3,
}

#[allow(unused)]
impl Level {
    pub fn to_u8(&self) -> u8 {
        match self {
            Level::Lvl1 => 1,
            Level::Lvl2 => 2,
            Level::Lvl3 => 3,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TDExercice {
    pub id: u32,
    pub lvl: Level,
}

#[derive(Serialize, Deserialize)]
pub struct TD {
    pub id: u32,
    pub name: String,
    pub exercices: Vec<TDExercice>,
}

#[derive(Serialize, Deserialize)]
pub struct TDList {
    pub tds: Vec<TD>,
}
