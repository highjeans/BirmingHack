use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! { <div class="flex items-center justify-center flex-grow">
        <h1 class="text-4xl font-black">404</h1>
    </div> }
}
