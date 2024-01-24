package main

import (
    "embed"
    "html/template"
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
const tmpDir = "tmp"

func main() {
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        p := strings.TrimPrefix(r.URL.Path, "/")
        if !regexp.MustCompile(`^[a-zA-Z0-9]+$`).MatchString(p) || len(p) > 16 {
            handleIllegalPath(w, r)
        } else {
            f := filepath.Join(tmpDir, p)
            if r.Method == http.MethodPost {
                handlePost(w, r, f)
            } else {
                handleGet(w, r, p, f)
            }
        }
    })
    http.ListenAndServe(":10003", nil)
}

func handleIllegalPath(w http.ResponseWriter, r *http.Request) {
    if r.Method == http.MethodPost {
    } else {
        a := "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        var p string
        for i := 0; i < 5; i++ {
            p += string(a[rand.New(rand.NewSource(time.Now().UnixNano())).Intn(len(a))])
        }
        http.Redirect(w, r, "/"+p, http.StatusFound)
    }
}

func handlePost(w http.ResponseWriter, r *http.Request, f string) {
    r.ParseForm()
    if !r.PostForm.Has("t") {
    } else {
        t := r.PostFormValue("t")
        if t == "" {
            os.Remove(f)
        } else {
            os.WriteFile(f, []byte(t), 0666)
        }
    }
}

func handleGet(w http.ResponseWriter, r *http.Request, p string, f string) {
    t, _ := os.ReadFile(f)
    ua := r.Header.Get("user-agent")
    if strings.HasPrefix(ua, "curl") || strings.HasPrefix(ua, "Wget") || r.URL.Query().Has("raw") {
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
