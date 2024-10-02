package path

import (
	"log"
	"os"
	"path/filepath"
)

var dataPath string

const dataDir = "webnote_data"

func GetFilePath(filename string) string {
	if dataPath == "" {
		exePath, err := os.Executable()
		if err != nil {
			log.Fatalf("data path error: %v", err)
		}

		dataPath = filepath.Join(filepath.Dir(exePath), dataDir)

		if _, err = os.Stat(dataPath); os.IsNotExist(err) {
			err = os.MkdirAll(dataPath, 0o755)
			if err != nil {
				log.Fatalf("data path create error: %v", err)
			}
		}
	}

	filePath := filepath.Join(dataPath, filename)
	return filePath
}
