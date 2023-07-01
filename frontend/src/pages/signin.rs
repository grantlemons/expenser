use leptos::*;

#[component]
pub fn SignInPage(cx: Scope) -> impl IntoView {
    let (signed_in, set_signed_in) = create_signal(cx, false);

    view! { cx,
        "signin"
    }
}
