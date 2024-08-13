package log

import (
	"path/filepath"

	"github.com/nibazshab/webnote/internal/dir"
)

func LogFilePath() string {
	return filepath.Join(dir.Init(), "log.log")
}
