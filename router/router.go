package router

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"github.com/nibazshab/webnote/internal/log"
	"github.com/nibazshab/webnote/internal/net"
	"github.com/nibazshab/webnote/pkg/util"
)

func Router(r *gin.Engine) {
	s := r.Group("/")
	{
		g := r.Group("/assets")
		{
			net.Static(g)
		}

		s.GET("/favicon.ico", func(c *gin.Context) {
			c.Data(http.StatusOK, "image/x-icon", []byte{})
		})

		s.GET("/", redirect)
		s.GET("/:id", get)
		s.POST("/:id", post)
	}
}

func get(c *gin.Context) {
	id := c.Param("id")

	if urlPathCheck(id) {
		net.HandleGet(c, id)
	} else {
		redirect(c)
	}
}

func post(c *gin.Context) {
	id := c.Param("id")

	if urlPathCheck(id) {
		m := net.HandlePost(c, id)
		if m != 'e' {
			log.Logging(c, id, m)
		}
	} else {
		c.String(http.StatusBadRequest, "ERROR: invalid path")
	}
}

func redirect(c *gin.Context) {
	c.Redirect(http.StatusFound, "/"+util.RandStr(4))
}
