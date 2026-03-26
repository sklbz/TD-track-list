use leptos::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::checkbox::CheckboxWithLabel;
use crate::todo_element::Collapse;
use crate::todo_element::SubCollapse;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Level {
    Lvl1,
    Lvl2,
    Lvl3,
}

impl Level {
    pub fn as_u8(&self) -> u8 {
        match self {
            Level::Lvl1 => 1,
            Level::Lvl2 => 2,
            Level::Lvl3 => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TDExercice {
    pub id: u32,
    pub lvl: Level,
    pub done: bool,
}

impl TDExercice {
    pub fn get_score(&self) -> u8 {
        if self.done {
            return self.lvl.as_u8();
        }

        0u8
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TD {
    pub id: u32,
    pub name: String,
    pub lvl1: Vec<TDExercice>,
    pub lvl2: Vec<TDExercice>,
    pub lvl3: Vec<TDExercice>,
}

impl TD {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_score(&self) -> u32 {
        let result_1: u32 = self
            .lvl1
            .iter()
            .fold(0u32, |acc, value| acc + value.get_score() as u32);

        let result_2: u32 = self
            .lvl2
            .iter()
            .fold(result_1, |acc, value| acc + value.get_score() as u32);

        let result_3: u32 = self
            .lvl3
            .iter()
            .fold(result_2, |acc, value| acc + value.get_score() as u32);

        result_3
    }

    pub fn max_score(&self) -> u32 {
        let raw_max = self.lvl1.len() + self.lvl2.len() * 2 + self.lvl3.len() * 3;

        raw_max as u32
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDList {
    pub tds: Vec<TD>,
}

impl TDList {
    pub fn get_score(&self) -> u32 {
        self.tds
            .iter()
            .fold(0u32, |acc, value| acc + value.get_score())
    }

    pub fn max_score(&self) -> u32 {
        self.tds
            .iter()
            .fold(0u32, |acc, value| acc + value.max_score())
    }

    pub fn proportion(&self) -> f32 {
        self.get_score() as f32 / self.max_score() as f32
    }

    pub fn sort(&mut self) {
        self.tds.sort_by(|a, b| a.id.cmp(&b.id));
    }

    pub fn set_task_state(&self, td_id: u32, exercice_id: u32, state: bool) -> TDList {
        let mut new_self = self.clone();
        for td in new_self.tds.iter_mut() {
            if td.id == td_id {
                td.set_task_state(exercice_id, state);
            }
        }

        new_self
    }
}

#[component]
pub fn TdList(list: TDList) -> impl IntoView {
    view! {
        <section class="td-list flex flex-col p-4">
            {list.tds.iter().map(|td| view! { <TdDisplay td={td.clone()}/> }).collect::<Vec<_>>()}
            // {list.tds.iter().map(|td| view! { <TdDisplayDebug td={td.clone()}/> }).collect::<Vec<_>>()}
        </section>
    }
}

/* #[component]
pub fn TdDisplayDebug(td: TD) -> impl IntoView {
    view! {
        <Collapse title={format!("TD {0}: {1}", td.id, td.name)}>
        "yay"
        </Collapse>
    }
} */

#[component]
pub fn TdDisplay(td: TD) -> impl IntoView {
    let id = td.id;
    let name = td.get_name();
    let score = td.get_score();
    let max_score = td.max_score();

    let lvl1_data: Vec<(u32, bool)> = td.lvl1.iter().map(|e| (e.id, e.done)).collect();
    let lvl2_data: Vec<(u32, bool)> = td.lvl2.iter().map(|e| (e.id, e.done)).collect();
    let lvl3_data: Vec<(u32, bool)> = td.lvl3.iter().map(|e| (e.id, e.done)).collect();

    let percentage = score as f32 / max_score as f32;

    let color = if percentage < 0.17 {
        "catppuccin-red".to_string()
    } else if percentage < 0.33 {
        "catppuccin-maroon".to_string()
    } else if percentage < 0.5 {
        "catppuccin-peach".to_string()
    } else if percentage < 0.8 {
        "catppuccin-yellow".to_string()
    } else {
        "catppuccin-green".to_string()
    };

    view! {
        <Collapse
            title={format!("TD {0}: {1}", id, name)}
            label=format!("{0}/{1}", score, max_score)
            label_color=color
            class="td-collapse".to_string()>

            <SubCollapse title="Niveau 1".to_string()>
                {lvl1_data.iter().map(|(e_id, done)| {
                    // let id = id;
                    let e_id = *e_id;
                    view! {
                        <CheckboxWithLabel
                            td_id=id
                            exercice_id=e_id
                            checked=*done
                        />
                    }}).collect::<Vec<_>>()
            }
            </SubCollapse>

            <SubCollapse title="Niveau 2".to_string()>
                {lvl2_data.iter().map(|(e_id, done)| {
                    // let id = id;
                    let e_id = *e_id;
                    view! {
                        <CheckboxWithLabel
                            td_id=id
                            exercice_id=e_id
                            checked=*done
                        />
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

            <SubCollapse title="Niveau 3".to_string()>
                {lvl3_data.iter().map(|(e_id, done)| {
                    // let id = id;
                    let e_id = *e_id;
                    view! {
                        <CheckboxWithLabel
                            td_id=id
                            exercice_id=e_id
                            checked=*done
                        />
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

        </Collapse>
    }
}
