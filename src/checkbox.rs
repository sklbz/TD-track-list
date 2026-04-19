use std::collections::HashMap;

use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::td::TDList;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn CheckboxWithLabel(td_id: u32, exercice_id: u32, checked: bool) -> impl IntoView {
    let td_list = use_context::<RwSignal<TDList>>().unwrap();

    let on_change = move |done: bool| {
        let id = td_id;
        let e_id = exercice_id;
        spawn_local(async move {
            td_list.set(td_list.get().set_task_state(id, e_id, done));
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "td": id,
                "exercice": e_id,
                "state": done
            }))
            .expect("Failed to serialize arguments");
            invoke("set_task_state", args).await;
        })
    };

    let titles = use_context::<RwSignal<HashMap<(u32, u32), String>>>().unwrap();

    view! {
        <div class="checkbox-container">
            <input type="checkbox"
                checked=checked
                on:change=move |ev| {
                    let checked = event_target_checked(&ev);
                    on_change(checked);
                }
                class="checkbox"/>
            <span class="checkbox-label text-xs">
            {move || {
                let map = titles.get();
                match map.get(&(td_id, exercice_id)) {
                    Some(s) if !s.is_empty() => format!("{}.{} {}", td_id, exercice_id, s),
                    _ => format!("{}.{}", td_id, exercice_id),
                    }
            }}
            </span>
        </div>
    }
}
