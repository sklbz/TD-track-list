use leptos::prelude::*;

#[component]
pub fn CheckboxWithLabel<F>(label: String, on_change: F, checked: bool) -> impl IntoView
where
    F: Fn(bool) + 'static,
{
    view! {
        <div class="checkbox-container">
            <input type="checkbox"
                checked=checked
                on:change=move |ev| {
                    let checked = event_target_checked(&ev);
                    on_change(checked);
                }
                class="checkbox"/>
            <span class="checkbox-label text-xs">{label}</span>
        </div>
    }
}

#[component]
pub fn Collapse(title: String, label: String, children: Children) -> impl IntoView {
    view! {
        <div class="collapse collapse-arrow bg-base-100 border-base-300 border p-2 m-3" style="max-width: 90vw">
            <span class="collapse-label">{label}</span>
            <input type="checkbox" />
              <div class="collapse-title font-semibold">{title}</div>
              <div class="collapse-content text-sm">
                {children()}
              </div>
        </div>
    }
}

#[component]
pub fn SubCollapse(title: String, children: Children) -> impl IntoView {
    view! {
        <div class="collapse collapse-arrow bg-base-100" style="max-width: 85vw">
            <input type="checkbox" />
              <div class="collapse-title font-semibold">{title}</div>
              <div class="collapse-content p-2 flex flex-col content-start justify-start">
                {children()}
              </div>
        </div>
    }
}
