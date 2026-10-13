# Recipes

Practical patterns for using PortalEnv in real applications.

## Loading the configuration once

Configuration rarely changes while an application runs. Load it once, and
share it through a static:

```rust
use portalenv::*;
use std::sync::LazyLock;

#[derive(EnvPortal)]
#[env_portal(mapping = {
    app_name: "Demo",
})]
pub struct AppConfig {
    pub app_name: String,
}

pub static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::from_env().expect("invalid configuration"));

fn main() {
    // Fail at startup rather than on first use deep inside the application.
    LazyLock::force(&CONFIG);

    assert_eq!(CONFIG.app_name, "Demo");
}
```

## Default values for optional variables

`env_var::NAME` requires the variable to be set. For a variable that may be
omitted, write a small helper and call it as a
[Rust expression](crate::docs::mapping#rust-expressions). The `?` operator can
be used in mapping expressions, and
[`EnvStrValue::from_env_var`](crate::EnvStrValue::from_env_var) reads and
converts a variable the same way as `env_var`:

```rust
use portalenv::*;

fn env_or<T: FromEnvStrValue>(name: &'static str, default: T) -> Result<T> {
    match EnvStrValue::from_env_var(name, name) {
        Ok(value) => value.convert(),
        Err(Error::EnvVarError {
            source: std::env::VarError::NotPresent,
            ..
        }) => Ok(default),
        Err(e) => Err(e),
    }
}

#[derive(EnvPortal)]
#[env_portal(mapping = {
    port: env_or("RECIPE_DEFAULT_PORT", 8080)?,
})]
pub struct AppConfig {
    pub port: u16,
}

# fn main() -> portalenv::Result<()> {
let config = AppConfig::from_env()?;
assert_eq!(config.port, 8080);
# Ok(())
# }
```

Since the helper reads the process environment when `from_env` runs, the
variable can also be defined in a `.env` file.

## Testing every environment

A typo in a rarely used environment, such as an `env_match` arm that does not
convert, should not be found in production.
[`from_env_with`](crate::EnvPortal::from_env_with) builds the configuration for
a given environment name, so a test can check all of them:

```rust
use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(
    env_name_key = "APP_ENV",
    mapping = {
        workers: env_match {
            "local" => 2,
            "stg" | "prd" => 16,
        },
        timeout_secs: env_match {
            "local" => "60",
            "stg" | "prd" => "10",
        },
    }
)]
pub struct AppConfig {
    pub workers: u32,
    pub timeout_secs: u64,
}

// In your test module, mark this function with #[test].
fn config_is_valid_in_every_environment() {
    for env in ["local", "stg", "prd"] {
        if let Err(e) = AppConfig::from_env_with(Some(env.to_string())) {
            panic!("invalid configuration for {env}: {e}");
        }
    }
}
# fn main() { config_is_valid_in_every_environment(); }
```

Unlike `from_env`, `from_env_with` does not load `.env` files, and fields read
with `env_var` still need their variables to be set.

## Tests that set environment variables

Environment variables are shared by the whole test process, and Rust runs tests
in parallel. Run tests that change environment variables one at a time, for
example with the [`serial_test`](https://crates.io/crates/serial_test) crate:

```rust,no_run
use portalenv::*;
use serial_test::serial;

#[derive(EnvPortal)]
#[env_portal(mapping = {
    port: env_var::PORT,
})]
pub struct AppConfig {
    pub port: u16,
}

#[test]
#[serial]
fn reads_port_from_environment() {
    // SAFETY: #[serial] ensures no other test accesses the environment concurrently.
    unsafe { std::env::set_var("PORT", "9090") };

    assert_eq!(AppConfig::from_env().unwrap().port, 9090);

    unsafe { std::env::remove_var("PORT") };
}
```

Remember that `from_env` also loads your `.env` files, so variables defined
there are visible in tests unless they are set in the process environment.

## `.env` files in production

By default, the `.env` file is looked up at `<CARGO_MANIFEST_DIR>/.env`, an
absolute path fixed when the application is compiled. This is convenient during
development, since it works from any working directory, but the path usually
does not exist where the binary is deployed, and the file is skipped.

Choose one of the following for deployed applications:

- **Use the process environment only.** This is the common approach for
  containers and cloud platforms; nothing needs to change.
- **Deploy a `.env` file next to the application.** Set a path relative to the
  working directory, such as `dotenv_file = ".env"`, and start the application
  from that directory.

In both cases, keep secrets out of the repository:

- Commit `.env` with defaults that are safe to share, such as values for local
  development.
- Do not commit files that contain secrets, such as `.env.local` or
  `.env.prd`; add them to `.gitignore`.
- Read secrets with `env_var`, and let each environment provide them.

## Sharing configuration across a workspace

To share a configuration between crates of a workspace, define it in a library
crate and [`env_include`](crate::docs::composition) it from each application.

Note that the default `dotenv_file` of the shared configuration points to the
`.env` file of the *library* crate, not of the application. Variables defined
in the application's `.env` are loaded first and take precedence, so you can
either keep shared defaults in the library's `.env` and override them per
application, or set `dotenv_file` explicitly in the shared configuration.

Back to the [user guide](crate::docs)
