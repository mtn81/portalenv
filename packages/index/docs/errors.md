# Errors

PortalEnv checks as much as possible at compile time, and reports the rest as
an error from `from_env` at runtime. This chapter lists both kinds, what causes
them, and how to fix them.

## Handling errors

`from_env` returns [`portalenv::Result<T>`](crate::Result), whose error type
[`Error`](crate::Error) implements `std::error::Error` and `Display`. Load the
configuration once at startup, and stop the application if it is invalid:

```rust,no_run
# use portalenv::*;
# #[derive(EnvPortal)]
# #[env_portal(mapping = { port: env_var::PORT })]
# pub struct AppConfig {
#     pub port: u16,
# }
fn main() {
    let config = AppConfig::from_env().unwrap_or_else(|e| {
        eprintln!("invalid configuration: {e}");
        std::process::exit(1);
    });
    // ...
#   let _ = config;
}
```

Since `Error` implements `std::error::Error`, it also works with `?` together
with crates such as `anyhow`.

## Runtime errors

| Variant | Cause |
|---|---|
| [`EnvVarError`](crate::Error::EnvVarError) | An environment variable is not set |
| [`DotEnvyError`](crate::Error::DotEnvyError) | A `.env` file cannot be parsed |
| [`ValConversionError`](crate::Error::ValConversionError) | A value cannot be converted into the field type |
| [`MissingEnvName`](crate::Error::MissingEnvName) | `env_match` is used without `env_name_key` |
| [`InvalidEnvName`](crate::Error::InvalidEnvName) | No `env_match` arm matches the environment name |

### `EnvVarError`

```text
environment variable `SERVER_PORT` not found (field `server.port`)
```

A variable read with `env_var::NAME`, or the variable named by
`env_name_key`, is set neither in the process environment nor in the loaded
`.env` files. The message shows the variable name and the field it is mapped
to, with the path of nested structs. The field is omitted for the variable
named by `env_name_key`. For a field of a struct composed with
`env_include<Type>`, the path is relative to `Type`, since `Type::from_env`
builds that struct on its own.

It is also returned when a variable contains invalid Unicode:

```text
environment variable `SERVER_PORT` is not valid unicode: "\xFF" (field `server.port`)
```

The variable name, the field path and the underlying
[`VarError`](std::env::VarError) are available as the `var`, `field` and
`source` fields of the variant.

### `DotEnvyError`

```text
Error parsing line: '[80, 443]', error at line index: 5
```

A `.env` file contains a line that is not a valid `KEY=value` pair. The message
shows the offending part of the line. A common cause is an unquoted value with
spaces; wrap such values in single quotes as described in
[Value types](crate::docs::value_types#collections).

### `ValConversionError`

```text
Failed to convert into u16: EnvStrValue { name: "server.port", value: "abc" }
```

A value cannot be converted into the field type. The message shows the type,
the field — with the path of nested structs, such as `server.port` — and the
value after normalization. Fix the value, or the field type if the value is
correct. Custom [`FromEnvStrValue`](crate::FromEnvStrValue) implementations
return this error as well.

### `MissingEnvName`

```text
Missing environment name
```

A field uses `env_match` or `env_partial_match`, but the struct has no
`env_name_key`, so there is no environment name to match against. Add
`env_name_key` as described in [Environments](crate::docs::environments).

### `InvalidEnvName`

```text
Invalid environment name
```

The environment name matches none of the arms of an `env_match`. Check the
value of the variable named by `env_name_key` for typos, or add a `_` arm as a
default.

## Compile errors

Mistakes in the `#[env_portal(...)]` attribute are reported at compile time.

### A field is missing from the mapping

```text
error: missing field `port` in `mapping`
```

Every field of the struct must appear in `mapping`. Add the field. For an
`Option<T>` field that should stay empty, write `port: None`.

### Unknown attribute argument

```text
error: failed to parse 'env_portal' attribute: expected one of: `mapping`, `dotenv_file`, `env_name_key`
```

The attribute accepts only `mapping`, `dotenv_file` and `env_name_key`. Check
for typos.

### Invalid mapping syntax

```text
error: failed to parse 'env_portal' attribute: expected `:`
```

Each entry of the mapping must be written as `field: value`, separated by
commas.

### Mismatched types

```text
error[E0308]: mismatched types
```

A [Rust expression](crate::docs::mapping#rust-expressions) does not have the
field type. Expressions are not converted, so `name: 10` does not compile for a
`String` field. Write a string literal, `name: "10"`, to have the value
converted instead.

### Missing `FromEnvStrValue` implementation

```text
error[E0277]: the trait bound `LogFormat: FromEnvStrValue` is not satisfied
```

A string-based value is used for a type that PortalEnv cannot convert. Derive
[`EnvEnum`](crate::EnvEnum) for a fieldless enum, or implement
`FromEnvStrValue` as described in
[Value types](crate::docs::value_types#custom-types). For an `Option<T>`
field, see [Optional values](crate::docs::value_types#optional-values).

### Derive on an unsupported type

```text
error: EnvPortal can only be derived for structs with named fields
error: EnvEnum can only be derived for enums with unit variants
```

`EnvPortal` supports structs with named fields only, and `EnvEnum` supports
enums whose variants have no fields.

Next: [Recipes](crate::docs::recipes)
