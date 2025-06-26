MUSL_LINK=https://musl.cc/x86_64-linux-musl-cross.tgz
MUSL_DIR=muslgcc
CC="$(CURDIR)/$(MUSL_DIR)/bin/x86_64-linux-musl-gcc"


FLAG="-s -w --extldflags '-static' \
 -X github.com/nibazshab/webnote/cmd/flag.Version=$(VERSION)"

CGO_ENABLED=1
GOOS=linux
GOARCH=amd64

VERSION=$(shell git describe --abbrev=0 --tags)

all: build

deps:
	go mod tidy

$(CC):
	mkdir -p $(MUSL_DIR)
	wget -O $(MUSL_DIR).tgz $(MUSL_LINK)
	tar -zxf $(MUSL_DIR).tgz --strip-components=1 -C $(MUSL_DIR)

build: deps $(CC)
	CGO_ENABLED=$(CGO_ENABLED) GOOS=$(GOOS) GOARCH=$(GOARCH) CC=$(CC) go build -ldflags=$(FLAG)

clean:
	rm -rf $(MUSL_DIR) $(MUSL_DIR).tgz

.PHONY: all deps build clean
