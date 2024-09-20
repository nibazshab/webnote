package db

import "github.com/nibazshab/webnote/internal/datapath"

type Data struct {
	ID  string `gorm:"type:char(16);primaryKey"`
	Con string
}

var tablePrefix = "webnote_"

var dbFile = datapath.GetDataFile("webnote.db")
