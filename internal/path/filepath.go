package path

import (
	"os"
	"path/filepath"
)

var dataPath string

const dataDir = "webnote_data"

func GetFilePath(filename string) string {
	if dataPath == "" {
		exePath, _ := os.Executable()
		dataPath = filepath.Join(filepath.Dir(exePath), dataDir)

		if _, err := os.Stat(dataPath); os.IsNotExist(err) {
			os.MkdirAll(dataPath, 0o755)
		}
	}

	filePath := filepath.Join(dataPath, filename)
	return filePath
}
