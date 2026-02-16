use leptos::prelude::*;

#[component]
pub fn ProgressBar(percentage: f32) -> impl IntoView {
    view! {
        <div class="progress-outer-bar">
            <div class="progress-inner-bar" style={format!("--proportion: {percentage}")}></div>
        </div>
    }
}
