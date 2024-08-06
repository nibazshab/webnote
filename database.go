package main

import (
    "database/sql"
    "log"
    "os"
    "path/filepath"

    _ "github.com/mattn/go-sqlite3"
)

var db *sql.DB

func init() {
    ex, _ := os.Executable()
    db_dir := filepath.Join(filepath.Dir(ex), "data")

    if _, err := os.Stat(db_dir); os.IsNotExist(err) {
        os.MkdirAll(db_dir, os.ModePerm)
    }

    db, _ = sql.Open("sqlite3", filepath.Join(db_dir, "webnote.db"))

    if err := db.Ping(); err != nil {
        log.Fatalf("database error: %v", err)
    }

    create_table := `
    CREATE TABLE IF NOT EXISTS webnote_data (
        id VARCHAR(16) PRIMARY KEY,
        text TEXT
    );`

    db.Exec(create_table)
}
