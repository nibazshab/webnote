MUSLPKG="https://musl.cc/x86_64-linux-musl-cross.tgz"
MUSLLIB=musllib

CGO_ENABLED=1
FLAG="-s -w --extldflags '-static'"

GOOS=linux
GOARCH=amd64
CC=$(PWD)/$(MUSLLIB)/bin/x86_64-linux-musl-gcc

all: build

setupcc:
	if [ ! -d "$(MUSLLIB)" ]; then \
		mkdir -p $(MUSLLIB) && \
		wget -O $(MUSLLIB).tgz $(MUSLPKG) && \
		tar -zxvf $(MUSLLIB).tgz --strip-components=1 -C $(MUSLLIB) && \
		rm $(MUSLLIB).tgz; \
	fi

setupgolib:
	go mod tidy

build: setupcc setupgolib
	CGO_ENABLED=$(CGO_ENABLED) GOOS=$(GOOS) GOARCH=$(GOARCH) CC=$(CC) go build -ldflags=$(FLAG)

clean:
	rm -f webnote

cleanall:
	rm -rf $(MUSLLIB) webnote

.PHONY: all setupcc setupgolib build clean cleanall
