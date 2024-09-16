package net

import (
	"io/fs"
	"net/http"

	"github.com/gin-gonic/gin"

	"github.com/nibazshab/webnote/web"
)

func Static(g *gin.RouterGroup) {
	public, _ := fs.Sub(web.Web, "public/assets")

	g.StaticFS("/", http.FS(public))
}
