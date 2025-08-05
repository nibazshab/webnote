# Web Notepad

这是一个简单的网页记事本，用于临时记录一些内容

> v1 版本已迁移至 go 分支

### 快速上手

下载 Releases 中的文件直接运行即可，默认监听 8080 端口，默认数据位于程序同目录

支持环境变量如下

| 参数 | 描述 |
|-|-|
| PORT | 端口号 |
| DATA_DIR | 数据目录 |

配合 systemd 使用的 webnote.service

```ini
[Unit]
Description=webnote service
[Service]
Environment=PORT=10003 DATA_DIR=/usr/local/webnote
ExecStart=/usr/local/webnote/webnote
Restart=on-failure
[Install]
WantedBy=multi-user.target
```

### 构建说明

```sh
cd node && npm install && npm run build && cd ..

cargo check
cargo test
cargo fmt --all -- --check
cargo clippy -- -D warnings

cargo build --verbose --release
```

### API

| 路径 | 方法 | 描述 |
|-|-|-|
| /{id} | POST | 发送表单数据：t = 文本内容 |
| /{id} | GET | 获取文本内容 |

示例

```sh
# /{id} post
curl -d t="text" 127.0.0.1:10003/p
curl -d "text" 127.0.0.1:10003/p
cat /etc/hosts | curl 127.0.0.1:10003/p -v --data-binary @-

# /{id} get
curl 127.0.0.1:10003/p
```

## 许可证

Powered by https://github.com/pereorga/minimalist-web-notepad

MIT © ZShab Niba
