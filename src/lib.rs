use url::Url;
use wasm_bindgen::prelude::*;
use web_sys::{Headers, Request, Response, ResponseInit};

include!(concat!(env!("OUT_DIR"), "/map.rs"));

mod rewrite_map;

const HOME_PAGE: &str = include_str!("home.html");

#[wasm_bindgen]
pub async fn handler(request: Request) -> Result<Response, JsValue> {
    console_error_panic_hook::set_once();

    // Determine the key from the url
    let url: Url = request.url().parse().unwrap();
    let path = url.path();

    // Serve a basic HTML page for the home page
    if path == "/" {
        return home(request).await;
    }

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

pub async fn home(_request: Request) -> Result<Response, JsValue> {
    let mut init = ResponseInit::default();
    let headers = Headers::new()?;
    headers.set("Content-Type", "text/html")?;
    init.headers(&headers);

    Response::new_with_opt_str_and_init(Some(HOME_PAGE), &init)
}
