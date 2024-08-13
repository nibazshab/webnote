package db

func Insert(idx string, con *string) {
	db.Exec("INSERT OR REPLACE INTO webnote_data (id, text) VALUES (?, ?)", idx, *con)
}

func Delete(idx string) {
	db.Exec("DELETE FROM webnote_data WHERE id = ?", idx)
}

func Select(idx string, con *string) {
	db.QueryRow("SELECT text FROM webnote_data WHERE id = ?", idx).Scan(con)
}
