# Value types

Environment variables are strings, but configuration values are not. This
chapter explains how PortalEnv turns a string into the type of a field, which
types are supported out of the box, and how to support your own types.

## How conversion works

Every string-based value — `env_var::NAME`, a string literal, or an array,
tuple or map literal in the mapping — is converted with the
[`FromEnvStrValue`](crate::FromEnvStrValue) trait of the field type. Before
conversion, the string is normalized:

- Leading and trailing whitespace is removed: `" 8080 "` becomes `8080`.
- A value wrapped in double quotes is taken literally without the quotes, so
  `"  padded  "` keeps its spaces. This also lets an element of a collection
  contain spaces or commas.

If the conversion fails, `from_env` returns an error that names the field and
the value. See [Errors](crate::docs::errors).

## Built-in types

| Type | Example value |
|---|---|
| `String` | `hello` |
| `bool` | `true`, `false` |
| `i8` – `i128`, `isize`, `u8` – `u128`, `usize` | `8080`, `-1` |
| `f32`, `f64` | `0.75` |
| `Vec<T>`, `HashSet<T>` | `[80, 443]` |
| `HashMap<K, V>` | `{read: 100, write: 10}` |
| Tuples of 2 to 9 elements | `(800, 600)` |

Numbers and booleans are parsed with Rust's standard `parse`, so `bool` accepts
exactly `true` and `false`.

## Collections

Arrays, maps and tuples use a syntax close to Rust literals. The elements, keys
and values are converted with the `FromEnvStrValue` implementation of their own
types:

| Field type | Value |
|---|---|
| `Vec<u16>` | `[80, 443]` |
| `Vec<String>` | `[api, web]` or `["api server", "web server"]` |
| `HashSet<u16>` | `[80, 443]` |
| `HashMap<String, u32>` | `{read: 100, write: 10}` |
| `(String, u32)` | `("localhost", 8080)` |

- Elements are separated by commas. Wrap an element in double quotes when it
  contains spaces or commas.
- An empty collection is written `[]` or `{}`.
- A tuple must have exactly as many elements as its type.
- Collections cannot be nested: `Vec<Vec<u32>>` is not supported.

The same syntax works in the mapping, in the process environment and in `.env`
files. In a `.env` file, **wrap the whole value in single quotes**:

```dotenv
ALLOWED_PORTS='[80, 443]'
HOSTS='["api server", "web server"]'
LIMITS='{read: 100, write: 10}'
```

Without the single quotes, the `.env` parser rejects values containing spaces,
and removes the double quotes around the elements, splitting `"api server"`
into two elements.

## Enums

Derive [`EnvEnum`](crate::EnvEnum) to convert a string into a fieldless enum by
its variant name:

```rust
use portalenv::*;

#[derive(Debug, PartialEq, EnvEnum)]
pub enum LogFormat {
    Text,
    Json,
}

#[derive(EnvPortal)]
#[env_portal(mapping = {
    log_format: "Json",
})]
pub struct AppConfig {
    pub log_format: LogFormat,
}

# fn main() -> portalenv::Result<()> {
let config = AppConfig::from_env()?;
assert_eq!(config.log_format, LogFormat::Json);
# Ok(())
# }
```

The value must be the variant name, `Json`, or the qualified name,
`LogFormat::Json`. The match is case-sensitive, and any other value is a
conversion error. Enums with fields are not supported.

`EnvEnum` is needed only when the value comes from a string, such as
`env_var::LOG_FORMAT`. A Rust expression like `log_format: LogFormat::Json`
works without it, as described in
[Mapping values](crate::docs::mapping#rust-expressions).

## Custom types

Implement [`FromEnvStrValue`](crate::FromEnvStrValue) for your own type to use
it as a field type. This is a good place to validate a value, so that an
invalid configuration is rejected at startup:

```rust
use portalenv::*;

pub struct EmailAddress(String);

impl FromEnvStrValue for EmailAddress {
    fn from_env_str(s: EnvStrValue) -> Result<Self> {
        if s.contains('@') {
            Ok(EmailAddress(s.into()))
        } else {
            Err(Self::conversion_error(s))
        }
    }
}

#[derive(EnvPortal)]
#[env_portal(mapping = {
    admin: "admin@example.com",
    notify: ["ops@example.com", "dev@example.com"],
})]
pub struct AppConfig {
    pub admin: EmailAddress,
    pub notify: Vec<EmailAddress>,
}

# fn main() -> portalenv::Result<()> {
let config = AppConfig::from_env()?;
assert_eq!(config.admin.0, "admin@example.com");
assert_eq!(config.notify.len(), 2);
# Ok(())
# }
```

- [`EnvStrValue`](crate::EnvStrValue) dereferences to `&str`, and converts into
  a `String` with `into()`.
- To reuse an existing conversion, call [`convert`](crate::EnvStrValue::convert),
  for example `let secs: u64 = s.convert()?;`.
- [`conversion_error`](crate::FromEnvStrValue::conversion_error) creates an
  error that includes the field name and the value.
- Once implemented, the type can also be used inside collections, such as
  `Vec<EmailAddress>` above.

Because of Rust's orphan rule, you cannot implement `FromEnvStrValue` for a
type from another crate, such as `std::time::Duration`. Wrap it in a newtype
as above, or build it with a [Rust expression](crate::docs::mapping#rust-expressions).

## Optional values

`Option<T>` does not implement `FromEnvStrValue`, so an `Option<T>` field
cannot be read directly with `env_var`. Use `None`, a `Some(...)` expression,
or [`env_partial_match`](crate::docs::environments#values-for-some-environments-only-env_partial_match)
instead.

Next: [Errors](crate::docs::errors)
