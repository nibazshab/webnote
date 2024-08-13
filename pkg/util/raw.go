package util

import (
	"net/http"
	"regexp"
)

var re = regexp.MustCompile(`^(curl|Wget)`)

func IsReqRaw(req *http.Request) bool {
	return re.MatchString(GetUserUA(req)) || req.URL.Query().Has("raw")
}
