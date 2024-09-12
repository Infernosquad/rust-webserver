# Basic Rust Web server supporting multithreading

Currently, supports GET requests only. :)

Multi threading implemented using official guide from Rust Book https://doc.rust-lang.org/beta/book/ch20-02-multithreaded.html

Features:

- Multi-threading
- Multi-socket
- Yaml configuration

Run:
```bash
cargo run example/config/config.yaml
```

Config file:
```yaml
---
  base_url: "127.0.0.1:8080"
  root: "example/html"
---
  base_url: "127.0.0.1:8081"
  root: "example/html"
```
