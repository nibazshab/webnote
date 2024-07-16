package main

import (
    "net/http"
)

func main() {
    defer db.Close()

    http.HandleFunc("/", route)
    http.ListenAndServe(":10003", nil)
}
