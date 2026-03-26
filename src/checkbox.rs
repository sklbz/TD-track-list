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

    view! {
        <div class="checkbox-container">
            <input type="checkbox"
                checked=checked
                on:change=move |ev| {
                    let checked = event_target_checked(&ev);
                    on_change(checked);
                }
                class="checkbox"/>
            <span class="checkbox-label text-xs">{format!("{0}.{1}", td_id, exercice_id)}</span>
        </div>
    }
}
