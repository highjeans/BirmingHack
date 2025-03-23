use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::IntoView;
use reqwest::Client;
use shared::SignupData;
use shared::SignupResponse;

#[component]
pub fn Signup() -> impl IntoView {
    let fullname = RwSignal::new("".to_string());
    let username = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());
    let error = RwSignal::new("".to_string());
    let navigate = leptos_router::hooks::use_navigate();

    let submitForm = async move |username: String, password: String, fullname: String| {
        let client = Client::builder().build().unwrap();
        let res = client.post("http://localhost:8000/users/signup").json(&SignupData {
            fullname,
            username,
            password,
        }).send().await.unwrap();
        if res.status() == 204 {
            navigate("/login", Default::default());
            return;
        }
        error.set(res.json::<SignupResponse>().await.unwrap().message);
    };

    view! {
        <div class="h-full w-full flex flex-col items-center justify-around py-20 min-h-120">
            <span class="text-2xl">{"Signup"}</span>
            <div class="flex flex-col gap-4 text-center items-center">
            <label class="flex flex-col gap-2 w-[20vw]">
                {"Full Name"}
                <input type="text" class="border rounded-md px-4 py-2"
                    bind:value=fullname
                />
            </label>
            <label class="flex flex-col gap-2 w-[20vw]">
                {"Username"}
                <input type="text" class="border rounded-md px-4 py-2"
                    bind:value=username
                />
            </label>
            <label class="flex flex-col gap-2 w-[20vw]">
                {"Password"}
                <input type="password" class="border rounded-md px-4 py-2"
                    bind:value=password
                />
            </label>
            <button class="rounded-md bg-green-800 w-[10vw] h-10 cursor-pointer hover:bg-green-900 active:bg-green-950"
                on:click = move |_| {
                    let (username_str, password_str, fullname_str) = (username.get().clone(), password.get().clone(), fullname.get().clone());
                    let submit = submitForm.clone();
                    spawn_local(async move { submit(username_str, password_str, fullname_str).await; } );
                }
            >{"Submit"}</button>
            <Show when=move || { !error.get().is_empty() }>
                <span>{format!("Error: {}", error.get())}</span>
            </Show>
            </div>
        </div>
    }
}
