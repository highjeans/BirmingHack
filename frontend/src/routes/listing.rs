use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn NewListing() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center flex-grow">
            <h1 class="text-lg font-bold">"Let's list your book for sale!"</h1>
            <h2 class="text-lg text-brown-200">"Start by taking a picture of the back of your book"</h2>
            <div class="mt-2">
                <Icon icon={i::BsArrowDown} {..} style="color: var(--color-brown-200)"/>
            </div>
    </div>

    }
}

#[component]
pub fn Listing() -> impl IntoView {
    view! { <div class="flex items-center justify-center flex-grow">
        <h1 class="text-4xl font-black">404</h1>
    </div> }
}
