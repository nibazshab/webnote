package cmd

import (
	"net/http"

	"webnote/internal/db"
	"webnote/internal/log"
	"webnote/internal/stream"
)

func init() {
	db.Init()
	log.Init()
}

func Start() {
	defer db.Close()

	http.HandleFunc("/", stream.Stream)

	if err := http.ListenAndServe(":10003", nil); err != nil {
		log.Fatalf("http start error: %v", err)
	}
}
