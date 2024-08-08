package dir

import (
	"os"
	"path/filepath"
)

var data_dir string

func Init() string {
	if data_dir == "" {
		ex, _ := os.Executable()
		data_dir = filepath.Join(filepath.Dir(ex), "data")

		if _, err := os.Stat(data_dir); os.IsNotExist(err) {
			os.MkdirAll(data_dir, 0o755)
		}
	}

	return data_dir
}
