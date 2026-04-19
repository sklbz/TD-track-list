use std::collections::HashMap;

use crate::progress_bar::ProgressBar;
use crate::td::TDList;
use crate::td::TdList;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub(super) async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let td_list = RwSignal::new(TDList { tds: vec![] });
    let titles = RwSignal::new(HashMap::<(u32, u32), String>::new());

    provide_context(td_list);
    provide_context(titles);

    spawn_local({
        async move {
            let result = invoke("get_td_list_json", JsValue::NULL).await;
            let s = result.as_string().unwrap();
            let mut list: TDList = serde_json::from_str(&s).unwrap();
            list.sort();
            td_list.set(list);
        }
    });

    spawn_local(async move {
        web_sys::console::log_1(&"calling get_all_titles".into());
        let result = invoke("get_all_titles", JsValue::NULL).await;
        let s = result.as_string().unwrap_or_default();
        web_sys::console::log_1(&format!("raw: {}", s).into());

        let pairs: Vec<((u32, u32), String)> = serde_json::from_str(&s).unwrap_or_default();
        web_sys::console::log_1(&format!("pairs len: {}", pairs.len()).into());

        let map: HashMap<(u32, u32), String> = pairs.into_iter().collect();
        titles.set(map);
    });

    view! {
        <main>
            { move || view!{ <TdList list=td_list.get()/>} }
        </main>
        {move || view!{ <ProgressBar percentage=td_list.get().proportion() />}}
    }
}
