use leptos::*;
use leptos_router::*;

#[component]
pub fn UserPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned());

    view! { cx,
        "user #"{id}
    }
}
