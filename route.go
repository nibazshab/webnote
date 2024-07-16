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
            r.ParseForm()

            if r.PostForm.Has("t") {
                do := write_data(r, id)

                logger(r, id, ua, do)
            }

        } else {
            show_data(w, r, id, ua)
        }
    } else {
        if r.Method == http.MethodGet {
            http.Redirect(w, r, "/"+rand_string(), http.StatusFound)
        }
    }
}
