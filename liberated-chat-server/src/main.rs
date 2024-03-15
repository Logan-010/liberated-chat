use actix_web::cookie::Cookie;
use actix_web::{get, post, HttpResponse};
use actix_web::{App, HttpServer};
mod types;
mod utils;

#[post("/login")]
async fn login(
    req: actix_web::HttpRequest,
    state: actix_web::web::Data<types::AppState>,
) -> Result<HttpResponse, types::AppError> {
    let username = req
        .headers()
        .get("Username")
        .and_then(|v| v.to_str().ok())
        .ok_or(types::AppError::UserError)?;

    let password = req
        .headers()
        .get("Password")
        .and_then(|v| v.to_str().ok())
        .ok_or(types::AppError::UserError)?;

    if username.is_empty() || password.is_empty() {
        return Err(types::AppError::UserError);
    }

    let db = state
        .db
        .lock()
        .map_err(|_| types::AppError::InternalError)?;

    if let Some(cookie) = req.cookie("Liberated-Chat-Auth") {
        if let Ok(v) = utils::validate_session(username, cookie.value(), &db) {
            if v {
                return Err(types::AppError::AlreadyLoggedIn);
            }
        }
    }

    let valid = utils::validate_password(username, password, &db)
        .map_err(|_| types::AppError::UserDoesNotExist)?;

    if valid {
        let session = utils::generate_session(username, &db).map_err(|e| {
            println!("{e:?}");
            types::AppError::DatabaseError
        })?;

        let cookie = Cookie::build("Liberated-Chat-Auth", session)
            .max_age(actix_web::cookie::time::Duration::days(2))
            .secure(true)
            .finish();

        Ok(HttpResponse::Ok().cookie(cookie).finish())
    } else {
        Err(types::AppError::WrongLogin)
    }
}

#[post("/register")]
async fn register_user(
    req: actix_web::HttpRequest,
    state: actix_web::web::Data<types::AppState>,
) -> Result<String, types::AppError> {
    let username = req
        .headers()
        .get("Username")
        .and_then(|v| v.to_str().ok())
        .ok_or(types::AppError::UserError)?;

    let password = req
        .headers()
        .get("Password")
        .and_then(|v| v.to_str().ok())
        .ok_or(types::AppError::UserError)?;

    if username.is_empty() || password.is_empty() {
        return Err(types::AppError::UserError);
    }

    let hashed_password = utils::hash(password).map_err(|_| types::AppError::InternalError)?;

    let db = state
        .db
        .lock()
        .map_err(|_| types::AppError::InternalError)?;

    utils::register_user(username, &hashed_password, &db)
        .map_err(|_| types::AppError::UserAlreadyExists)?;

    Ok("Success!".into())
}

#[get("/posts")]
async fn get_posts(
    req: actix_web::HttpRequest,
    state: actix_web::web::Data<types::AppState>,
) -> Result<String, types::AppError> {
    let db = state
        .db
        .lock()
        .map_err(|_| types::AppError::InternalError)?;

    let auth_cookie = if let Some(cookie) = req.cookie("Liberated-Chat-Auth") {
        cookie.value().to_string()
    } else {
        return Err(types::AppError::NotLoggedIn);
    };

    let username = utils::get_username_from_session(&auth_cookie, &db)
        .map_err(|_| types::AppError::NotLoggedIn)?;

    let authorized = utils::validate_session(&username, &auth_cookie, &db)
        .map_err(|_| types::AppError::NotLoggedIn)?;

    if authorized {
        let posts = utils::get_posts(&db).map_err(|_| types::AppError::DatabaseError)?;

        serde_json::to_string(&posts).map_err(|_| types::AppError::InternalError)
    } else {
        Err(types::AppError::NotLoggedIn)
    }
}

#[post("/newpost")]
async fn new_post(
    body: actix_web::web::Bytes,
    req: actix_web::HttpRequest,
    state: actix_web::web::Data<types::AppState>,
) -> Result<String, types::AppError> {
    let db = state
        .db
        .lock()
        .map_err(|_| types::AppError::InternalError)?;

    let auth_cookie = if let Some(cookie) = req.cookie("Liberated-Chat-Auth") {
        cookie.value().to_string()
    } else {
        return Err(types::AppError::NotLoggedIn);
    };

    let username = utils::get_username_from_session(&auth_cookie, &db)
        .map_err(|_| types::AppError::NotLoggedIn)?;

    let authorized = utils::validate_session(&username, &auth_cookie, &db)
        .map_err(|_| types::AppError::DatabaseError)?;

    if !authorized {
        Err(types::AppError::NotLoggedIn)
    } else {
        let post = types::InsertPost {
            user: username,
            message: String::from_utf8_lossy(body.as_ref()).into(),
            time: utils::get_formatted_time(),
        };

        utils::send_message(&post, &db).map_err(|_| types::AppError::DatabaseError)?;

        Ok("Success".into())
    }
}

#[post("/logout")]
async fn logout(
    req: actix_web::HttpRequest,
    state: actix_web::web::Data<types::AppState>,
) -> Result<HttpResponse, types::AppError> {
    let db = state
        .db
        .lock()
        .map_err(|_| types::AppError::InternalError)?;

    let auth_cookie = if let Some(cookie) = req.cookie("Liberated-Chat-Auth") {
        cookie.value().to_string()
    } else {
        return Err(types::AppError::NotLoggedIn);
    };

    let username = utils::get_username_from_session(&auth_cookie, &db)
        .map_err(|_| types::AppError::NotLoggedIn)?;

    let authorized = utils::validate_session(&username, &auth_cookie, &db)
        .map_err(|_| types::AppError::DatabaseError)?;

    if !authorized {
        Err(types::AppError::NotLoggedIn)
    } else {
        utils::logout(&username, &db).map_err(|_| types::AppError::DatabaseError)?;

        let mut response = HttpResponse::Ok().body("Success!");
        response
            .add_removal_cookie(&Cookie::new("Liberated-Chat-Auth", ""))
            .map_err(|_| types::AppError::InternalError)?;

        Ok(response)
    }
}

async fn page_404() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::NotFound().body("404 Page not found."))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("SERVER_PORT")
        .expect("Set SERVER_PORT!")
        .parse()
        .unwrap();

    let state = actix_web::web::Data::new(types::AppState::new());

    println!("Listening on:\n\thttp://127.0.0.1:{port}\n\thttp://[::1]:{port}\n\t");

    HttpServer::new(move || {
        App::new()
            //Clone required, actix uses multiple threads.
            .app_data(state.clone())
            .wrap(actix_web::middleware::Compress::default())
            .service(register_user)
            .service(login)
            .service(get_posts)
            .service(new_post)
            .service(logout)
            //ALWAYS have static files last, unless no other path will match anything.
            .service(
                actix_files::Files::new("/", "./liberated-chat-frontend/dist")
                    .index_file("index.html"),
            )
            .default_service(actix_web::web::route().to(page_404))
    })
    .bind(("localhost", port))?
    .run()
    .await
}
