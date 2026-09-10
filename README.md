# PortalEnv

PortalEnv is a rust-friendly, ergonomic and type safe env configuration library.

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

### Features

You can embed env configuration in rust codes with dedicated derive macros.

- Natively rust expression is enabled.
- Env variable reference.
- Env matching with envronment name.
- Array-like, HashMap, Tuple syntax support.
- Nested object configuration.
- Implicit type conversion from string value.
- dotenv file support via `dotenvy`.

By using PortalEnv, you can gain clearer configuration.

- Eliminate duplicate common settings.
- The environment variables in use become clear at a glance.
- The differences between environments become clear.

### Example

```rust
use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(
    dotenv_file = ".env",
    env_name_key = "APP_ENV",
    mapping = {
        app_name: "Demo",
        server: ServerConfig {
            host: env_var::SERVER_HOST,
            port: env_var::SERVER_PORT,
        },
        email: EmailConfig {
            address: env_match {
                "local" | "stg" => "potalenv-test@xxx.yyy",
                "prd" => "potalenv@xxx.yyy",
            }
        },
        demo: DemoConfig {
            vec: ["this", "is", "a", "demo"],
            set: [1, 2, 3],
            map: {
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
    address: String,
}
pub struct DemoConfig {
    vec: Vec<String>,
    set: HashSet<u32>,
    map: HashMap<String, u32>,
}

#[derive(EnvEnum)]
pub enum SizeType {
    Small, Medium, Large
}

fn main() {
    let _config = EnvConfig::from_env().unwrap();
}


```

### Guides

For detailed guides, see [docs page](https://docs.rs/portalenv/latest/portalenv/docs/index.html)

#### License

Licensed under either of [Apache License, Version
2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
