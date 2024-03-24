use rusqlite::params;

fn main() {
    let path = format!(
        "{}/{}",
        env!("DATABASE_PATH", "Set DATABASE_PATH!"),
        env!("DATABASE_NAME", "Set DATABASE_NAME!")
    );
    let db = rusqlite::Connection::open(path).unwrap();

    db.execute("DELETE FROM posts;", params![]).unwrap();
}
