package db

import (
	"database/sql"
	"log"

	_ "github.com/mattn/go-sqlite3"
)

var db *sql.DB

func Init() {
	db, _ = sql.Open("sqlite3", getDbPath())

	if err := db.Ping(); err != nil {
		log.Fatalf("db connect error: %v", err)
	}

	db.Exec(initSQL())
}

func Close() {
	err := db.Close()
	if err != nil {
		log.Fatalf("db close error: %v", err)
	}
}
