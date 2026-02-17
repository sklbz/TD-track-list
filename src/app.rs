use crate::progress_bar::ProgressBar;
use crate::todo_element::CheckboxWithLabel;
use crate::todo_element::Collapse;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (text, set_text) = signal(String::new());

    spawn_local({
        let set_text = set_text.clone();
        async move {
            let result = invoke("test_command", wasm_bindgen::JsValue::NULL).await;
            let s = result
                .as_string()
                .unwrap_or_else(|| "Command failed".to_string());

            set_text.set(s);
        }
    });

    view! {
        <main>
            <CheckboxWithLabel label="test".to_string() />
            <CheckboxWithLabel label="test".to_string() />
            <CheckboxWithLabel label="test".to_string() />
            <Collapse title="TD 1".to_string()>
                <CheckboxWithLabel label="test".to_string() />
                {move || text.get()}
            </Collapse>
        </main>
        <ProgressBar percentage=0.618f32/>
    }
}
