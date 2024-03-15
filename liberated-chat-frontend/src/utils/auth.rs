use reqwest::StatusCode;
use std::path::Path;

pub async fn login(username: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(super::BASE_URL);

    let client = reqwest::Client::new();
    let req = client
        .post(path.join("login").to_str().unwrap())
        .header("Username", username)
        .header("Password", password)
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => {
            let text = req.text().await?;

            if text.is_empty() {
                Ok("Success!".into())
            } else {
                Ok(text)
            }
        }
        StatusCode::INTERNAL_SERVER_ERROR => Err("Internal service error!".into()),
        StatusCode::BAD_REQUEST => Err("Not all inputs provided!".into()),
        StatusCode::UNAUTHORIZED => Err("No/invalid login!".into()),
        StatusCode::CONFLICT => Err("No user exists!".into()),
        e => Err(format!("{e:?}").into()),
    }
}

pub async fn register(
    username: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(super::BASE_URL);

    let client = reqwest::Client::new();
    let req = client
        .post(path.join("register").to_str().unwrap())
        .header("Username", username)
        .header("Password", password)
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => {
            let text = req.text().await?;

            if text.is_empty() {
                Ok("Success!".into())
            } else {
                Ok(text)
            }
        }
        StatusCode::INTERNAL_SERVER_ERROR => Err("Internal service error!".into()),
        StatusCode::BAD_REQUEST => Err("Not all inputs provided!".into()),
        StatusCode::UNAUTHORIZED => Err("No/invalid login!".into()),
        StatusCode::CONFLICT => Err("User already exists!".into()),
        e => Err(format!("{e:?}").into()),
    }
}

pub async fn logout() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(super::BASE_URL);

    let client = reqwest::Client::new();
    let req = client
        .post(path.join("logout").to_str().unwrap())
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        StatusCode::INTERNAL_SERVER_ERROR => Err("Internal service error!".into()),
        StatusCode::BAD_REQUEST => Err("Not all inputs provided!".into()),
        StatusCode::UNAUTHORIZED => Err("Not logged in!".into()),
        e => Err(format!("{e:?}").into()),
    }
}

pub async fn send_message(message: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(super::BASE_URL);

    let client = reqwest::Client::new();
    let req = client
        .post(path.join("newpost").to_str().unwrap())
        .body(message)
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        StatusCode::INTERNAL_SERVER_ERROR => Err("Internal service error!".into()),
        StatusCode::BAD_REQUEST => Err("Not all inputs provided!".into()),
        StatusCode::UNAUTHORIZED => Err("No/invalid login!".into()),
        e => Err(format!("{e:?}").into()),
    }
}
