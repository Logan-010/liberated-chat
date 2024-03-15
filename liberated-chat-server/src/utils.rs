use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rusqlite::params;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time::{format_description, OffsetDateTime};

fn get_two_days() -> u64 {
    let now = SystemTime::now();
    let two_days = Duration::from_secs(2 * 24 * 60 * 60);
    let two_days_from_now = now + two_days;
    //Unwrap can never be reached, so long as system time is not before unix timestamp
    two_days_from_now
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn get_time() -> u64 {
    let now = SystemTime::now();
    //Unwrap can never be reached, so long as system time is not before unix timestamp
    now.duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn get_formatted_time() -> String {
    let now = OffsetDateTime::now_utc();
    let format = format_description::parse("[month]/[day]/[year] @ [hour]:[minute]").unwrap();
    now.format(&format).unwrap()
}

pub fn hash(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn verify_hash(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn register_user(
    username: &str,
    password: &str,
    db: &rusqlite::Connection,
) -> Result<(), rusqlite::Error> {
    let mut stmt = db.prepare("INSERT INTO users VALUES (?, ?);")?;

    stmt.execute(params![username, password])?;

    Ok(())
}

pub fn generate_session(
    username: &str,
    db: &rusqlite::Connection,
) -> Result<String, rusqlite::Error> {
    let session = uuid::Uuid::new_v4().to_string();

    db.execute(
        "DELETE FROM sessions WHERE username = ?;",
        params![username],
    )?;

    db.execute(
        "INSERT INTO sessions VALUES (?, ?, ?);",
        params![username, session, get_two_days()],
    )?;

    Ok(session)
}

pub fn validate_password(
    username: &str,
    password: &str,
    db: &rusqlite::Connection,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut stmt = db.prepare("SELECT password FROM users WHERE username = ?;")?;

    let password_hash: String = stmt.query_row(params![username], |row| row.get::<_, String>(0))?;

    let authorized = match verify_hash(password, &password_hash) {
        Ok(v) => v,
        Err(e) => return Err(format!("{e:?}").into()),
    };

    Ok(authorized)
}

pub fn validate_session(
    username: &str,
    session: &str,
    db: &rusqlite::Connection,
) -> Result<bool, rusqlite::Error> {
    let mut stmt = db.prepare("SELECT sessionId, expiration FROM sessions WHERE username = ?;")?;
    let session_info = stmt.query_row(params![username], |row| {
        // Fetch sessionId and expiration from the row
        let session_id: String = row.get(0)?;
        let expiration: u64 = row.get(1)?;

        // Return session_id and expiration as a tuple
        Ok((session_id, expiration))
    })?;

    // Unpack the session info tuple
    let (session_id, expiration) = session_info;

    if (session == session_id) && (get_time() < expiration) {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn get_username_from_session(
    session: &str,
    db: &rusqlite::Connection,
) -> Result<String, rusqlite::Error> {
    let mut stmt = db.prepare("SELECT username FROM sessions WHERE sessionId = ?;")?;
    stmt.query_row(params![session], |row| row.get::<_, String>(0))
}

pub fn get_posts(db: &rusqlite::Connection) -> Result<Vec<super::types::Post>, rusqlite::Error> {
    let mut stmt = db.prepare("SELECT * FROM posts;")?;

    let posts_iter = stmt.query_map(params![], |row| {
        Ok(super::types::Post {
            post_num: row.get(0)?,
            user: row.get(1)?,
            message: row.get(2)?,
            time: row.get(3)?,
        })
    })?;

    let posts: Vec<super::types::Post> = match posts_iter.collect() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(posts)
}

pub fn send_message(
    message: &super::types::InsertPost,
    db: &rusqlite::Connection,
) -> Result<(), rusqlite::Error> {
    db.execute(
        "INSERT INTO posts (username, message, time) VALUES (?, ?, ?);",
        params![message.user, message.message, message.time],
    )?;

    Ok(())
}

pub fn logout(username: &str, db: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    db.execute(
        "DELETE FROM sessions WHERE username = ?;",
        params![username],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password_hash() {
        let password = "Hello, world!";

        //Creates test db
        let db = rusqlite::Connection::open_in_memory().unwrap();

        db.execute(
            "CREATE TABLE IF NOT EXISTS users (
                        username TEXT NOT NULL UNIQUE,
                        password BLOB NOT NULL
                    );",
            rusqlite::params![],
        )
        .unwrap();

        let hash = hash(password).unwrap();

        db.execute(
            "INSERT INTO users VALUES ('john', ?)",
            rusqlite::params![hash],
        )
        .unwrap();

        let valid = validate_password("john", password, &db).unwrap();

        //Ensure hash is valid
        assert!(valid);

        db.close().unwrap();
    }

    #[test]
    fn test_new_user() {
        //Creates test database
        let db = rusqlite::Connection::open_in_memory().unwrap();

        db.execute(
            "CREATE TABLE IF NOT EXISTS users (
                        username TEXT NOT NULL UNIQUE,
                        password BLOB NOT NULL
                    );",
            rusqlite::params![],
        )
        .unwrap();

        //Should not return error
        assert!(super::register_user("jack", "password", &db).ok().is_some());

        //Should return error
        assert!(super::register_user("jack", "password", &db)
            .err()
            .is_some());

        db.close().unwrap();
    }

    #[test]
    fn test_validate_session() {
        // Create a test session for validation
        let test_session_id = uuid::Uuid::new_v4().to_string();
        let test_username = "test_user";
        let test_expiration = get_two_days();

        let db = rusqlite::Connection::open_in_memory().unwrap();

        db.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                        username TEXT NOT NULL,
                        sessionId TEXT NOT NULL,
                        expiration INTEGER NOT NULL
                    );",
            rusqlite::params![],
        )
        .unwrap();

        // Insert the test session into the database
        db.execute(
            "INSERT INTO sessions VALUES (?, ?, ?);",
            params![test_username, test_session_id, test_expiration],
        )
        .unwrap();

        // Validate session with correct session ID and expiration time
        assert!(validate_session(test_username, &test_session_id, &db).unwrap());

        // Validate session with incorrect session ID
        assert!(!validate_session(test_username, "invalid_session_id", &db).unwrap());

        // Validate session for a non-existent user
        assert!(validate_session("nonexistent_user", &test_session_id, &db).is_err());

        db.close().unwrap();
    }
}
