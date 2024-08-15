package datapath

import (
	"os"
	"path/filepath"
)

var dataPath string

func GetDataFile(file string) string {
	if dataPath == "" {
		ex, _ := os.Executable()
		dataPath = filepath.Join(filepath.Dir(ex), "data")

		if _, err := os.Stat(dataPath); os.IsNotExist(err) {
			os.MkdirAll(dataPath, 0o755)
		}
	}

	datafile := filepath.Join(dataPath, file)
	return datafile
}
