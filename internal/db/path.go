package db

import (
	"path/filepath"

	"webnote/internal/dir"
)

func GetDbFile() string {
	return filepath.Join(dir.Init(), "webnote.db")
}
