use leptos::prelude::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router_macro::path;

mod routes;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav></nav>
            <main>
                <Routes fallback=|| routes::not_found::NotFound>
                    <Route path=path!("/") view=routes::index::Index/>
                    <Route path=path!("/login") view=|| view! { <h1>Not yet implemented.</h1> }/>
                    <Route path=path!("/register") view=|| view! { <h1>Not yet implemented.</h1> }/>
                    <Route path=path!("/profile") view=|| view! { <h1>Not yet implemented.</h1> }/>
                    <Route path=path!("/profile/:id") view=|| view! { <h1>Not yet implemented.</h1> }/>
                    <Route path=path!("/listing") view=|| view! { <h1>Not yet implemented.</h1> }/>
                    <Route path=path!("/listing/:id") view=|| view! { <h1>Not yet implemented.</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
