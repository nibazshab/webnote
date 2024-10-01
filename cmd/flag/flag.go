package flag

import "flag"

var Port *string

const port = "10003"

func init() {
	Port = flag.String("port", port, "server port")
	flag.Parse()
}
