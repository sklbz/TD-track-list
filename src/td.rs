use leptos::leptos_dom::logging;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;
use serde::Serialize;

use crate::todo_element::CheckboxWithLabel;
use crate::todo_element::Collapse;
use crate::todo_element::SubCollapse;

use super::app::invoke;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Level {
    Lvl1,
    Lvl2,
    Lvl3,
}

impl Level {
    pub fn to_u8(&self) -> u8 {
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
            return self.lvl.to_u8();
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

    view! {
        <Collapse title={format!("TD {0}: {1}", id, name)} label=format!("{0}/{1}", score, max_score)>

            <SubCollapse title="Niveau 1".to_string()>
                {lvl1_data.iter().map(|(e_id, done)| {
                    let id = id;
                    let e_id = *e_id;
                    view! {
                        <CheckboxWithLabel
                            label={format!("{0}.{1}", id, e_id)}
                            checked=*done
                            on_change=move |done: bool| {
                                let id = id;
                                let e_id = e_id;
                                spawn_local(async move {
                                    log!("set_task_state");
                                    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                                        "td": id,
                                        "exercice": e_id,
                                        "state": done
                                    }))
                                    .expect("Failed to serialize arguments");
                                    invoke("set_task_state", args).await;
                                })
                            }
                        />
                    }}).collect::<Vec<_>>()
            }
            </SubCollapse>

            <SubCollapse title="Niveau 2".to_string()>
                {lvl2_data.iter().map(|(e_id, done)| {
                    let id = id;
                    let e_id = *e_id;
                    view! {
                        <CheckboxWithLabel
                            label={format!("{0}.{1}", id, e_id)}
                            checked=*done
                            on_change=move |done: bool| {
                                let id = id;
                                let e_id = e_id;
                                spawn_local(async move {
                                    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                                        "td": id,
                                        "exercice": e_id,
                                        "state": done
                                    }))
                                    .expect("Failed to serialize arguments");
                                    invoke("set_task_state", args).await;
                                })
                            }
                        />
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

            <SubCollapse title="Niveau 3".to_string()>
                {lvl3_data.iter().map(|(e_id, done)| {
                    let id = id;
                    let e_id = *e_id;
                    view! {
                        <CheckboxWithLabel
                            label={format!("{0}.{1}", id, e_id)}
                            checked=*done
                            on_change=move |done: bool| {
                                let id = id;
                                let e_id = e_id;
                                spawn_local(async move {
                                    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                                        "td": id,
                                        "exercice": e_id,
                                        "state": done
                                    }))
                                    .expect("Failed to serialize arguments");
                                    invoke("set_task_state", args).await;
                                })
                            }
                        />
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

        </Collapse>
    }
}
