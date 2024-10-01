package log

import "github.com/nibazshab/webnote/internal/path"

const logFileName = "log.log"

var logFile = path.GetFilePath(logFileName)
