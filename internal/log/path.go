package log

import "github.com/nibazshab/webnote/internal/datapath"

func getLogPath() string {
	return datapath.GetDataFile("log.log")
}
