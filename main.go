package main

import (
    "net/http"
)

func main() {
    defer db.Close()

    http.HandleFunc("/", route)
    http.ListenAndServe(":8080", nil)
}
