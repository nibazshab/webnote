package router

import (
	"regexp"
)

var regexPath = regexp.MustCompile(`^[a-zA-Z0-9]+$`)

func urlPathCheck(id string) bool {
	return regexPath.MatchString(id) && len(id) < 17
}
