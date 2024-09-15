package flag

import "flag"

var Port *string

var _port = "10003"

func init() {
	Port = flag.String("port", _port, "server port")
	flag.Parse()
}
