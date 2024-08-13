package net

import (
	"net/http"

	"github.com/nibazshab/webnote/web"
)

func AssetFile(idx string, w http.ResponseWriter) {
	idx = "public/" + idx
	data, _ := web.Web.ReadFile(idx)

	w.Write(data)
}
