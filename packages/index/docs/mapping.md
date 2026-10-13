# Mapping values

The `mapping` of `#[env_portal(...)]` lists every field of the struct together
with the value it gets:

```text
mapping = {
    field: value,
    ...
}
```

This chapter explains the kinds of `value` you can write. They fall into two
groups:

- **String-based values** — string literals, array, tuple and map literals, and
  `env_var::NAME`. They are treated as strings and converted into the field
  type, exactly as a value read from an environment variable would be.
- **Rust expressions** — everything else, such as `10`, `true` or
  `Duration::from_secs(30)`. They are used as is, and must already have the
  field type.

## String literals

A string literal is converted into the field type, so it can be used for any
type that PortalEnv knows how to parse:

```rust
use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(mapping = {
    name: "Demo",
    port: "8080",
    debug: "true",
})]
pub struct AppConfig {
    pub name: String,
    pub port: u16,
    pub debug: bool,
}

# fn main() -> portalenv::Result<()> {
let config = AppConfig::from_env()?;
assert_eq!(config.port, 8080);
assert!(config.debug);
# Ok(())
# }
```

Because the value goes through the same conversion as an environment variable,
replacing `"8080"` with `env_var::PORT` later never changes how the value is
parsed. The supported types and how to add your own are described in
[Value types](crate::docs::value_types).

## Rust expressions

Any other Rust expression is inserted into the generated code verbatim. This
includes number and boolean literals, arithmetic, enum variants, constants and
function calls:

```rust
use portalenv::*;
use std::time::Duration;

const DEFAULT_RETRIES: u8 = 3;

pub enum LogFormat {
    Text,
    Json,
}

#[derive(EnvPortal)]
#[env_portal(mapping = {
    port: 8080,
    debug: false,
    retries: DEFAULT_RETRIES,
    max_body_size: 4 * 1024 * 1024,
    timeout: Duration::from_secs(30),
    log_format: LogFormat::Json,
})]
pub struct AppConfig {
    pub port: u16,
    pub debug: bool,
    pub retries: u8,
    pub max_body_size: usize,
    pub timeout: Duration,
    pub log_format: LogFormat,
}

# fn main() -> portalenv::Result<()> {
let config = AppConfig::from_env()?;
assert_eq!(config.max_body_size, 4_194_304);
assert_eq!(config.timeout, Duration::from_secs(30));
# Ok(())
# }
```

Since expressions are not converted, their type must match the field type
exactly. For example, `name: 10` for a `String` field is a compile error; write
`name: "10"` instead.

Expressions are the way to use types that cannot be parsed from a string, such
as `Duration` above, or `LogFormat`, which does not need to implement any trait
when it is written as an expression.

## Arrays, tuples and maps

Array, tuple and map literals are string-based values, too. Each element is
converted into the element type of the field:

```rust
use portalenv::*;
use std::collections::{HashMap, HashSet};

#[derive(EnvPortal)]
#[env_portal(mapping = {
    hosts: ["api.example.com", "web.example.com"],
    allowed_ports: [80, 443],
    limits: {
        read: 100,
        write: 10,
    },
    window: (800, 600),
})]
pub struct AppConfig {
    pub hosts: Vec<String>,
    pub allowed_ports: HashSet<u16>,
    pub limits: HashMap<String, u32>,
    pub window: (u32, u32),
}

# fn main() -> portalenv::Result<()> {
let config = AppConfig::from_env()?;
assert_eq!(config.hosts, ["api.example.com", "web.example.com"]);
assert!(config.allowed_ports.contains(&443));
assert_eq!(config.limits["write"], 10);
assert_eq!(config.window, (800, 600));
# Ok(())
# }
```

The same syntax can be used in environment variables and `.env` files, for
example `ALLOWED_PORTS='[80, 443]'`. See
[Value types](crate::docs::value_types#collections) for the details, including
how to quote such values in a `.env` file.

## Environment variables

`env_var::NAME` reads the environment variable `NAME` and converts it into the
field type:

```rust,no_run
# use portalenv::*;
#[derive(EnvPortal)]
#[env_portal(mapping = {
    database_url: env_var::DATABASE_URL,
    pool_size: env_var::DATABASE_POOL_SIZE,
})]
pub struct AppConfig {
    pub database_url: String,
    pub pool_size: u32,
}
```

The variable is required: if it is not set, `from_env` returns an error.
Variables defined in a `.env` file are available here as well, as described in
[Getting started](crate::docs::getting_started#using-a-env-file).

## Optional values

For an `Option<T>` field, write `None` to leave it empty, or a `Some(...)`
expression to give it a value:

```rust
use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(mapping = {
    proxy: None,
    banner: Some("Welcome!".to_string()),
})]
pub struct AppConfig {
    pub proxy: Option<String>,
    pub banner: Option<String>,
}

# fn main() -> portalenv::Result<()> {
let config = AppConfig::from_env()?;
assert!(config.proxy.is_none());
assert_eq!(config.banner.as_deref(), Some("Welcome!"));
# Ok(())
# }
```

To set a value only in some environments, use `env_partial_match`, described in
[Environments](crate::docs::environments).

## Nested structs

Configuration is easier to read when related values are grouped. Write
`Type { ... }` to build a nested struct with the same mapping syntax:

```rust
use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(mapping = {
    server: ServerConfig {
        host: "127.0.0.1",
        port: 8080,
    },
    database: DatabaseConfig {
        url: "postgres://localhost/demo",
        pool: PoolConfig {
            min: 1,
            max: 10,
        },
    },
})]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub struct DatabaseConfig {
    pub url: String,
    pub pool: PoolConfig,
}

pub struct PoolConfig {
    pub min: u32,
    pub max: u32,
}

# fn main() -> portalenv::Result<()> {
let config = AppConfig::from_env()?;
assert_eq!(config.server.port, 8080);
assert_eq!(config.database.pool.max, 10);
# Ok(())
# }
```

The nested types are plain structs: they do not derive `EnvPortal`, and the
whole configuration is visible in one place. Structs can be nested to any
depth, and every kind of value in this chapter can be used inside them.

When a nested struct should keep its own mapping instead, for example because
it is shared by several applications, use `env_include` as described in
[Composition](crate::docs::composition).

## Summary

| Value | Example | Converted from a string |
|---|---|---|
| String literal | `"8080"` | Yes |
| Array, tuple, map | `[1, 2]`, `(1, "a")`, `{ a: 1 }` | Yes |
| Environment variable | `env_var::PORT` | Yes |
| Rust expression | `8080`, `Duration::from_secs(30)` | No |
| `None` | `None` | No |
| Nested struct | `ServerConfig { .. }` | – |
| Per-environment value | `env_match { .. }` | See [Environments](crate::docs::environments) |
| Included config | `env_include<T>` | See [Composition](crate::docs::composition) |

For the formal syntax, see the [`EnvPortal`](crate::EnvPortal) derive macro
reference.

Next: [Environments](crate::docs::environments)
