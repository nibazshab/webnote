# WebNote

这是一个简单的网页记事本，用于临时记录一些内容

### 快速上手

独立的二进制文件，直接运行即可，默认监听 `10003` 端口，使用 SQLite 存储内容，数据和日志位于 webnote_data 目录中

```sh
./webnote
```

### 构建说明

所需软件包：go, musl

go 使用包管理器或任意方式安装，musl 可以通过如下命令安装

```sh
musl="https://musl.cc/x86_64-linux-musl-cross.tgz"
wget -O- "$musl" | tar -zxvf - --strip-components=1 -C /usr/local
```

开始构建

```sh
go get ./...

flags="-s -w --extldflags '-static -fpic'"
export GOOS=linux
export GOARCH=amd64
export CC=x86_64-linux-musl-gcc
export CGO_ENABLED=1

go build -ldflags="$flags"
```

### 使用说明

命令行可以接收的参数

| 参数 | 默认值 | 描述 |
| - | - | - |
| -port | 10003 | 程序监听的端口号 |

### API

- ___POST /{uid}___

请求：application/x-www-form-urlencoded，无返回

body：`t` 文本内容

- ___GET /{uid}___

返回该链接所对应的文本内容

---

> [!TIP]
> - 测试平台：Linux amd64
> - 暂不支持反向代理到域名子目录

## PLAN-B

- [x] 解决 favicon.ico 被重定向的问题
- [x] 变更相对路径为绝对路径
- [x] 自定义端口号

## 许可证

Powered by https://github.com/pereorga/minimalist-web-notepad

MIT © ZShab Niba