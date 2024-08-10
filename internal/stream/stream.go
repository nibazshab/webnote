package stream

import (
	"net/http"
	"regexp"
	"strings"

	"github.com/nibazshab/webnote/internal/log"
	"github.com/nibazshab/webnote/internal/net"
	"github.com/nibazshab/webnote/pkg/util"
)

var re = regexp.MustCompile(`^[a-zA-Z0-9]+$`)

func Stream(w http.ResponseWriter, r *http.Request) {
	idx := strings.TrimPrefix(r.URL.Path, "/")

	if re.MatchString(idx) && len(idx) < 17 {
		if r.Method == http.MethodPost {

			if !strings.HasPrefix(r.Header.Get("Content-Type"), "application/x-www-form-urlencoded") {
				w.Write([]byte("ERROR: content-type not application/x-www-form-urlencoded"))

				return
			}

			r.ParseForm()

			if r.PostForm.Has("t") {
				msg := net.HttpPost(idx, r)

				log.Message(idx, msg, r)

			} else {
				w.Write([]byte("ERROR: body not 't'"))
			}
		} else {
			net.HttpGet(idx, w, r)
		}
	} else {
		if r.Method == http.MethodGet {
			if idx == "style.css" || idx == "script.js" {
				net.AssetFile(idx, w)
			} else {
				http.Redirect(w, r, "/"+util.RandString(4), http.StatusFound)
			}
		} else {
			w.Write([]byte("ERROR: path illegal"))
		}
	}
}
