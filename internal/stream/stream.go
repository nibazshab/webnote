package stream

import (
	"net/http"
	"regexp"
	"strings"

	"github.com/nibazshab/webnote/internal/log"
	"github.com/nibazshab/webnote/internal/net"
	"github.com/nibazshab/webnote/pkg/util"
)

var rePath = regexp.MustCompile(`^[a-zA-Z0-9]+$`)

func Stream(w http.ResponseWriter, req *http.Request) {
	idx := strings.TrimPrefix(req.URL.Path, "/")

	if rePath.MatchString(idx) && len(idx) < 17 {
		if req.Method == http.MethodPost {
			if !strings.HasPrefix(req.Header.Get("Content-Type"), "application/x-www-form-urlencoded") {
				w.Write([]byte("ERROR: content-type not application/x-www-form-urlencoded"))
				return
			}

			req.ParseForm()

			if req.PostForm.Has("t") {
				msg := net.RespPost(idx, req)
				log.Message(idx, msg, req)
			} else {
				w.Write([]byte("ERROR: body not 't'"))
			}
		} else {
			net.RespGet(idx, w, req)
		}
	} else {
		if req.Method == http.MethodGet {
			if idx == "style.css" || idx == "script.js" {
				net.AssetFile(idx, w)
			} else {
				http.Redirect(w, req, "/"+util.RandIdx(4), http.StatusFound)
			}
		} else {
			w.Write([]byte("ERROR: path illegal"))
		}
	}
}
