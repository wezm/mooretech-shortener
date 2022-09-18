use url::Url;
use wasm_bindgen::prelude::*;
use web_sys::{Headers, Request, Response, ResponseInit};

include!(concat!(env!("OUT_DIR"), "/map.rs"));

#[wasm_bindgen]
pub async fn handler(request: Request) -> Result<Response, JsValue> {
    console_error_panic_hook::set_once();

    // Determine the key from the url
    let url: Url = request.url().parse().unwrap();

    // Look it up in the rewrite map, return a redirect
    if let Some((_key, destination)) = MAPPINGS.iter().find(|(key, _value)| *key == url.path()) {
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
