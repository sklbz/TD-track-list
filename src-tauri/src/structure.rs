use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Level {
    Lvl1,
    Lvl2,
    Lvl3,
}

#[derive(Serialize, Deserialize)]
pub struct TDExercice {
    pub id: u32,
    pub lvl: Level,
    pub done: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TD {
    pub id: u32,
    pub name: String,
    pub lvl1: Vec<TDExercice>,
    pub lvl2: Vec<TDExercice>,
    pub lvl3: Vec<TDExercice>,
}

#[derive(Serialize, Deserialize)]
pub struct TDList {
    pub tds: Vec<TD>,
}

impl TDList {
    pub fn set_task_state(&mut self, td_id: u32, exercice_id: u32, state: bool) {
        for td in self.tds.iter_mut() {
            if td.id == td_id {
                td.set_task_state(exercice_id, state);
            }
        }
    }
}

impl TD {
    pub fn set_task_state(&mut self, exercice_id: u32, state: bool) {
        for exercice in self.lvl1.iter_mut() {
            if exercice.id == exercice_id {
                exercice.done = state;
            }
        }
        for exercice in self.lvl2.iter_mut() {
            if exercice.id == exercice_id {
                exercice.done = state;
            }
        }
        for exercice in self.lvl3.iter_mut() {
            if exercice.id == exercice_id {
                exercice.done = state;
            }
        }
    }
}
