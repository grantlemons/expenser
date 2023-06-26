use leptos::*;
use leptos_router::*;

mod pages {
    mod dashboard;
    mod report;
    mod signin;
    mod user;

    pub use dashboard::*;
    pub use report::*;
    pub use signin::*;
    pub use user::*;
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
            <Route path="/signin" view=|cx| view! { cx, <SignInPage/>}/>
            <Route path="/dashboard" view=|cx| view! { cx, <DashboardPage/>}/>
            <Route path="/report/:id" view=|cx| view! { cx, <ReportPage/>}/>
            <Route path="/user/:id" view=|cx| view! { cx, <UserPage/>}/>
            /* ... */
          </Routes>
        </main>
      </Router>
    }
}
