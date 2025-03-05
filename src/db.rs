use rusqlite::{Connection, params};
use seahash::SeaHasher;
use std::hash::Hasher;
use std::path::Path;
use std::sync::{Arc, Mutex};

const DB_FILENAME: &str = "webnote.db3";
const MAX_UID_LENGTH: usize = 16;

const PRAGMA_SETTINGS: &str = "
PRAGMA journal_mode=WAL;
PRAGMA synchronous=NORMAL;
PRAGMA wal_autocheckpoint=100;
PRAGMA foreign_keys=ON;
";
const CREATE_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS webnote (
    hash_key INTEGER PRIMARY KEY,
    uid TEXT NOT NULL UNIQUE CHECK(LENGTH(uid) <= ?),
    content TEXT NOT NULL
);
";
const GET_CONTENT_SQL: &str = "
SELECT content FROM webnote WHERE hash_key = ?1
";
const UPSERT_CONTENT_SQL: &str = "
INSERT INTO webnote (hash_key, uid, content) VALUES (?1, ?2, ?3)
ON CONFLICT(hash_key) DO UPDATE SET 
    content = excluded.content
";

#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<Connection>>,
}

impl AppState {
    pub fn new(db_dir: &str) -> Result<Self, rusqlite::Error> {
        let db_path = Path::new(db_dir).join(DB_FILENAME);
        let conn = Connection::open(&db_path)?;

        conn.execute_batch(PRAGMA_SETTINGS)?;
        conn.execute(
            &CREATE_TABLE_SQL.replace("?", &MAX_UID_LENGTH.to_string()),
            [],
        )?;

        Ok(AppState {
            db: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn get_content(&self, uid: &str) -> Result<String, rusqlite::Error> {
        let hash_key = conv_hash(uid);
        let db = self
            .db
            .lock()
            .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;

        let mut stmt = db.prepare(GET_CONTENT_SQL)?;
        match stmt.query_row(params![hash_key], |row| row.get::<_, String>(0)) {
            Ok(content) => Ok(content),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(String::new()),
            Err(e) => Err(e),
        }
    }

    pub fn save_content(&self, uid: &str, content: &str) -> rusqlite::Result<()> {
        let hash_key = conv_hash(uid);
        let db = self
            .db
            .lock()
            .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;

        db.execute(UPSERT_CONTENT_SQL, params![hash_key, uid, content])?;
        Ok(())
    }
}

fn conv_hash(uid: &str) -> i64 {
    let mut hasher = SeaHasher::new();
    hasher.write(uid.as_bytes());
    hasher.finish() as i64
}
