package cmd

import (
	"net/http"

	"github.com/nibazshab/webnote/internal/db"
	"github.com/nibazshab/webnote/internal/log"
	"github.com/nibazshab/webnote/internal/stream"
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
