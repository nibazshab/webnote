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
    "unsafe"
)

//go:embed index.html
var web embed.FS

const app_data = "tmp"

func main() {
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        p := strings.TrimPrefix(r.URL.Path, "/")
        if regexp.MustCompile(`^[a-zA-Z0-9]+$`).MatchString(p) && len(p) < 16 {
            ua := r.Header.Get("user-agent")
            f := filepath.Join(app_data, p)
            if r.Method == http.MethodPost {
                r.ParseForm()
                if r.PostForm.Has("t") {
                    d := RequestPost(r, f)
                    Log(r, p, d, ua)
                }
            } else {
                RequestGet(w, r, f, p, ua)
            }
        } else {
            if r.Method == http.MethodGet {
                RedirectPath(w, r)
            }
        }
    })
    http.ListenAndServe(":10003", nil)
}

func RequestPost(r *http.Request, f string) string {
    var d string
    t := r.PostFormValue("t")
    if t == "" {
        os.Remove(f)
        d = "DELETE"
    } else {
        os.WriteFile(f, []byte(t), 0o666)
        d = "UPDATE"
    }
    return d
}

func Log(r *http.Request, p string, d string, ua string) {
    xff := r.Header.Get("X-Forwarded-For")
    if xff == "" {
        xff = r.RemoteAddr
    }
    log.Print(xff + " - " + p + " - " + d + " - " + ua)
}

func RequestGet(w http.ResponseWriter, r *http.Request, f string, p string, ua string) {
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

func RedirectPath(w http.ResponseWriter, r *http.Request) {
    http.Redirect(w, r, "/"+RandString(), http.StatusFound)
}

const letter_bytes = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
const (
    letter_idx_bits = 6
    letter_idx_mask = 1<<letter_idx_bits - 1
    letter_idx_max  = 63 / letter_idx_bits
)

var src = rand.NewSource(time.Now().UnixNano())

func RandString() string {
    b := make([]byte, 4)
    for i, cache, remain := 3, src.Int63(), letter_idx_max; i >= 0; {
        if remain == 0 {
            cache, remain = src.Int63(), letter_idx_max
        }
        if idx := int(cache & letter_idx_mask); idx < len(letter_bytes) {
            b[i] = letter_bytes[idx]
            i--
        }
        cache >>= letter_idx_bits
        remain--
    }
    return *(*string)(unsafe.Pointer(&b))
}
