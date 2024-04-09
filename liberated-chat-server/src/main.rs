mod types;
mod utils;

use axum::{
    body::Bytes,
    extract::{Request, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use std::env;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
};

const FAVICON: &[u8] = include_bytes!("../favicon.ico");

async fn favicon() -> Bytes {
    Bytes::from(FAVICON)
}

async fn login(
    jar: CookieJar,
    State(state): State<types::AppState>,
    req: Request,
) -> Result<CookieJar, StatusCode> {
    let headers = req.headers();
    let username = headers
        .get("Username")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let password = headers
        .get("Password")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    if username.is_empty() || password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(cookie) = jar.get("Liberated-Chat-Auth") {
        if let Ok(v) = utils::validate_session(username, cookie.value(), &db) {
            if v {
                return Err(StatusCode::OK);
            }
        }
    }

    let valid =
        utils::validate_password(username, password, &db).map_err(|_| StatusCode::CONFLICT)?;

    if valid {
        let session = utils::generate_session(username, &db)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(jar.add(Cookie::new("Liberated-Chat-Auth", session)))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn register(
    State(state): State<types::AppState>,
    req: Request,
) -> Result<String, StatusCode> {
    let headers = req.headers();
    let username = headers
        .get("Username")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let password = headers
        .get("Password")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    if username.is_empty() || password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let hashed_password = utils::hash(password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    utils::register_user(username, &hashed_password, &db).map_err(|_| StatusCode::CONFLICT)?;
    Ok("Success!".into())
}

async fn posts(jar: CookieJar, State(state): State<types::AppState>) -> Result<String, StatusCode> {
    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_cookie = if let Some(cookie) = jar.get("Liberated-Chat-Auth") {
        cookie.value().to_string()
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let username = utils::get_username_from_session(&auth_cookie, &db)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let authorized = utils::validate_session(&username, &auth_cookie, &db)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if authorized {
        let posts = utils::get_posts(&db).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        serde_json::to_string(&posts).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn newpost(
    jar: CookieJar,
    State(state): State<types::AppState>,
    body: Bytes,
) -> Result<String, StatusCode> {
    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_cookie = if let Some(cookie) = jar.get("Liberated-Chat-Auth") {
        cookie.value().to_string()
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let username = utils::get_username_from_session(&auth_cookie, &db)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let authorized = utils::validate_session(&username, &auth_cookie, &db)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !authorized {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        let post = types::InsertPost {
            user: username,
            message: String::from_utf8_lossy(body.as_ref()).into(),
            time: utils::get_formatted_time(),
        };

        utils::send_message(&post, &db).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok("Success".into())
    }
}

async fn logout(
    jar: CookieJar,
    State(state): State<types::AppState>,
) -> Result<(CookieJar, String), StatusCode> {
    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_cookie = if let Some(cookie) = jar.get("Liberated-Chat-Auth") {
        cookie.value().to_string()
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let username = utils::get_username_from_session(&auth_cookie, &db)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let authorized = utils::validate_session(&username, &auth_cookie, &db)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !authorized {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        utils::logout(&username, &db).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok((
            jar.remove(Cookie::from("Liberated-Chat-Auth")),
            "Success!".into(),
        ))
    }
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "Nothing to see here. Error 404 page not found.",
    )
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file. Is there one?");

    let port: u16 = env::var("SERVER_PORT")
        .expect("Set SERVER_PORT env variable!")
        .parse()
        .unwrap();

    let state = types::AppState::new();

    println!("Listening on:\n\thttp://localhost:{port}");

    let frontend_path = env::var("FRONTEND_PATH").expect("Set FRONTEND_PATH!");
    let routes = Router::new()
        .route("/favicon", get(favicon))
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/posts", get(posts))
        .route("/newpost", post(newpost))
        .route("/logout", post(logout))
        .nest_service(
            "/",
            ServeDir::new(&frontend_path)
                .fallback(ServeFile::new(format!("{}/index.html", &frontend_path))),
        )
        .fallback(handler_404)
        .with_state(state)
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind(("localhost", port))
        .await
        .unwrap();

    axum::serve(listener, routes).await.unwrap();
}
