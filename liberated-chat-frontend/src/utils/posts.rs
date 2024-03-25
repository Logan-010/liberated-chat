use std::path::Path;

use reqwest::StatusCode;

pub async fn get_posts() -> Result<String, Box<dyn std::error::Error>> {
    let base_url = super::get_base_url().expect("Failed to get base url!");
    let path = Path::new(&base_url);

    let req = reqwest::get(path.join("posts").to_str().unwrap()).await?;

    match req.status() {
        StatusCode::OK => Ok(req.text().await?),
        StatusCode::INTERNAL_SERVER_ERROR => Err("Internal service error!".into()),
        StatusCode::BAD_REQUEST => Err("Not all inputs provided!".into()),
        StatusCode::UNAUTHORIZED => Err("No/invalid login!".into()),
        StatusCode::CONFLICT => Err("No user exists!".into()),
        e => Err(format!("{e:?}").into()),
    }
}
