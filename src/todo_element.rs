use leptos::prelude::*;

#[component]
pub fn Collapse(
    title: String,
    label: String,
    label_color: String,
    children: Children,
    class: String,
) -> impl IntoView {
    view! {
        <div class=format!("{} collapse collapse-arrow bg-base-100 border-base-300 border p-2 m-3", class) style="max-width: 90vw">
            <span class="collapse-label" style=format!("color: var(--{});", label_color)>{label}</span>
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
        <div class="collapse sub-collapse collapse-arrow bg-base-100" style="max-width: 85vw">
            <input type="checkbox" />
              <div class="collapse-title font-semibold">{title}</div>
              <div class="collapse-content p-2 flex flex-col content-start justify-start">
                {children()}
              </div>
        </div>
    }
}
