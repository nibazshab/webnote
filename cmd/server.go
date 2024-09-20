package cmd

import (
	"os"
	"os/signal"
	"syscall"

	"github.com/gin-gonic/gin"

	"github.com/nibazshab/webnote/cmd/flag"
	"github.com/nibazshab/webnote/internal/db"
	"github.com/nibazshab/webnote/internal/log"
	"github.com/nibazshab/webnote/router"
)

func init() {
	db.Init()
	log.Init()
}

func Start() {
	ch := make(chan os.Signal, 1)
	signal.Notify(ch, syscall.SIGINT, syscall.SIGTERM)

	defer db.Close()

	gin.SetMode(gin.ReleaseMode)
	r := gin.New()

	router.Router(r)

	go func() {
		if err := r.Run(":" + *flag.Port); err != nil {
			log.Fatalf("start error: %v", err)
		}
	}()

	<-ch
}
