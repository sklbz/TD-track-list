use crate::progress_bar::ProgressBar;
use crate::td::TDList;
use crate::td::TdList;
use crate::todo_element::CheckboxWithLabel;
use crate::todo_element::Collapse;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (td_list, set_td_list) = signal(TDList { tds: vec![] });

    spawn_local({
        let set_td_list = set_td_list.clone();
        async move {
            let result = invoke("test_command", wasm_bindgen::JsValue::NULL).await;
            let list = serde_json::from_str(&result.as_string().unwrap()).unwrap();
            set_td_list.set(list);
        }
    });

    view! {
        <main>
            <CheckboxWithLabel label="test".to_string() />
            <CheckboxWithLabel label="test".to_string() />
            <CheckboxWithLabel label="test".to_string() />
            <Collapse title="TD 1".to_string()>
                <CheckboxWithLabel label="test".to_string() />
                { move || view!{ <TdList list=td_list.get()/>} }
            </Collapse>
        </main>
        <ProgressBar percentage=0.618f32/>
    }
}
