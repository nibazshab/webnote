package main

import (
    "embed"
    "html/template"
    "net/http"
    "regexp"
)

func write_data(r *http.Request, id string) int {
    con := r.PostFormValue("t")

    if con == "" {
        db.Exec("DELETE FROM webnote_data WHERE id = ?", id)
        return 0
    } else {
        db.Exec("INSERT OR REPLACE INTO webnote_data (id, text) VALUES (?, ?)", id, con)
        return 1
    }
}

//go:embed templates/index.html
var assets embed.FS

func show_data(w http.ResponseWriter, r *http.Request, id string, ua string) {
    var con string
    db.QueryRow("SELECT text FROM webnote_data WHERE id = ?", id).Scan(&con)

    if regexp.MustCompile(`^(curl|Wget).*`).MatchString(ua) || r.URL.Query().Has("raw") {
        w.Header().Set("Content-type", "text/plain; charset=utf-8")
        w.Write([]byte(con))
    } else {
        template.Must(template.ParseFS(assets, "templates/index.html")).Execute(w, struct {
            URL string
            CON string
        }{
            URL: id,
            CON: con,
        })
    }
}
