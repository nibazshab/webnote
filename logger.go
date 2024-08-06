package main

import (
    "io"
    "log"
    "net/http"
    "os"
    "strconv"
)

func log_init() {
    f, err := os.Create(log_)
    if err != nil {
        log.Fatalf("log.log error: %v", err)
    }
    defer f.Close()
}

func logging(r *http.Request, id string, ua string, do int) {
    ip := r.Header.Get("X-Forwarded-For")
    if ip == "" {
        ip = r.RemoteAddr
    }

    f, _ := os.OpenFile(log_, os.O_APPEND|os.O_RDWR, os.ModePerm)
    defer f.Close()

    multiWriter := io.MultiWriter(os.Stdout, f)
    log.SetOutput(multiWriter)

    log.Print(id + " - " + strconv.Itoa(do) + " - " + ip + " - " + ua)
}
