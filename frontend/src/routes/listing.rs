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
            isbn: "dsdhfusuhf".to_string(),
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

    let isbn = RwSignal::new("".to_string());
    let file_ref: NodeRef<leptos::html::Input> = NodeRef::new();

    view! {
        <div class="flex flex-col items-center justify-center flex-grow max-w-md w-full mx-auto gap-8">
            <h1 class="text-lg font-bold leading-none mb-1">"Let's list your book for exchange!"</h1>
            <div class="flex flex-col gap-1">
                <h2 class="text-lg text-brown-200 leading-none mb-1">"First, write the ISBN of your book (Found on the back)"</h2>
                <input
                    type="text"
                    placeholder="ISBN"
                    class="w-full bg-brown-800 border border-brown-700 px-2 py-1"
                    bind:value=isbn
                />
            </div>
            <div class="flex flex-col items-center w-full">
                <h2 class="text-lg text-brown-200 leading-none">"Next, take a picture of the back of your book"</h2>
                <div class="my-4">
                    <Icon icon={i::BsArrowDown} {..} style="color: var(--color-brown-200)"/>
                </div>
                <div class="w-full">
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
                        disabled={(isbn.get().len() == 0).to_string()}
                        class="flex flex-col border border-brown-700 border-dashed rounded-sm w-full py-12 items-center cursor-pointer hover:bg-[#00000011] transition-colors disabled:cursor-not-allowed"
                    >
                        <span class="text-brown-300">"Upload a picture"</span>
                        <span class="text-brown-400">"Take a clear picture of the "<strong>"back"</strong>" of your book"</span>
                    </button>
                </div>
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
    let nav = use_navigate();
    let params = use_params_map();
    let id = params.read().get("id").clone().unwrap_or_default();

    let auth_state = expect_context::<Store<AuthState>>();

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
        similar_listings: vec![
            GetSimilarListingResponse {
                id: "uhhh".to_string(),
                isbn: "123".to_string(),
                title: "A similar book".to_string(),
                author: "another author".to_string(),
            },
            GetSimilarListingResponse {
                id: "uhhh".to_string(),
                isbn: "123".to_string(),
                title: "A similar book".to_string(),
                author: "another author".to_string(),
            },
            GetSimilarListingResponse {
                id: "uhhh".to_string(),
                isbn: "123".to_string(),
                title: "A similar book".to_string(),
                author: "another author".to_string(),
            },
            GetSimilarListingResponse {
                id: "uhhh".to_string(),
                isbn: "123".to_string(),
                title: "A similar book".to_string(),
                author: "another author".to_string(),
            },
            GetSimilarListingResponse {
                id: "uhhh".to_string(),
                isbn: "123".to_string(),
                title: "A similar book".to_string(),
                author: "another author".to_string(),
            },
        ],
    };

    let delete = async move |jwt: String, listingId: String| {
        let client = Client::builder().build().unwrap();
        let res = client
            .delete(format!("http://localhost:8000/listings/{}", listingId))
            .header("Authorization", &jwt)
            .send()
            .await
            .unwrap();

        if res.status() != 204 {
            return;
        }

        nav("/", Default::default());
    };

    view! {
        <div class="flex flex-col flex-grow my-8">
            <div class="flex flex-row items-center justify-between">
                <h1 class="text-2xl text-brown-200">{data.title}</h1>
                <div class="flex flex-col items-end gap-1">
                    <button
                        on:click = move |_| {
                            let id_clone = id.clone();
                            let delete_clone = delete.clone();
                            let jwt = auth_state.get().jwt;
                            spawn_local(async move { delete_clone(jwt, id_clone).await; } );
                        }
                        class="flex flex-row gap-2 items-center block px-4 py-1 bg-brown-200 rounded-sm text-stone-900 hover:bg-brown-100 transition-colors cursor-pointer text-sm"
                    >
                        <Icon icon={i::BsTrashFill} {..} style="color: var(--color-stone-900)"/>
                        "Delist"
                    </button>
                    <span class="text-brown-500">"isbn: "{data.isbn}</span>
                </div>
            </div>
            <h2 class="text-lg text-brown-300">"By "{data.author}</h2>
            <h2 class="text-lg mt-2 text-brown-300">"Listed for exchange by "<a class="text-brown-400 hover:text-brown-300 hover:underline" href={format!("/profiles/{}", &data.user_id)}>{data.user_fullname}</a></h2>
            <h2 class="text-xl text-brown-200 mt-8 mb-2">"Other similar books you might be interested in"</h2>
            <div class="flex flex-row flex-wrap gap-2">
                {data.similar_listings.iter().map(|listing| view! {
                    <div class="p-8 bg-brown-800 flex-grow border border-brown-700 hover:bg-brown-700 cursor-pointer transition-colors">
                        <h1 class="text-brown-100 font-bold">{listing.title.clone()}</h1>
                        <h2 class="text-brown-200 font-bold">"By "{listing.author.clone()}</h2>
                        <a href={format!("/listing/{}", listing.id)} class="flex flex-row gap-2 mt-4 items-center">
                            <h3 class="text-brown-300">"Go to listing"</h3>
                            <Icon icon={i::FiExternalLink} {..} style="color: var(--color-brown-300)"/>
                        </a>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
