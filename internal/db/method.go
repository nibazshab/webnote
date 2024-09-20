package db

func Insert(id string, con *string) {
	db.Save(&Data{ID: id, Con: *con})
}

func Delete(id string) {
	db.Where(Data{ID: id}).Delete(&Data{})
}

func Select(id string) *string {
	con := Data{ID: id}
	db.Where(con).First(&con)
	return &con.Con
}
