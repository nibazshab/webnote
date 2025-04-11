# Web Notepad

这是一个简单的网页记事本，用于临时记录一些内容

> go 语言版本已迁移至 go 分支

### 快速上手

独立的二进制文件，直接运行即可，默认监听 10003 端口，使用 SQLite 存储内容

```sh
./webnote
```

配合 systemd 使用 webnote.service

```ini
[Unit]
Description=Webnote service
[Service]
ExecStart=/usr/local/webnote/webnote  -D /usr/local/webnote
Restart=on-failure
[Install]
WantedBy=multi-user.target
```

### 构建说明

```sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

```sh
cd node && npm install && npm run build
cd .. && cargo build --verbose --release
```

### 使用说明

命令行可以接收的参数

| 参数 | 默认值 | 描述 |
|-|-|-|
| -P, --port | 10003 | 程序监听的端口号 |
| -D, --db-dir | 程序同目录 | 数据目录 |

### API

- ___POST /{uid}___

请求：application/x-www-form-urlencoded，无返回

body：`t` 文本内容

- ___GET /{uid}___

返回该链接所对应的文本内容

## 许可证

Powered by https://github.com/pereorga/minimalist-web-notepad

MIT © ZShab Niba
