package dir

import (
	"os"
	"path/filepath"
)

var dataDir string

func Init() string {
	if dataDir == "" {
		exe, _ := os.Executable()
		dataDir = filepath.Join(filepath.Dir(exe), "data")

		if _, err := os.Stat(dataDir); os.IsNotExist(err) {
			os.MkdirAll(dataDir, 0o755)
		}
	}
	return dataDir
}
