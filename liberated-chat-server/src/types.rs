use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone)]
pub struct AppState {
    //Mutex is best practice for a simple sqlite3 db
    pub pool: Pool<SqliteConnectionManager>,
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
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager).expect("Failed to open database file!");

        pool.get()
            .expect("Failed to access database!")
            .execute_batch(
                "
                    PRAGMA journal_mode=WAL;
                    PRAGMA busy_timeout = 5000;
                    PRAGMA synchronous = NORMAL;
                    PRAGMA cache_size = 1000000000;
                    PRAGMA foreign_keys = true;
                    PRAGMA temp_store = memory;
                    CREATE TABLE IF NOT EXISTS users (
                        username TEXT NOT NULL UNIQUE,
                        password TEXT NOT NULL
                    ) STRICT;
                    CREATE INDEX IF NOT EXISTS username_index ON users (username);
                    CREATE TABLE IF NOT EXISTS posts (
                        postNum INTEGER PRIMARY KEY AUTOINCREMENT,
                        username TEXT NOT NULL,
                        message TEXT NOT NULL,
                        time TEXT NOT NULL
                    ) STRICT;
                    CREATE TABLE IF NOT EXISTS sessions (
                        username TEXT NOT NULL UNIQUE,
                        sessionId TEXT NOT NULL UNIQUE,
                        expiration INTEGER NOT NULL
                    ) STRICT;
                    CREATE INDEX IF NOT EXISTS sessions_index ON sessions (username, sessionId);
                ",
            )
            .unwrap();

        Self { pool }
    }
}
