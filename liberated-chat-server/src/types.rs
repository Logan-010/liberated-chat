use actix_web::http::StatusCode;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Display, Error)]
pub enum AppError {
    DatabaseError,
    InternalError,
    UserError,
    NotLoggedIn,
    WrongLogin,
    UserAlreadyExists,
    UserDoesNotExist,
    AlreadyLoggedIn,
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UserError => StatusCode::BAD_REQUEST,
            AppError::NotLoggedIn => StatusCode::UNAUTHORIZED,
            AppError::WrongLogin => StatusCode::UNAUTHORIZED,
            AppError::UserAlreadyExists => StatusCode::CONFLICT,
            AppError::UserDoesNotExist => StatusCode::CONFLICT,
            AppError::AlreadyLoggedIn => StatusCode::OK,
        }
    }
}

pub struct AppState {
    //Mutex is best practice for a simple sqlite3 db
    pub db: Mutex<rusqlite::Connection>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Post {
    pub post_num: u64,
    pub user: String,
    pub message: String,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InsertPost {
    pub user: String,
    pub message: String,
    pub time: String,
}

impl AppState {
    pub fn new() -> Self {
        let db =
            rusqlite::Connection::open(std::env::var("DATABASE_URL").expect("Set DATABASE_URL!"))
                .unwrap();

        db.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS users (
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS posts (
                postNum INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                message TEXT NOT NULL,
                time TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS sessions (
                username TEXT NOT NULL UNIQUE,
                sessionId TEXT NOT NULL UNIQUE,
                expiration INTEGER NOT NULL
            );
            ",
        )
        .unwrap();

        Self { db: Mutex::new(db) }
    }
}
