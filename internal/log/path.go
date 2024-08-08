package log

import (
	"path/filepath"

	"github.com/nibazshab/webnote/internal/dir"
)

func GetLogFile() string {
	return filepath.Join(dir.Init(), "log.log")
}
