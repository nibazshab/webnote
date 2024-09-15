package db

func Insert(id string, con *string) {
	db.Exec("INSERT OR REPLACE INTO webnote_data (id, con) VALUES (?, ?)", id, *con)
}

func Delete(id string) {
	db.Exec("DELETE FROM webnote_data WHERE id = ?", id)
}

func Select(id string, con *string) {
	db.QueryRow("SELECT con FROM webnote_data WHERE id = ?", id).Scan(con)
}
