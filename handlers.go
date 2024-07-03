package main

import (
    "embed"
    "html/template"
    "net/http"
    "os"
    "regexp"
)

//go:embed templates/index.html
var assets embed.FS

const app_data = "tmp"

func upload(r *http.Request, file string) string {
    var option string
    upload_text := r.PostFormValue("t")

    if upload_text == "" {
        os.Remove(file)
        option = "DELETE"
    } else {
        os.WriteFile(file, []byte(upload_text), 0o666)
        option = "UPDATE"
    }

    return option
}

func get_text(w http.ResponseWriter, r *http.Request, file string, url_path string, client_ua string) {
    text_raw, _ := os.ReadFile(file)

    if regexp.MustCompile(`^(curl|Wget).*`).MatchString(client_ua) || r.URL.Query().Has("raw") {
        w.Header().Set("Content-type", "text/plain; charset=UTF-8")
        w.Write(text_raw)
    } else {
        template.Must(template.ParseFS(assets, "templates/index.html")).Execute(w, struct {
            GET_URL_PATH string
            GET_TEXT_RAW string
        }{
            GET_URL_PATH: url_path,
            GET_TEXT_RAW: string(text_raw),
        })
    }
}
