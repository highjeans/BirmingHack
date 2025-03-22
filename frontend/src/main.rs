use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router_macro::path;

mod routes;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="flex max-w-3xl mx-auto">
                <div class="flex flex-col w-full min-h-dvh mx-4">
                    <nav class="flex flex-row items-center justify-between my-4">
                        <a href="/" class="font-bold text-xl w-fit hover:underline">bookexch</a>
                        <div class="flex flex-row gap-6 items-center">
                            <a href="/login" class="block rounded-sm text-brown-200 hover:text-brown-100 transition-colors cursor-pointer text-sm hover:underline">
                                Login
                            </a>
                            <a href="/register" class="block px-4 py-1 bg-brown-200 rounded-sm text-stone-900 hover:bg-brown-100 transition-colors cursor-pointer text-sm">
                                Register
                            </a>
                        </div>
                    </nav>
                    <main class="flex flex-col flex-grow">
                        <Routes fallback=|| routes::not_found::NotFound>
                            <Route path=path!("/") view=routes::index::Index/>
                            <Route path=path!("/login") view=|| view! { <h1>Not yet implemented.</h1> }/>
                            <Route path=path!("/register") view=|| view! { <h1>Not yet implemented.</h1> }/>
                            <Route path=path!("/profile") view=|| view! { <h1>Not yet implemented.</h1> }/>
                            <Route path=path!("/profile/:id") view=|| view! { <h1>Not yet implemented.</h1> }/>
                            <Route path=path!("/listing") view=routes::listing::NewListing/>
                            <Route path=path!("/listing/:id") view=routes::listing::Listing/>
                        </Routes>
                    </main>
                    <footer class="my-4 text-center text-brown-600">
                        Created in 24 hours by Vivan, James and Freddy
                    </footer>
                </div>
            </div>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
