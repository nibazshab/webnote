package db

func Insert(id uint32, con *string) {
	db.Save(&Data{ID: id, Con: *con})
}

func Delete(id uint32) {
	db.Where(Data{ID: id}).Delete(&Data{})
}

func Select(id uint32) *string {
	con := Data{ID: id}
	db.Where(con).First(&con)
	return &con.Con
}
