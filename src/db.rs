use rusqlite::{Connection, params};
use seahash::SeaHasher;
use std::hash::Hasher;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<Connection>>,
}

impl AppState {
    pub fn new(db_dir: &str) -> Self {
        let db_path = Path::new(db_dir).join("webnote.db3");
        let conn = Connection::open(&db_path).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS webnote (
                hash_key INTEGER PRIMARY KEY,
                uid TEXT NOT NULL UNIQUE CHECK(LENGTH(uid) <= 16),
                content TEXT NOT NULL
            )",
            [],
        )
        .unwrap();

        AppState {
            db: Arc::new(Mutex::new(conn)),
        }
    }

    pub fn get_content(&self, uid: &str) -> Result<String, rusqlite::Error> {
        let hash_key = conv_hash(uid);
        let db = self.db.lock().unwrap();

        db.query_row(
            "SELECT content FROM webnote WHERE hash_key = ?1",
            params![hash_key],
            |row| row.get::<_, String>(0),
        )
    }

    pub fn save_content(&self, uid: &str, content: &str) -> rusqlite::Result<()> {
        let hash_key = conv_hash(uid);
        let db = self.db.lock().unwrap();

        db.execute(
            "INSERT INTO webnote (hash_key, uid, content) VALUES (?1, ?2, ?3)
             ON CONFLICT(hash_key) DO UPDATE SET content = excluded.content",
            params![hash_key, uid, content],
        )?;
        Ok(())
    }
}

fn conv_hash(uid: &str) -> i64 {
    let mut hasher = SeaHasher::new();
    hasher.write(uid.as_bytes());
    let hash = hasher.finish();
    hash as i64
}
