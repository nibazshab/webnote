package flag

import "flag"

var Port *string

func init() {
	Port = flag.String("port", "10003", "server port")
	flag.Parse()
}
