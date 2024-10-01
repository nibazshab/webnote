package log

import (
	"io"
	"log"
	"os"

	"github.com/gin-gonic/gin"

	"github.com/nibazshab/webnote/pkg/util"
)

func Init() {
	file, err := os.OpenFile(logFile, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		log.Fatalf("log open error: %v", err)
	}
	defer file.Close()
}

func Logging(c *gin.Context, id string, msg rune) {
	file, _ := os.OpenFile(logFile, os.O_APPEND|os.O_WRONLY, 0o644)
	defer file.Close()

	multiWriter := io.MultiWriter(os.Stdout, file)
	log.SetOutput(multiWriter)
	log.Printf("%s | %c | %s | %s", id, msg, util.GetUserIP(c.Request), util.GetUserUA(c.Request))
}

func Fatalf(format string, v ...any) {
	log.Fatalf(format, v...)
}
