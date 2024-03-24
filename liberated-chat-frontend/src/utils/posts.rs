use std::path::Path;

use reqwest::StatusCode;

pub async fn get_posts() -> Result<String, Box<dyn std::error::Error>> {
    let url = Path::new(super::BASE_URL);

    let req = reqwest::get(url.join("posts").to_str().unwrap()).await?;

    match req.status() {
        StatusCode::OK => Ok(req.text().await?),
        StatusCode::INTERNAL_SERVER_ERROR => Err("Internal service error!".into()),
        StatusCode::BAD_REQUEST => Err("Not all inputs provided!".into()),
        StatusCode::UNAUTHORIZED => Err("No/invalid login!".into()),
        StatusCode::CONFLICT => Err("No user exists!".into()),
        e => Err(format!("{e:?}").into()),
    }
}
