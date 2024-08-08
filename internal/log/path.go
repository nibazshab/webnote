package log

import (
	"path/filepath"

	"webnote/internal/dir"
)

func GetLogFile() string {
	return filepath.Join(dir.Init(), "log.log")
}
