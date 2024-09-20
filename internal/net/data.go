package net

import "github.com/nibazshab/webnote/internal/db"

func del(id string) {
	db.Delete(id)
}

func ins(id string, con *string) {
	db.Insert(id, con)
}

func sel(id string) *string {
	con := db.Select(id)
	return con
}
