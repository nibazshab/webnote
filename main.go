package main

import (
    "net/http"
)

func main() {
    http.HandleFunc("/", route)
    http.ListenAndServe(":10003", nil)
}
