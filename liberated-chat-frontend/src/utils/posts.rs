use std::path::Path;

pub async fn get_posts() -> Result<String, reqwest::Error> {
    let url = Path::new(super::BASE_URL);

    let req = reqwest::get(url.join("posts").to_str().unwrap()).await?;

    req.text().await
}
