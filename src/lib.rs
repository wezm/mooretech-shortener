use url::Url;
use wasm_bindgen::prelude::*;
use web_sys::{Headers, Request, Response, ResponseInit};

include!(concat!(env!("OUT_DIR"), "/map.rs"));

mod rewrite_map;

#[wasm_bindgen]
pub async fn handler(request: Request) -> Result<Response, JsValue> {
    console_error_panic_hook::set_once();

    // Determine the key from the url
    let url: Url = request.url().parse().unwrap();
    let path = url.path();

    // Look it up in the rewrite map, return a redirect
    if let Ok(idx) = MAPPINGS.binary_search_by_key(&path, |entry| entry.0) {
        let (_, destination) = MAPPINGS[idx];
        let headers = Headers::new()?;
        headers.set("Location", destination)?;

        let mut init = ResponseInit::default();
        init.headers(&headers).status(302);

        Response::new_with_opt_str_and_init(Some("Redirecting"), &init)
    } else {
        let mut init = ResponseInit::default();
        init.status(404);

        Response::new_with_opt_str_and_init(Some("Not found"), &init)
    }
}
