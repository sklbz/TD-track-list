use leptos::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::todo_element::Collapse;

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
            <table>
                <tr>
                    <th>lvl1</th>
                    <th>lvl2</th>
                    <th>lvl3</th>
                    <th>lvl1o</th>
                    <th>lvl2o</th>
                    <th>lvl3o</th>

                </tr>
                <tr>
                    <td>{td.lvl1}</td>
                    <td>{td.lvl2}</td>
                    <td>{td.lvl3}</td>
                    <td>{td.lvl1o}</td>
                    <td>{td.lvl2o}</td>
                    <td>{td.lvl3o}</td>
                </tr>
            </table>
        </Collapse>
    }
}
