use leptos::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::todo_element::CheckboxWithLabel;
use crate::todo_element::Collapse;
use crate::todo_element::SubCollapse;

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
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDList {
    pub tds: Vec<TD>,
}

#[component]
pub fn TdList(list: TDList) -> impl IntoView {
    view! {
        <section class="td-list flex flex-col p-4">
            {list.tds.iter().map(|td| view! { <TdDisplay td={td.clone()}/> }).collect::<Vec<_>>()}
            {list.tds.iter().map(|td| view! { <TdDisplayDebug td={td.clone()}/> }).collect::<Vec<_>>()}
        </section>
    }
}

#[component]
pub fn TdDisplayDebug(td: TD) -> impl IntoView {
    view! {
        <Collapse title={format!("TD {0}: {1}", td.id, td.name)}>
        "yay"
        </Collapse>
    }
}

#[component]
pub fn TdDisplay(td: TD) -> impl IntoView {
    view! {
        <Collapse title={format!("TD {0}: {1}", td.get_id(), td.get_name())}>

            <SubCollapse title="Niveau 1".to_string()>
                {td.lvl1.iter().map(|e| {
                    view! {
                        <CheckboxWithLabel label={format!("{0}.{1}", td.id, e.id)}/>
                    }}).collect::<Vec<_>>()
            }
            </SubCollapse>

            <SubCollapse title="Niveau 2".to_string()>
                {td.lvl2.iter().map(|e| {
                    view! {
                        <CheckboxWithLabel label={format!("{0}.{1}", td.id, e.id)}/>
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

            <SubCollapse title="Niveau 3".to_string()>
                {td.lvl3.iter().map(|e| {
                    view! {
                        <CheckboxWithLabel label={format!("{0}.{1}", td.id, e.id)}/>
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

        </Collapse>
    }
}
