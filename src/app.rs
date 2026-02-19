use crate::progress_bar::ProgressBar;
use crate::td::TDList;
use crate::td::TdList;
// use crate::todo_element::CheckboxWithLabel;
use crate::todo_element::Collapse;
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
    let (td_list, set_td_list) = signal(TDList { tds: vec![] });
    // let (debug, set_debug) = signal(String::new());

    spawn_local({
        async move {
            let result = invoke("get_td_list_json", JsValue::NULL).await;
            let s = result.as_string().unwrap();
            let list = serde_json::from_str(&s).unwrap();
            // set_debug.set(s);
            set_td_list.set(list);
        }
    });

    view! {
        <main>
            // <CheckboxWithLabel label="test".to_string() />
            // <CheckboxWithLabel label="test".to_string() />
            // <CheckboxWithLabel label="test".to_string() />
            /* { move || view!{
                <Collapse title=debug.get()>
                    <CheckboxWithLabel label="".to_string() />
                <Collapse/>
            } } */
            <Collapse title="test".to_string() label="test".to_string() >
        {"yay"}
            </Collapse>
            { move || view!{ <TdList list=td_list.get()/>} }
        </main>
        {move || view!{ <ProgressBar percentage=td_list.get().proportion() />}}
    }
}
