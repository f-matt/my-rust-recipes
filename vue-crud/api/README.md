# API Project

## System dependencies:

```
# zypper in postgresql-devel
```

## Create project: 

```
$ cargo new api
```

## Project dependencies:

```
$ cargo add axum
$ cargo add tokio --features full
$ cargo add serde --features derive
$ cargo add serde_json
$ cargo add dotenv
$ cargo add diesel --features postgresql
```

## Diesel assistant:

```
$ cargo install diesel_cli --no-default-features --features postgres
```

## Database config:

```
$ echo DATABASE_URL=postgres://username:password@localhost/db > .env
$ diesel setup
$ diesel migration generate create_rust_foos
$ diesel migration run
```

