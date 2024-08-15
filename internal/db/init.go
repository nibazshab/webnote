package db

func initSQL() string {
	return "CREATE TABLE IF NOT EXISTS webnote_data (id VARCHAR(16) PRIMARY KEY, text TEXT);"
}
