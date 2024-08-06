package main

import (
    "database/sql"
    "log"

    _ "github.com/mattn/go-sqlite3"
)

var db *sql.DB

func db_init() {
    db, _ = sql.Open("sqlite3", db_)

    if err := db.Ping(); err != nil {
        log.Fatalf("database connect error: %v", err)
    }

    create_table := `
    CREATE TABLE IF NOT EXISTS webnote_data (
        id VARCHAR(16) PRIMARY KEY,
        text TEXT
    );`

    db.Exec(create_table)
}
