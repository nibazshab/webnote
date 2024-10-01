package db

const (
	dbFileName  = "database.sqlite"
	tablePrefix = "webnote_"
)

type Data struct {
	ID  uint32
	Con string
}
