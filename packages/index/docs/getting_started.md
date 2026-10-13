# Getting started

This chapter builds a small configuration struct step by step: first with fixed
values, then with values read from environment variables, and finally with
those variables stored in a `.env` file.

## Installation

```bash
cargo add portalenv
```

## A first configuration struct

Define a struct that holds your configuration, derive [`EnvPortal`](crate::EnvPortal)
on it, and describe how each field gets its value in `mapping`:

```rust
use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(mapping = {
    app_name: "Demo",
    max_connections: 10,
})]
pub struct AppConfig {
    pub app_name: String,
    pub max_connections: u32,
}

fn main() -> portalenv::Result<()> {
    let config = AppConfig::from_env()?;

    assert_eq!(config.app_name, "Demo");
    assert_eq!(config.max_connections, 10);
    Ok(())
}
```

[`from_env`](crate::EnvPortal::from_env) builds the struct from the mapping.
Every field of the struct must appear in `mapping`; a missing one is reported
as a compile error, so a configuration value can never be forgotten silently.

## Reading environment variables

Fixed values alone are rarely enough. Use `env_var::NAME` to read a field from
the environment variable `NAME`:

```rust,no_run
use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(mapping = {
    app_name: "Demo",
    database_url: env_var::DATABASE_URL,
    port: env_var::PORT,
})]
pub struct AppConfig {
    pub app_name: String,
    pub database_url: String,
    pub port: u16,
}

fn main() -> portalenv::Result<()> {
    let config = AppConfig::from_env()?;
    println!("listening on port {}", config.port);
    Ok(())
}
```

Environment variables are always strings, so PortalEnv converts them into the
field type: `PORT=8080` becomes `8080_u16`. If `DATABASE_URL` or `PORT` is not
set, or `PORT` is not a valid `u16`, `from_env` returns an error instead of
building a half-valid configuration.

The mapping is also a single place that lists every environment variable your
application depends on.

## Using a `.env` file

During development it is convenient to keep these variables in a `.env` file
instead of exporting them in your shell. Put a `.env` file next to your
`Cargo.toml`:

```dotenv
DATABASE_URL=postgres://localhost/demo
PORT=8080
```

`from_env` loads this file automatically before reading the variables, so the
example above works without any change. A missing file is simply skipped.

Variables that are already set in the process environment take precedence over
the file, so a deployment can override any value without editing it:

```bash
PORT=9090 cargo run
```

To load a different file, specify its path with `dotenv_file`. Relative paths
are resolved against the current working directory:

```rust,no_run
# use portalenv::*;
#[derive(EnvPortal)]
#[env_portal(
    dotenv_file = "config/.env",
    mapping = {
        port: env_var::PORT,
    }
)]
pub struct AppConfig {
    pub port: u16,
}
```

## What's next

- Literals, Rust expressions and nested structs in `mapping` are covered in
  [Mapping values](crate::docs::mapping).
- Switching values between `local`, `stg` and `prd` is covered in
  [Environments](crate::docs::environments).

Next: [Mapping values](crate::docs::mapping)
