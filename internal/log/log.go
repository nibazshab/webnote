package log

import (
	"io"
	"log"
	"os"

	"github.com/gin-gonic/gin"

	"github.com/nibazshab/webnote/pkg/util"
)

func Init() {
	f, err := os.OpenFile(logFile, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		log.Fatalf("log open error: %v", err)
	}
	defer f.Close()
}

func Logging(c *gin.Context, id string, handle rune) {
	f, _ := os.OpenFile(logFile, os.O_APPEND|os.O_WRONLY, 0o644)
	defer f.Close()

	multiWriter := io.MultiWriter(os.Stdout, f)
	log.SetOutput(multiWriter)
	log.Print(id + " | " + string(handle) + " | " + util.GetUserIP(c.Request) + " | " + util.GetUserUA(c.Request))
}

func Fatalf(format string, v ...any) {
	log.Fatalf(format, v...)
}
