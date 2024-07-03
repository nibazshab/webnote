package main

import (
    "net/http"
    "path/filepath"
    "regexp"
    "strings"
)

func route(w http.ResponseWriter, r *http.Request) {
    url_path := strings.TrimPrefix(r.URL.Path, "/")
    if regexp.MustCompile(`^[a-zA-Z0-9]+$`).MatchString(url_path) && len(url_path) < 16 {
        client_ua := r.Header.Get("user-agent")
        file := filepath.Join(app_data, url_path)
        if r.Method == http.MethodPost {
            r.ParseForm()
            if r.PostForm.Has("t") {
                operation := upload(r, file)
                logging(r, url_path, operation, client_ua)
            }
        } else {
            get_text(w, r, file, url_path, client_ua)
        }
    } else {
        if r.Method == http.MethodGet {
            http.Redirect(w, r, "/"+rand_string(), http.StatusFound)
        }
    }
}
