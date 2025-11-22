use wasm_bindgen::prelude::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use web_sys::window;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub sub: String,
    pub name: String,
    pub picture: String,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct AuthState {
    pub user: Option<UserProfile>,
    pub token: Option<String>,
}

// Global signal for auth state
// We'll use a context for this in a real app, but for now a global signal or window event is easier to bridge.
// Actually, let's use a RwSignal that we can pass around or access via context.

pub fn get_stored_token() -> Option<String> {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|ls| ls.get_item("google_token").ok().flatten())
}

pub fn set_stored_token(token: Option<String>) {
    if let Some(w) = window() {
        if let Ok(Some(ls)) = w.local_storage() {
            if let Some(t) = token {
                let _ = ls.set_item("google_token", &t);
            } else {
                let _ = ls.remove_item("google_token");
            }
        }
    }
}

// Decode JWT (simplified, no verification on client side)
pub fn decode_jwt(token: &str) -> Option<UserProfile> {
    use base64::{Engine as _, engine::general_purpose};
    
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    
    let payload = parts[1];
    // Base64 decode
    // Need to handle URL safe base64
    let payload = payload.replace('-', "+").replace('_', "/");
    // Add padding if needed
    let padding = match payload.len() % 4 {
        2 => "==",
        3 => "=",
        _ => "",
    };
    let payload = format!("{}{}", payload, padding);
    
    let decoded = general_purpose::STANDARD.decode(&payload).ok()?;
    let json_str = String::from_utf8(decoded).ok()?;
    
    serde_json::from_str(&json_str).ok()
}

// Exported function for JS to call
#[wasm_bindgen]
pub fn handle_credential_response(response: JsValue) {
    let response_obj: js_sys::Object = response.into();
    let credential_key = JsValue::from_str("credential");
    
    if let Ok(credential) = js_sys::Reflect::get(&response_obj, &credential_key) {
        if let Some(token) = credential.as_string() {
            web_sys::console::log_1(&format!("Got token: {}", token).into());
            set_stored_token(Some(token.clone()));
            
            // Reload page to refresh state (simplest way for now)
            // Or dispatch a custom event
            if let Some(w) = window() {
                let _ = w.location().reload();
            }
        }
    }
}
