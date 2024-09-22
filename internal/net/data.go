package net

import "github.com/nibazshab/webnote/internal/db"

func del(id string) {
	hid := convHashId(id)
	db.Delete(hid)
}

func ins(id string, con *string) {
	hid := convHashId(id)
	db.Insert(hid, con)
}

func sel(id string) *string {
	hid := convHashId(id)
	con := db.Select(hid)
	return con
}
