# webnote

网页记事本，用于临时记录一些内容

## 使用说明

默认监听 10003 端口，数据以文件的形式明文储存在 webnote 文件同目录的 tmp 目录下（需要自己手动创建）

1. 编译 main.go
2. 创建 tmp 目录
3. 运行程序 `./webnote`

__编译步骤__

```sh
git clone https://github.com/nibazshab/webnote.git
cd webnote
CGO_ENABLED=0 go build -ldflags="-s -w"
```

## API

> ___POST /{uid}___

参数：`t` 文本内容

无响应

> ___GET /{uid}___

返回该链接所对应的文本内容

## 开源地址

https://github.com/nibazshab/webnote

Powered by https://github.com/pereorga/minimalist-web-notepad

## 许可证

MIT © ZShab Niba
