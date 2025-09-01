# README

不想写 README 了 : (

### 启动

systemd service

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

### 手动编译

检查

```sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

构建

```sh
cargo build --verbose --release
```

### API

- `/{id}` GET/POST
- `/` GET/POST
- `/-/{id}` GET

```sh
# POST
curl -d t="text" 127.0.0.1:8080/test
curl -d "text" 127.0.0.1:8080
cat /etc/hosts | curl --data-binary @- 127.0.0.1:8080/test
cat /etc/hosts | curl -F f=@- 127.0.0.1:8080
```

- `/b/` GET/POST
- `/b/{id}` GET/DELETE

```sh
# POST
curl -F f=@a.jpg 127.0.0.1:8080/b/
# DELETE
curl -X DELETE 127.0.0.1:8080/b/test -H 'token: 2A9B3F692B1715A6'
```
