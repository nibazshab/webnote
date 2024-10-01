package handle

import "github.com/nibazshab/webnote/internal/db"

func writeDbData(id *string, con *string) {
	var writeData db.DataMethod

	writeData = &db.Data{
		ID:  convHashId(id),
		Con: *con,
	}
	writeData.WriteData()
}

func deleteDbData(id *string) {
	var dataDelId db.DataMethod

	dataDelId = &db.Data{
		ID: convHashId(id),
	}
	dataDelId.DeleteData()
}

func getDbData(id *string) *string {
	var dataGetId db.DataMethod

	dataGetId = &db.Data{
		ID: convHashId(id),
	}
	getData := dataGetId.GetData()

	return &getData.Con
}
