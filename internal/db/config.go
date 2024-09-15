package db

import "github.com/nibazshab/webnote/internal/datapath"

func initSQL() string {
	return "CREATE TABLE IF NOT EXISTS webnote_data (id VARCHAR(16) PRIMARY KEY, con TEXT);"
}

func getDbPath() string {
	return datapath.GetDataFile("webnote.db")
}
