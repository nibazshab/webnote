package log

import (
	"io"
	"log"
	"net/http"
	"os"

	"webnote/pkg/util"
)

var log_file string

func Init() {
	if log_file == "" {
		log_file = GetLogFile()
	}

	f, err := os.OpenFile(log_file, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		log.Fatalf("log.log error: %v", err)
	}
	defer f.Close()
}

func Message(idx string, msg string, r *http.Request) {
	f, _ := os.OpenFile(log_file, os.O_APPEND|os.O_WRONLY, 0o644)
	defer f.Close()

	multiWriter := io.MultiWriter(os.Stdout, f)
	log.SetOutput(multiWriter)

	log.Print(idx + " | " + msg + " | " + util.GetIP(r) + " | " + util.GetUA(r))
}

func Fatalf(msg string, err error) {
	log.Fatalf(msg, err)
}
