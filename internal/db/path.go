package db

import (
	"path/filepath"

	"github.com/nibazshab/webnote/internal/dir"
)

func GetDbFile() string {
	return filepath.Join(dir.Init(), "webnote.db")
}
