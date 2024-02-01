package main

import (
    "embed"
    "html/template"
    "log"
    "math/rand"
    "net/http"
    "os"
    "path/filepath"
    "regexp"
    "strings"
    "time"
)

//go:embed index.html
var web embed.FS

const AppData = "tmp"

func main() {
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        p := strings.TrimPrefix(r.URL.Path, "/")
        if regexp.MustCompile(`^[a-zA-Z0-9]+$`).MatchString(p) && len(p) < 16 {
            ua := r.Header.Get("user-agent")
            f := filepath.Join(AppData, p)
            if r.Method == http.MethodPost {
                handlePost(w, r, f, p, ua)
            } else {
                handleGet(w, r, f, p, ua)
            }
        } else {
            if r.Method == http.MethodGet {
                illegalPath(w, r)
            }
        }
    })
    http.ListenAndServe(":10003", nil)
}

func illegalPath(w http.ResponseWriter, r *http.Request) {
    a := "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
    var p string
    for i := 0; i < 4; i++ {
        p += string(a[rand.New(rand.NewSource(time.Now().UnixNano())).Intn(len(a))])
    }
    http.Redirect(w, r, "/"+p, http.StatusFound)
}

func handlePost(w http.ResponseWriter, r *http.Request, f string, p string, ua string) {
    r.ParseForm()
    if r.PostForm.Has("t") {
        t := r.PostFormValue("t")
        var d string
        if t == "" {
            os.Remove(f)
            d = "DELETE"
        } else {
            os.WriteFile(f, []byte(t), 0666)
            d = "UPDATE"
        }
        xff := r.Header.Get("X-Forwarded-For")
        if xff == "" {
            xff = r.RemoteAddr
        } else {
            xff = xff[:strings.LastIndex(xff, ":")]
        }
        log.Print(xff + " - " + p + " - " + d + " - " + ua)
    }
}

func handleGet(w http.ResponseWriter, r *http.Request, f string, p string, ua string) {
    t, _ := os.ReadFile(f)
    if regexp.MustCompile(`^(curl|Wget).*`).MatchString(ua) || r.URL.Query().Has("raw") {
        w.Header().Set("Content-type", "text/plain; charset=UTF-8")
        w.Write(t)
    } else {
        template.Must(template.ParseFS(web, "index.html")).Execute(w, struct {
            P string
            T string
        }{
            P: p,
            T: string(t),
        })
    }
}
