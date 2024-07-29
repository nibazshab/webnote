# webnote

网页记事本，用于临时记录一些内容

## 使用说明

默认监听 10003 端口，数据储存在 webnote 可执行文件同级 data 目录下的 webnote.db 文件中

1. 编译源代码
2. 运行程序 `./webnote`

__编译步骤__

编译依赖：gcc，go

```sh
git clone https://github.com/nibazshab/webnote.git
cd webnote
go get ./...
CGO_ENABLED=1 go build -ldflags="-s -w"
```

> [!TIP]
> - 测试平台：Linux amd64
> - 反向代理时不要代理到域名子目录
> - 使用了 mattn/go-sqlite3 库而受限，编译产物非静态文件

__API__

> ___POST /{uid}___

请求：application/x-www-form-urlencoded，无返回

body：`t` 文本内容

> ___GET /{uid}___

返回该链接所对应的文本内容

## PLAN-B

- [x] 解决 favicon.ico 被重定向的问题
- [x] 变更相对路径为绝对路径

## 许可证

Powered by https://github.com/pereorga/minimalist-web-notepad

MIT © ZShab Niba
