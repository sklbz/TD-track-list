use leptos::prelude::*;

#[component]
pub fn CheckboxWithLabel(label: String) -> impl IntoView {
    view! {
        <div class="checkbox-container">
            <input type="checkbox"
                checked="checked"
                class="checkbox"/>
            <span class="checkbox-label">{label}</span>
        </div>
    }
}

#[component]
pub fn Collapse(title: String, children: Children) -> impl IntoView {
    view! {
        <div class="collapse collapse-arrow bg-base-100 border-base-300 border" style="max-width: 90vw">
            <input type="checkbox" />
              <div class="collapse-title font-semibold">{title}</div>
              <div class="collapse-content text-sm">
                {children()}

              </div>
        </div>
    }
}
