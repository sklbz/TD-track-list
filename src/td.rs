use leptos::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::todo_element::CheckboxWithLabel;
use crate::todo_element::Collapse;
use crate::todo_element::SubCollapse;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDList {
    pub tds: Vec<TD>,
}

#[component]
pub fn TdList(list: TDList) -> impl IntoView {
    view! {
        <section class="td-list flex flex-col p-4">
            {list.tds.iter().map(|td| view! { <TdDisplay td={td.clone()}/> }).collect::<Vec<_>>()}
        </section>
    }
}

#[component]
pub fn TdDisplay(td: TD) -> impl IntoView {
    view! {
        <Collapse title={td.name}>

            <SubCollapse title="Niveau 1".to_string()>
                {(0..td.lvl1).map(|n| {
                    view! {
                        <CheckboxWithLabel label={format!("{0}.{n}", td.id)}/>
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

            <SubCollapse title="Niveau 2".to_string()>
                {(0..td.lvl2).map(|n| {
                    view! {
                        <CheckboxWithLabel label={format!("{0}.{1}", td.id, n + td.lvl1)}/>
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

            <SubCollapse title="Niveau 3".to_string()>
                {(0..td.lvl3).map(|n| {
                    view! {
                        <CheckboxWithLabel label={format!("{0}.{1}", td.id, n + td.lvl2 + td.lvl1)}/>
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

            <SubCollapse title="Niveau 1".to_string()>
                {(0..td.lvl1o).map(|n| {
                    view! {
                        <CheckboxWithLabel label={format!("{0}.{1}", td.id, n + td.lvl3 + td.lvl2 + td.lvl1)}/>
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

            <SubCollapse title="Niveau 2".to_string()>
                {(0..td.lvl2o).map(|n| {
                    view! {
                        <CheckboxWithLabel label={format!("{0}.{1}", td.id, n + td.lvl1o + td.lvl3 + td.lvl2 + td.lvl1)}/>
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

            <SubCollapse title="Niveau 3".to_string()>
                {(0..td.lvl3o).map(|n| {
                    view! {
                        <CheckboxWithLabel label={format!("{0}.{1}", td.id, n + td.lvl2o + td.lvl1o + td.lvl3 + td.lvl2 + td.lvl1)}/>
                    }}).collect::<Vec<_>>()
                }
            </SubCollapse>

        </Collapse>
    }
}
