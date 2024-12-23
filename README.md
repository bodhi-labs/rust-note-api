# Setup

- cargo init

```shell

# Init Project
cargo init rust-note-api

```

- add Depedencies
```shell

# Depedency
cargo add axum
cargo add tokio -F full
cargo add tower-http -F "cors"
cargo add serde_json
cargo add serde -F derive
cargo add chrono -F serde
cargo add dotenv
cargo add uuid -F "serde v4"
cargo add sqlx --features "runtime-async-std-native-tls mysql chrono uuid"

```
- Additionally, we can also re-run automatically every time the code change using cargo-watch

```shell
# CLI For Watch source when running & Automatically rebuild the project
cargo install cargo-watch

# Run with watch the src/
cargo watch -q -c -w src/ -x run
```

