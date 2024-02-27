# hostifier
Hostifier used for discovering virtual hosts/subdomains via http host header.

## Build & Run

```bash
cargo build
cargo run -- --domain host-to-scan.com --target http://10.10.x.x --wordlist names.txt
```

