package net

import (
	"net/http"

	"github.com/nibazshab/webnote/internal/db"
)

func RespPost(idx string, req *http.Request) string {
	con := req.PostFormValue("t")
	if con == "" {
		db.Delete(idx)
		return "del"
	} else {
		db.Insert(idx, &con)
		return "ins"
	}
}
