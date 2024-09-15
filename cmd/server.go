package cmd

import (
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
	defer db.Close()

	gin.SetMode(gin.ReleaseMode)
	r := gin.New()

	router.Router(r)

	if err := r.Run(":" + *flag.Port); err != nil {
		log.Fatalf("start error: %v", err)
	}
}
