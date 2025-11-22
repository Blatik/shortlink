mod app;
mod components;
mod pages;
mod api;
mod auth;

use app::App;
use leptos::*;
use wasm_bindgen::JsValue;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    
    // Expose handle_credential_response to window for Google Sign-In
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::window;

    let cb = Closure::wrap(Box::new(move |response: JsValue| {
        auth::handle_credential_response(response);
    }) as Box<dyn FnMut(JsValue)>);

    if let Some(w) = window() {
        let _ = js_sys::Reflect::set(
            &w, 
            &JsValue::from_str("handleCredentialResponse"), 
            cb.as_ref().unchecked_ref()
        );
    }
    cb.forget(); // Keep it alive

    mount_to_body(|| view! { <App/> })
}
