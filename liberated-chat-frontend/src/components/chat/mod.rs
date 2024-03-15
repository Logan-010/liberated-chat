use leptos::{component, view, IntoView};
use serde::Deserialize;

mod footer;

#[derive(Deserialize)]
struct Post {
    user: String,
    message: String,
    time: String,
}

impl Post {
    async fn new() -> Result<Vec<Post>, Box<dyn std::error::Error>> {
        let posts_string = crate::utils::posts::get_posts().await?;
        Ok(serde_json::from_str::<Vec<Post>>(&posts_string)?)
    }
}

impl ToString for Post {
    fn to_string(&self) -> String {
        format!("{}: {} - {}", self.user, self.message, self.time)
    }
}

#[component]
pub fn Chat() -> impl IntoView {
    view! { <footer::Footer></footer::Footer> }
}
