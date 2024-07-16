package main

import (
    "database/sql"

    _ "github.com/mattn/go-sqlite3"
)

var db *sql.DB

func init() {
    db, _ = sql.Open("sqlite3", "./data.db")

    create_table := `
    CREATE TABLE IF NOT EXISTS webnote_data (
        id VARCHAR(16) PRIMARY KEY,
        text TEXT
    );`

    db.Exec(create_table)
}
