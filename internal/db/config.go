package db

import "github.com/nibazshab/webnote/internal/datapath"

type Data struct {
	ID  uint32
	Con string
}

var tablePrefix = "webnote_"

var dbFile = datapath.GetDataFile("database.sqlite")
