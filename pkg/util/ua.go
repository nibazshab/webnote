package util

import (
	"net/http"
	"regexp"
)

var re = regexp.MustCompile(`^(curl|Wget)`)

func GetUA(r *http.Request) string {
	return r.Header.Get("user-agent")
}

func UACheck(r *http.Request) bool {
	return re.MatchString(GetUA(r)) || r.URL.Query().Has("raw")
}
