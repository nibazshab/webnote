package util

import (
	"net/http"
)

func GetIP(r *http.Request) string {
	ip := r.Header.Get("X-Forwarded-For")

	if ip == "" {
		ip = r.RemoteAddr
	}

	return ip
}
