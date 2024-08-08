package net

import (
	"net/http"

	"webnote/internal/db"
)

func HttpPostIns(idx string, con string) {
	db := db.GetDb()
	db.Exec("INSERT OR REPLACE INTO webnote_data (id, text) VALUES (?, ?)", idx, con)
}

func HttpPostDel(idx string) {
	db := db.GetDb()
	db.Exec("DELETE FROM webnote_data WHERE id = ?", idx)
}

func HttpPost(idx string, r *http.Request) string {
	con := r.PostFormValue("t")

	if con == "" {
		HttpPostDel(idx)

		return "del"

	} else {
		HttpPostIns(idx, con)

		return "ins"
	}
}
