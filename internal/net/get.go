package net

import (
	"html/template"
	"net/http"

	"github.com/nibazshab/webnote/internal/db"
	"github.com/nibazshab/webnote/pkg/util"
	"github.com/nibazshab/webnote/web"
)

func respWebPage(idx string, con *string, w http.ResponseWriter) {
	template.Must(template.ParseFS(web.Web, "public/index.html")).Execute(w, struct {
		URL string
		CON *string
	}{
		URL: idx,
		CON: con,
	})
}

func respRawData(con *string, w http.ResponseWriter) {
	w.Header().Set("Content-type", "text/plain; charset=utf-8")
	w.Write([]byte(*con))
}

func RespGet(idx string, w http.ResponseWriter, req *http.Request) {
	con := new(string)
	db.Select(idx, con)

	if util.IsReqRaw(req) {
		respRawData(con, w)
	} else {
		respWebPage(idx, con, w)
	}
}
