package net

import (
	"html/template"
	"net/http"

	"github.com/nibazshab/webnote/internal/db"
	"github.com/nibazshab/webnote/pkg/util"
	"github.com/nibazshab/webnote/web"
)

func HttpGetPage(idx string, con *string, w http.ResponseWriter) {
	template.Must(template.ParseFS(web.Web, "public/index.html")).Execute(w, struct {
		URL string
		CON *string
	}{
		URL: idx,
		CON: con,
	})
}

func HttpGetRaw(con *string, w http.ResponseWriter) {
	w.Header().Set("Content-type", "text/plain; charset=utf-8")
	w.Write([]byte(*con))
}

func HttpGet(idx string, w http.ResponseWriter, r *http.Request) {
	db := db.GetDb()

	var con string
	db.QueryRow("SELECT text FROM webnote_data WHERE id = ?", idx).Scan(&con)

	if util.UACheck(r) {
		HttpGetRaw(&con, w)
	} else {
		HttpGetPage(idx, &con, w)
	}
}

func AssetFile(idx string, w http.ResponseWriter) {
	idx = "public/" + idx

	data, _ := web.Web.ReadFile(idx)

	w.Write(data)
}
