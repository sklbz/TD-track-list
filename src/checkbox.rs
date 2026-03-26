use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::td::TDList;

use super::invoke;

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

    let title = Resource::new(
        move || (td_id, exercice_id), // dependency
        move |(td_id, exercice_id)| async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "td_id": td_id,
                "exercice_id": exercice_id
            }))
            .expect("Failed to serialize arguments");

            let result = invoke("get_title", args).await;
            result.as_string().unwrap_or_default()
        },
    );

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
                    match title.get() {
                        Some(s) => format!("{}.{} {}", td_id, exercice_id, s),
                        None => format!("{}.{} ...", td_id, exercice_id), // loading
                    }
                }}
            </span>
        </div>
    }
}
