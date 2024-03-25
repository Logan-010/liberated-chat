use rusqlite::params;
use std::env;

fn main() {
    dotenv::dotenv().expect("Failed to load .env file. Is there one?");

    let path = format!(
        "{}/{}",
        env::var("DATABASE_PATH").expect("Set DATABASE_PATH env variable!"),
        env::var("DATABASE_NAME").expect("Set DATABASE_NAME env variable!")
    );

    let db = rusqlite::Connection::open(path).unwrap();

    db.execute("DELETE FROM posts;", params![]).unwrap();
}
