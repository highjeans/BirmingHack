use leptos::prelude::*;
use leptos::IntoView;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="h-full w-full flex flex-col items-center justify-around py-20 min-h-120">
            <span class="text-2xl">{"Login"}</span>
            <div class="flex flex-col gap-4 text-center items-center">
            <label class="flex flex-col gap-2 w-[20vw]">
                {"Email"}
                <input type="text" class="border rounded-md px-4 py-2"

                />
            </label>
            <label class="flex flex-col gap-2 w-[20vw]">
                {"Password"}
                <input type="password" class="border rounded-md px-4 py-2"

                />
            </label>
            <button class="rounded-md bg-green-800 w-[10vw] h-10 cursor-pointer hover:bg-green-900 active:bg-green-950">{"Submit"}</button>
            </div>
        </div>
    }
}
