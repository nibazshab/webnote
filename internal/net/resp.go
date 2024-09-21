package net

import (
	"html/template"
	"io/fs"
	"net/http"

	"github.com/gin-gonic/gin"

	"github.com/nibazshab/webnote/web"
)

func webPage(c *gin.Context, id string, con *string) {
	template.Must(template.ParseFS(web.Web, "public/index.html")).Execute(c.Writer, struct {
		URL string
		CON *string
	}{
		URL: id,
		CON: con,
	})
}

func Static(g *gin.RouterGroup) {
	public, _ := fs.Sub(web.Web, "public/assets")
	g.StaticFS("/", http.FS(public))
}

func rawData(c *gin.Context, con *string) {
	c.Data(http.StatusOK, "text/plain; charset=utf-8", []byte(*con))
}
