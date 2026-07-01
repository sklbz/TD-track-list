use leptos::prelude::*;

#[component]
pub fn ProgressBar(percentage: f32) -> impl IntoView {
    let proportion = percentage * 100f32;
    let atom = 100f32 / 60f32;
    let stage = proportion.div_euclid(atom);
    let progress = proportion.rem_euclid(atom);
    let atomic_progress = progress / atom;
    view! {
        <footer>
            <div class="progress-outer-bar">
                <div class="progress-inner-bar" style={format!("--proportion: {}%", atomic_progress * 100f32)}></div>
            </div>
            <span class="stage_count">{format!("{}/60", stage)}</span>
        </footer>
    }
}
