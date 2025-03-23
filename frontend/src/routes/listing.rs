use crate::AuthState;
use base64::encode;
use icondata as i;
use leptos::logging::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_icons::Icon;
use leptos_router::hooks::{use_navigate, use_params_map};
use reactive_stores::Store;
use reqwest::Client;
use shared::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{ArrayBuffer, Uint8Array};
use web_sys::{File, HtmlInputElement};

struct Specifics {
    isbn: String,
}

async fn upload_picture(jwt: String, input: Option<web_sys::HtmlInputElement>) -> String {
    let file: File = input.unwrap().files().unwrap().get(0).unwrap();

    let array_buffer = JsFuture::from(file.array_buffer()).await.unwrap();
    let uint8_array = Uint8Array::new(&array_buffer.dyn_into::<ArrayBuffer>().unwrap());

    let mut body = vec![0; uint8_array.length() as usize];
    uint8_array.copy_to(&mut body);

    log!("jwt: {:?}", jwt);

    let base64_data = encode(&body);
    let res = Client::new()
        .post("http://localhost:8000/listings/extract")
        .header("Authorization", &jwt)
        .json(&ExtractRequest { image: base64_data })
        .send()
        .await
        .unwrap();

    let resp = res.json::<ExtractResponse>().await.unwrap();

    let res = Client::new()
        .post("http://localhost:8000/listings")
        .header("Authorization", &jwt)
        .json(&CreateListingRequest {
            isbn: resp.isbn,
            blurb: resp.blurb,
        })
        .send()
        .await
        .unwrap();

    let resp = res.json::<CreateListingResponse>().await.unwrap();
    resp.listing_id
}

#[component]
pub fn NewListing() -> impl IntoView {
    let nav = use_navigate();
    let auth_state = expect_context::<Store<AuthState>>();

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
                        let jwt = auth_state.get().jwt;

                        let nav_clone = nav.clone();
                        spawn_local(async move {
                            let id = upload_picture(jwt, file).await;
                            nav_clone(&("/listing/".to_string() + &id), Default::default());
                        })
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
                    <span class="text-brown-400">"Take a clear picture of the "<strong>"back"</strong>" of your book"</span>
                </button>
            </div>
    </div>

    }
}

async fn fetch_listing(id: String) -> Result<GetListingResponse, String> {
    let client = Client::new();
    let url = format!("http://localhost:8080/listings/{}", &id);
    Ok(client
        .get(&url)
        .send()
        .await
        .unwrap()
        .json::<GetListingResponse>()
        .await
        .unwrap())
}

#[component]
pub fn Listing() -> impl IntoView {
    let params = use_params_map();
    let id = params.read().get("id").clone().unwrap_or_default();

    //view! {
    //    <div class="flex items-center justify-center flex-grow">
    //        <Await
    //            future=fetch_listing(id.clone())
    //            let:data
    //        >
    //            <h1>"hi"</h1>
    //        </Await>
    //    </div>
    //}

    let data = GetListingResponse {
        isbn: "123".to_string(),
        title: "Book title".to_string(),
        author: "Author's name".to_string(),
        user_fullname: "Freddy Snow".to_string(),
        user_id: "123".to_string(),
    };

    view! {
        <div class="flex flex-col flex-grow my-8">
            <div class="flex flex-row items-center justify-between">
                <h1 class="text-2xl text-brown-200">{data.title}</h1>
                <span class="text-brown-500">ISBN: {data.isbn}</span>
            </div>
            <h2 class="text-lg text-brown-300">"By "{data.author}</h2>
            <h2 class="text-lg mt-2 text-brown-400">"Listed for exchange by "<a class="hover:underline" href={format!("/profiles/{}", &data.user_id)}>{data.user_fullname}</a></h2>
        </div>
    }
}
