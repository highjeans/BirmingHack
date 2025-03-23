use base64::encode;
use icondata as i;
use leptos::logging::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_icons::Icon;
use reqwest::Client;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{ArrayBuffer, Uint8Array};
use web_sys::{File, HtmlInputElement};

struct Specifics {
    isbn: String,
}

async fn upload_picture(input: Option<web_sys::HtmlInputElement>) {
    let file: File = input.unwrap().files().unwrap().get(0).unwrap();
    let array_buffer = JsFuture::from(file.array_buffer()).await.unwrap();
    let uint8_array = Uint8Array::new(&array_buffer.dyn_into::<ArrayBuffer>().unwrap());

    let mut body = vec![0; uint8_array.length() as usize];
    uint8_array.copy_to(&mut body);

    let base64_data = encode(&body);
    let json_body = serde_json::json!({
        "image": format!("data:image/jpeg;base64,{}", base64_data)
    });

    let res = Client::new()
        .post("https://webhook.site/69aa4849-56f7-429a-a7c4-bc4d3b4e7344")
        .json(&json_body)
        .send()
        .await;

    log!("{:?}", res);
}

#[component]
pub fn NewListing() -> impl IntoView {
    let (state, set_state) = signal(0);
    let file_ref: NodeRef<leptos::html::Input> = NodeRef::new();

    let (specific, set_specific) = signal(Specifics {
        isbn: "".to_string(),
    });

    view! {
        <div class="flex flex-col items-center justify-center flex-grow">
            <div class="flex flex-col items-center">
                <h1 class="text-lg font-bold leading-none">"Let's list your book for exchange!"</h1>
                <h2 class="text-lg text-brown-200 leading-none">"Start by taking a picture of the back of your book"</h2>
            </div>
            <div class="my-4">
                <Icon icon={i::BsArrowDown} {..} style="color: var(--color-brown-200)"/>
            </div>
            <div>
                <input
                    on:change=move |_| {
                        let file = file_ref.get();
                        spawn_local(async move { upload_picture(file).await; })
                    }
                    node_ref=file_ref type="file" class="hidden"
                />
                <button
                    on:click=move |_| {
                        if let Some(input) = file_ref.get() {
                            input.click();
                        }
                    }
                    class="flex flex-col border border-brown-700 border-dashed rounded-sm px-16 py-12 items-center cursor-pointer hover:bg-[#00000011] transition-colors"
                >
                    <span class="text-brown-300">"Upload a picture"</span>
                    <span class="text-brown-400">"Take a clear picture of the "<strong>"back"</strong>" of your book."</span>
                </button>
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
