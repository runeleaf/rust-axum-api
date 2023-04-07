# Rust Axum API Example

[axum](https://github.com/tokio-rs/axum)

## Setup

```bash
$ cargo install cargo-shuttle
$ cargo install cargo-watch
$ cargo install sqlx-cli
```

### Format Rust code

```bash
$ rustup component add rustfmt
$ cargo fmt
```

### Create Database

```bash
$ export DATABASE_URL=postgres://postgres:password@localhost:5432/rust_axum_development
or
$ cat > .env
DATABASE_URL=postgres://postgres:password@localhost:5432/rust_axum_development

$ export $(cat .env | xargs)

$ cargo sqlx database create
```

### Migration

```bash
$ cargo sqlx migrate add create_users
$ cargo sqlx migrate run
```

### Run Server

```bash
$ cargo watch -x run
```
