use leptos::*;
use leptos_router::*;

mod pages {
    mod signin;

    pub use signin::*;
}
use pages::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
          <Routes>
            <Route path="/signin" view=|cx| view! { cx, <SignIn/>}/>
            /* ... */
          </Routes>
        </main>
      </Router>
    }
}
