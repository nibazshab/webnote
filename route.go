package main

import (
    "net/http"
    "os"
    "path/filepath"
    "regexp"
    "strings"
)

var (
    db_  string
    log_ string
)

func init() {
    ex, _ := os.Executable()
    datadir := filepath.Join(filepath.Dir(ex), "data")

    if _, err := os.Stat(datadir); os.IsNotExist(err) {
        os.MkdirAll(datadir, os.ModePerm)
    }

    db_ = filepath.Join(datadir, "webnote.db")
    log_ = filepath.Join(datadir, "log.log")

    db_init()
    log_init()
}

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
                logging(r, id, ua, do)
            } else {
                w.Write([]byte("ERROR: body not 't'"))
            }
        } else {
            show_data(w, r, id, ua)
        }
    } else {
        if r.Method == http.MethodGet {
            http.Redirect(w, r, "/"+rand_string(), http.StatusFound)
        } else {
            w.Write([]byte("ERROR: path length more than 16"))
        }
    }
}
