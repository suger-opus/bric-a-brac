# Knowledge microservice

> Rust RPC API built with gRPC

## Usage

> Don't forget to have a well configured `mise.local.toml`

First run the database:
```bash
mise run db-start
```
> See `mise.toml` for more commands

Then run the API locally:
```bash
cargo run
```

Or with docker:
```bash
mise run server-start
```
> See `mise.toml` for more commands
