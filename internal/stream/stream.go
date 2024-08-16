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
			msg := net.RespPost(idx, w, req)
			if msg != "" {
				log.Message(idx, msg, req)
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
