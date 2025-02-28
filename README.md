# Web Notepad

这是一个简单的网页记事本，用于临时记录一些内容

> v1 版本已迁移至 go 分支

### 快速上手

独立的二进制文件，直接运行即可，默认监听 10003 端口，使用 SQLite 存储内容

```sh
./webnote
```

### 构建说明

```sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

```sh
cargo build --verbose --release
```

### 使用说明

命令行可以接收的参数

| 参数    | 默认值          | 描述              |
|-------|--------------|-----------------|
| -P, --port | 10003        | 程序监听的端口号        |
| -D, --db-dir | . | 数据目录（相对程序文件的路径） |

### API

- ___POST /{uid}___

请求：application/x-www-form-urlencoded，无返回

body：`t` 文本内容

- ___GET /{uid}___

返回该链接所对应的文本内容

## 许可证

Powered by https://github.com/pereorga/minimalist-web-notepad

MIT © ZShab Niba
