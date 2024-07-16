package main

import (
    "net/http"
    "regexp"
    "strings"
)

func route(w http.ResponseWriter, r *http.Request) {
    id := strings.TrimPrefix(r.URL.Path, "/")
    if regexp.MustCompile(`^[a-zA-Z0-9]+$`).MatchString(id) && len(id) < 17 {
        ua := r.Header.Get("user-agent")

        if r.Method == http.MethodPost {
            if !strings.HasPrefix(r.Header.Get("Content-Type"), "application/x-www-form-urlencoded") {
                w.Write([]byte("ERROR: content-type not application/x-www-form-urlencoded"))
                return
            }

            r.ParseForm()
            if r.PostForm.Has("t") {
                do := write_data(r, id)
                logger(r, id, ua, do)
            } else {
                w.Write([]byte("ERROR: body should be - t"))
            }
        } else {
            show_data(w, r, id, ua)
        }
    } else {
        if r.Method == http.MethodGet {
            http.Redirect(w, r, "/"+rand_string(), http.StatusFound)
        } else {
            w.Write([]byte("ERROR: path should be <= 16"))
        }
    }
}
