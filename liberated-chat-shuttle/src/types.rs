use serde::{Deserialize, Serialize};
use std::{
    env,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct AppState {
    //Mutex is best practice for a simple sqlite3 db
    pub db: Arc<Mutex<rusqlite::Connection>>,
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
        let path = format!(
            "{}/{}",
            env::var("DATABASE_PATH").expect("Set DATABASE_PATH env variable!"),
            env::var("DATABASE_NAME").expect("Set DATABASE_NAME env variable!")
        );
        let db = rusqlite::Connection::open(path).unwrap();

        db.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS users (
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS username_index ON users (username);
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
            CREATE INDEX IF NOT EXISTS sessions_index ON sessions (username, sessionId);
            ",
        )
        .unwrap();

        Self {
            db: Arc::new(Mutex::new(db)),
        }
    }
}
