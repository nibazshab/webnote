package db

import (
	"log"

	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
	"gorm.io/gorm/schema"
)

var db *gorm.DB

func Init() {
	var err error

	db, err = gorm.Open(sqlite.Open(dbFile+"?_journal=WAL&_vacuum=incremental"), &gorm.Config{
		NamingStrategy: schema.NamingStrategy{
			TablePrefix: tablePrefix,
		},
		Logger: logger.Default.LogMode(logger.Silent),
	})
	if err != nil {
		log.Fatalf("db connect error: %v", err)
	}

	err = db.AutoMigrate(&Data{})
	if err != nil {
		log.Fatalf("db init error: %v", err)
	}
}

func Close() {
	dB, _ := db.DB()
	err := dB.Close()
	if err != nil {
		log.Fatalf("db close error: %v", err)
	}
}
