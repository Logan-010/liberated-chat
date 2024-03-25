use leptos::{component, view, IntoView};
use serde::Deserialize;

mod footer;
mod message;

#[derive(Deserialize, Debug, Clone)]
struct Post {
    // Not used yet!
    _post_num: u64,
    user: String,
    message: String,
    time: String,
}

impl Post {
    async fn new() -> Result<Vec<Post>, Box<dyn std::error::Error>> {
        let posts_string = crate::utils::posts::get_posts().await?;
        Ok(serde_json::from_str(&posts_string)?)
    }
}

#[component]
pub fn Chat() -> impl IntoView {
    view! {
        <message::Messages></message::Messages>
        <footer::Footer></footer::Footer>
    }
}
