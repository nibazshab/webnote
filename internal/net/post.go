package net

import (
	"net/http"
	"strings"

	"github.com/nibazshab/webnote/internal/db"
)

func RespPost(idx string, w http.ResponseWriter, req *http.Request) (string, string) {
	if !strings.HasPrefix(req.Header.Get("Content-Type"), "application/x-www-form-urlencoded") {
		w.Write([]byte("ERROR: content-type not application/x-www-form-urlencoded"))
		return "", ""
	}

	req.ParseForm()

	var msg string
	if req.PostForm.Has("t") {
		con := req.PostFormValue("t")
		if con == "" {
			db.Delete(idx)
			msg = "del"
		} else {
			db.Insert(idx, &con)
			msg = "ins"
		}
	} else {
		w.Write([]byte("ERROR: body not 't'"))
		return "", ""
	}

	return idx, msg
}
