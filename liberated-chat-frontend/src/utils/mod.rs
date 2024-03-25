pub mod auth;
pub mod posts;

pub fn get_base_url() -> Option<String> {
    if let Some(window) = leptos::web_sys::window() {
        Some(window.location().href().ok()?.to_string())
    } else {
        None
    }
}
