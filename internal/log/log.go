package log

import (
	"io"
	"log"
	"net/http"
	"os"

	"github.com/nibazshab/webnote/pkg/util"
)

var logFile string

func Init() {
	if logFile == "" {
		logFile = getLogPath()
	}

	f, err := os.OpenFile(logFile, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		log.Fatalf("log.log error: %v", err)
	}
	defer f.Close()
}

func Message(idx string, msg string, req *http.Request) {
	f, _ := os.OpenFile(logFile, os.O_APPEND|os.O_WRONLY, 0o644)
	defer f.Close()

	multiWriter := io.MultiWriter(os.Stdout, f)
	log.SetOutput(multiWriter)
	log.Print(idx + " | " + msg + " | " + util.GetUserIP(req) + " | " + util.GetUserUA(req))
}

func Fatalf(msg string, err error) {
	log.Fatalf(msg, err)
}
