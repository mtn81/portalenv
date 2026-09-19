# PortalEnv

PortalEnv is a Rust-friendly, ergonomic and type-safe environment configuration library.

PortalEnv provides a clearer view of your configuration.

- Eliminate duplicated common settings.
- See at a glance which environment variables are in use.
- See clearly how environments differ.

<div align="left">
  <!-- Crates version -->
  <a href="https://crates.io/crates/portalenv">
    <img src="https://img.shields.io/crates/v/portalenv.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- docs -->
  <a href="https://docs.rs/portalenv">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- CI -->
  <a href="https://github.com/mtn81/portalenv/actions">
    <img src="https://github.com/mtn81/portalenv/actions/workflows/ci-for-main.yml/badge.svg"
      alt="CI status" />
  </a>
</div>

## Features

Embed environment configuration directly in your Rust code with dedicated derive macros.

- Native Rust expressions as configuration values.
- Environment variable references.
- Value switching by environment name.
- Nested object configuration.
- Inclusion and override of other env configurations.
- Type-safe implicit conversion from string values, including custom value types.
- Dotenv file support via `dotenvy`.
- Array, map, and tuple literal syntax and these are also available in dotenv files.

## Installation

Run the following command:

```bash
cargo add portalenv
```

Or add it to your `Cargo.toml` manually:

```toml
[dependencies]
portalenv = "0.1"
```

## Example

```rust
use portalenv::*;
use std::collections::{HashMap, HashSet};

#[derive(EnvPortal)]
#[env_portal(
    env_name_key = "APP_ENV",
    mapping = {
        app_name: "Demo",
        server: ServerConfig {
            host: env_var::SERVER_HOST,
            port: env_var::SERVER_PORT,
        },
        email: EmailConfig {
            address: env_match {
                "local" | "stg" => "portalenv-test@xxx.yyy",
                "prd" => "portalenv@xxx.yyy",
            },
            title: env_partial_match {
                "prd" => "portalenv mail"
            }
        },
        demo: DemoConfig {
            vec: ["this", "is", "a", "demo"],
            set: [1, 2, 3],
            map: {
                count_a: 100,
                count_b: 200,
            },
            child: DemoChild {
                name: "portal env",
                amount: 20,
                size: SizeType::Small,
            },
        }
    }
)]
pub struct EnvConfig {
    app_name: String,
    server: ServerConfig,
    email: EmailConfig,
    demo: DemoConfig,
}
pub struct ServerConfig {
    host: String,
    port: u32,
}
pub struct EmailConfig {
    address: EmailAddress,
    title: Option<String>,
}
pub struct EmailAddress(String);
impl FromEnvStrValue for EmailAddress {
    fn from_env_str(s: EnvStrValue) -> Result<Self> {
        // return Err(...) here if the value is invalid
        Ok(EmailAddress(s.into()))
    }
}

pub struct DemoConfig {
    vec: Vec<String>,
    set: HashSet<u32>,
    map: HashMap<String, u32>,
    child: DemoChild,
}

pub struct DemoChild {
    name: String,
    amount: u32,
    size: SizeType,
}

#[derive(EnvEnum)]
pub enum SizeType {
    Small, Medium, Large
}

fn main() {
    let _config = EnvConfig::from_env().unwrap();
}
```

## Guides

For detailed guides, see the [docs page](https://docs.rs/portalenv/latest/portalenv/docs/index.html)

## License

Licensed under either of [Apache License, Version
2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
