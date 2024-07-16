package main

import (
    "log"
    "net/http"
    "strconv"
)

func logger(r *http.Request, id string, ua string, do int) {
    ip := r.Header.Get("X-Forwarded-For")

    if ip == "" {
        ip = r.RemoteAddr
    }

    log.Print(id + " - " + strconv.Itoa(do) + " - " + ip + " - " + ua)
}
