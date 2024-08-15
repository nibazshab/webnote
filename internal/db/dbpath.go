package db

import "github.com/nibazshab/webnote/internal/datapath"

func getDbPath() string {
	return datapath.GetDataFile("webnote.db")
}
