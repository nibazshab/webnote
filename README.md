# Web Notepad

这是一个简单的网页记事本，用于临时记录一些内容

> v1 版本已迁移至 go 分支

### 快速上手

下载 Releases 中的文件直接运行即可，默认监听 10003 端口，使用 SQLite 存储内容

命令行可以接收的参数

| 参数 | 默认值 | 描述 |
|-|-|-|
| -P / --port | 10003 | 端口号 |
| -D / --db-dir | 程序同目录 | 数据目录 |

配合 systemd 使用的 webnote.service

```ini
[Unit]
Description=webnote service
[Service]
ExecStart=/usr/local/webnote/webnote -D /usr/local/webnote -P 10003
Restart=on-failure
[Install]
WantedBy=multi-user.target
```

### 构建说明

1. 检查

```sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

2. 构建

```sh
cd node && npm install && npm run build
cd .. && cargo build --verbose --release
```

### API

| 参数 | 默认值 | 描述 |
|-|-|-|
| /{uid} | POST | 表单数据：t = 文本内容 |
| /{uid} | GET | 文本内容 |

示例

```sh
# /{uid} post
curl -d t="text" 127.0.0.1:10003/p

# /{uid} get
curl 127.0.0.1:10003/p
```

## 许可证

Powered by https://github.com/pereorga/minimalist-web-notepad

MIT © ZShab Niba
