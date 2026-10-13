# Composition

As an application grows, parts of its configuration are often shared: several
binaries connect to the same database, or a library comes with its own
settings. `env_include` lets you define such a part once, as its own
`EnvPortal` struct, and include it wherever it is needed.

## Including another configuration

Write `env_include<Type>` to fill a field with `Type::from_env()`, where `Type`
derives `EnvPortal` itself:

```rust
use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(mapping = {
    host: "localhost",
    port: 5432,
    pool_size: 10,
})]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub pool_size: u32,
}

#[derive(EnvPortal)]
#[env_portal(mapping = {
    app_name: "api",
    database: env_include<DatabaseConfig>,
})]
pub struct ApiConfig {
    pub app_name: String,
    pub database: DatabaseConfig,
}

# fn main() -> portalenv::Result<()> {
let config = ApiConfig::from_env()?;
assert_eq!(config.database.pool_size, 10);
# Ok(())
# }
```

The included configuration is built exactly as if `DatabaseConfig::from_env()`
were called on its own, with its own `mapping`, `env_name_key` and
`dotenv_file`. It can be placed in another module or crate, and can include
other configurations in turn.

Compare this with a [nested struct](crate::docs::mapping#nested-structs), which
is written inline and belongs to a single configuration. Use a nested struct to
group values, and `env_include` to reuse them.

## Overriding included values

To change some fields of an included configuration, list them in braces after
`env_include<Type>`. The other fields keep the values produced by
`Type::from_env()`:

```rust
# use portalenv::*;
#
# #[derive(EnvPortal)]
# #[env_portal(mapping = {
#     host: "localhost",
#     port: 5432,
#     pool_size: 10,
# })]
# pub struct DatabaseConfig {
#     pub host: String,
#     pub port: u16,
#     pub pool_size: u32,
# }
#
#[derive(EnvPortal)]
#[env_portal(mapping = {
    app_name: "batch",
    database: env_include<DatabaseConfig> {
        pool_size: 2,
    },
})]
pub struct BatchConfig {
    pub app_name: String,
    pub database: DatabaseConfig,
}

# fn main() -> portalenv::Result<()> {
let config = BatchConfig::from_env()?;
assert_eq!(config.database.host, "localhost");
assert_eq!(config.database.pool_size, 2);
# Ok(())
# }
```

An overriding value can be any [mapping value](crate::docs::mapping),
including `env_var` and `env_match`, so you can, for example, adjust a shared
setting only in production:

```rust,no_run
# use portalenv::*;
#
# #[derive(EnvPortal)]
# #[env_portal(mapping = { host: "localhost", port: 5432, pool_size: 10 })]
# pub struct DatabaseConfig {
#     pub host: String,
#     pub port: u16,
#     pub pool_size: u32,
# }
#
#[derive(EnvPortal)]
#[env_portal(
    env_name_key = "APP_ENV",
    mapping = {
        database: env_include<DatabaseConfig> {
            pool_size: env_match {
                "prd" => 50,
                _ => 10,
            },
        },
    }
)]
pub struct ApiConfig {
    pub database: DatabaseConfig,
}
```

Keep in mind the following:

- An override replaces the whole field. To change a single field of a struct
  inside the included configuration, write the complete struct.
- The included configuration is still built in full before the overrides are
  applied. Every environment variable it reads must therefore be set, even if
  the corresponding field is overridden.
- The overridden fields must be visible from where the mapping is written, so
  they are usually `pub`.

## `.env` files of included configurations

Each included configuration loads its own `.env` files when it is built, after
the including configuration has loaded its own. Because loading a file never
overrides a variable that is already set, values from the outer configuration's
files take precedence over those of the included one.

When both configurations use the default `dotenv_file` and live in the same
crate, they share the same `.env` file.

Next: [Value types](crate::docs::value_types)
