use crate::progress_bar::ProgressBar;
use crate::todo_element::CheckboxWithLabel;
use crate::todo_element::Collapse;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <CheckboxWithLabel label="test".to_string() />
            <CheckboxWithLabel label="test".to_string() />
            <CheckboxWithLabel label="test".to_string() />
            <Collapse title="TD 1".to_string()>
                <CheckboxWithLabel label="test".to_string() />
            </Collapse>
        </main>
        <ProgressBar percentage=0.618f32/>
    }
}
