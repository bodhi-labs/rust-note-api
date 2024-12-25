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

## Database Migration
- Create a Notes Table, you can use other method, 
- but now let try to using migration with `sqlx-cli`. 
- Install `sqlx-cli` and create a migration file

```shell
# CLI For migration
cargo install sqlx-cli

# create a migration
sqlx migrate add -r create_notes_table
```

- Then we can run the migration via `sqlx-cli`(And if you need to revert, you can also do that). 
- Finally our database is ready to be handle (you can check using Visual Tools like `DBeaver` or `MySQL Workbench` or else)

```shell
# perform migration up
sqlx migrate run

# (Bonus!, perform migration down/revert)
sqlx migrate revert
```

```shell
sqlx migrate run
Applied 20241224085617/migrate create notes table (818.578625ms)
```

## Define API Request Schema
Create schema.rs and define struct for Schema API Request

```shell
# Create schema.rs
touch src/schema.rs
```


