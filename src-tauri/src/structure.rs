use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TD {
    pub id: u32,
    pub name: String,
    pub lvl1: u32,
    pub lvl2: u32,
    pub lvl3: u32,
    pub lvl1o: u32,
    pub lvl2o: u32,
    pub lvl3o: u32,
}

#[derive(Serialize, Deserialize)]
pub struct TDList {
    pub tds: Vec<TD>,
}
