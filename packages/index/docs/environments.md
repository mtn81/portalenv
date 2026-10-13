# Environments

Most applications run in several environments, such as a developer's machine,
staging and production, and some settings differ between them. PortalEnv lets
you write those differences side by side in the mapping, so you can see at a
glance how the environments differ.

## Naming the current environment

First, tell PortalEnv which environment variable holds the name of the current
environment with `env_name_key`:

```rust,no_run
# use portalenv::*;
#[derive(EnvPortal)]
#[env_portal(
    env_name_key = "APP_ENV",
    mapping = {
        app_name: "Demo",
    }
)]
pub struct AppConfig {
    pub app_name: String,
}
```

The name is read from the process environment, or from the `.env` file when it
is not set there:

```bash
APP_ENV=prd ./my-app
```

The names themselves are up to you; this guide uses `local`, `stg` and `prd`.

Once `env_name_key` is specified, the environment name is required: if the
variable is set neither in the process environment nor in the `.env` file,
`from_env` returns an error.

## Switching values with `env_match`

`env_match` chooses a value by the environment name. Its arms look like those
of a Rust `match` on a string:

```rust
use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(
    env_name_key = "APP_ENV",
    mapping = {
        log_level: env_match {
            "local" => "debug",
            "stg" | "prd" => "info",
        },
        workers: env_match {
            "prd" => 16,
            _ => 2,
        },
    }
)]
pub struct AppConfig {
    pub log_level: String,
    pub workers: u32,
}

# fn main() -> portalenv::Result<()> {
# let config = AppConfig::from_env_with(Some("local".to_string()))?;
# assert_eq!(config.log_level, "debug");
# assert_eq!(config.workers, 2);
# let config = AppConfig::from_env_with(Some("prd".to_string()))?;
# assert_eq!(config.log_level, "info");
# assert_eq!(config.workers, 16);
# Ok(())
# }
```

- `"stg" | "prd"` matches either name.
- `_` matches any name, and can be used as a default.
- If no arm matches, `from_env` returns an error. Without a `_` arm, a typo in
  `APP_ENV` is therefore caught at startup instead of silently falling back.

The value of an arm can be any [mapping value](crate::docs::mapping), including
environment variables and nested structs:

```rust,no_run
# use portalenv::*;
#[derive(EnvPortal)]
#[env_portal(
    env_name_key = "APP_ENV",
    mapping = {
        database: env_match {
            "local" => DatabaseConfig {
                url: "postgres://localhost/demo",
                pool_size: 2,
            },
            "stg" | "prd" => DatabaseConfig {
                url: env_var::DATABASE_URL,
                pool_size: 20,
            },
        },
    }
)]
pub struct AppConfig {
    pub database: DatabaseConfig,
}

pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}
```

Conversely, `env_match` can be used inside a nested struct, so you can switch
only the fields that actually differ.

## Values for some environments only: `env_partial_match`

`env_partial_match` is a variant of `env_match` for `Option<T>` fields. A
matching arm produces `Some(value)`, and when no arm matches the field is
`None` instead of an error:

```rust
use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(
    env_name_key = "APP_ENV",
    mapping = {
        maintenance_banner: env_partial_match {
            "stg" => "This is the staging environment",
        },
    }
)]
pub struct AppConfig {
    pub maintenance_banner: Option<String>,
}

# fn main() -> portalenv::Result<()> {
# let config = AppConfig::from_env_with(Some("stg".to_string()))?;
# assert_eq!(config.maintenance_banner.as_deref(), Some("This is the staging environment"));
# let config = AppConfig::from_env_with(Some("prd".to_string()))?;
# assert_eq!(config.maintenance_banner, None);
# Ok(())
# }
```

Note that the arm values are written without `Some(...)`.

## Per-environment `.env` files

When `env_name_key` is specified, PortalEnv also loads a `.env` file dedicated
to the current environment, named after the base file with the environment
name appended:

```text
my-app/
├── Cargo.toml
├── .env          # loaded in every environment
├── .env.local    # loaded only when APP_ENV=local
└── .env.stg      # loaded only when APP_ENV=stg
```

If you set `dotenv_file = "config/.env"`, the files are `config/.env`,
`config/.env.local` and so on. Missing files are skipped.

A variable may be defined in more than one place. The first one found in the
following order wins:

1. The process environment
2. The per-environment file, such as `.env.local`
3. The base file, `.env`

So the base file holds the defaults, the per-environment file overrides them,
and the process environment overrides everything.

Since the environment name decides which per-environment file to load, it can
only be set in the process environment or the base `.env` file, not in a
per-environment file.

## `env_match` or per-environment files?

Both switch values per environment. A rule of thumb:

- Use **`env_match`** for values that can live in the source code. The
  differences between environments are reviewed together with the code, and
  you can see them all in one place.
- Use **`env_var` with per-environment files or the process environment** for
  secrets such as passwords and API keys, and for values decided at deployment
  time, such as the address of a database.

Next: [Composition](crate::docs::composition)
