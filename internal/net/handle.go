package net

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func HandleGet(c *gin.Context, id string) {
	con := new(string)
	sel(id, con)

	if reqRawCheck(c) {
		respRawData(c, con)
	} else {
		respWebPage(c, id, con)
	}
}

func HandlePost(c *gin.Context, id string) rune {
	if !conTypeCheck(c) {
		c.String(http.StatusBadRequest, "ERROR: content-type not application/x-www-form-urlencoded")
		return 'e'
	}

	con, i := c.GetPostForm("t")

	if i {
		if con == "" {
			del(id)
			return 'd'
		} else {
			ins(id, &con)
			return 'i'
		}
	} else {
		c.String(http.StatusBadRequest, "ERROR: body not t")
		return 'e'
	}
}
