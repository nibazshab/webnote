use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::{Error, Row, SqlitePool};
use std::str::FromStr;
use std::{env, path};

use crate::var::Note;
use crate::{features, utils};

impl Note {
    pub async fn write(&self, pool: &SqlitePool) -> Result<(), Error> {
        let key = utils::hash(&self.id);

        sqlx::query(
            "
INSERT INTO notes (key, id, content) VALUES (?1, ?2, ?3)
ON CONFLICT(key) DO UPDATE SET
    content = excluded.content
",
        )
        .bind(key)
        .bind(&self.id)
        .bind(&self.content)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn read(&mut self, pool: &SqlitePool) -> Result<(), Error> {
        let key = utils::hash(&self.id);

        if let Some(rs) = sqlx::query("SELECT content FROM notes WHERE key = ?")
            .bind(key)
            .fetch_optional(pool)
            .await?
        {
            self.content = rs.get("content")
        }

        Ok(())
    }
}

pub async fn init_database() -> Result<SqlitePool, Error> {
    let dir = env::var("DATA_DIR").ok().unwrap_or_else(|| {
        let mut path = env::current_exe().unwrap();
        path.pop();
        path.display().to_string()
    });

    let db_url = path::Path::new(format!("sqlite:{dir}").as_str())
        .join("note.db")
        .display()
        .to_string();

    let options = SqliteConnectOptions::from_str(&db_url)?
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .create_if_missing(true);

    println!("Connecting to {db_url}");
    let pool = SqlitePool::connect_with(options).await?;

    create_table(&pool).await?;
    Ok(pool)
}

async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
    let mut sql = String::from(
        "
CREATE TABLE IF NOT EXISTS notes (
    key INTEGER PRIMARY KEY,
    id TEXT NOT NULL,
    content TEXT NOT NULL
);",
    );

    sql.push_str(
        #[cfg(feature = "file")]
        features::file::CREATE_TABLE_FILE,
        #[cfg(not(feature = "file"))]
        "",
    );

    sqlx::query(sql.as_str()).execute(pool).await?;

    Ok(())
}
