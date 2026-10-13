---
paths:
  - "packages/macro/**"
---

# portalenv-macro の実装規約

可読性の観点から、1マクロが1ファイルになるように分割して、書き方を以下の例の通りに揃える。

`lib.rs`

```rust
mod derive_env_portal;
derive_env_portal::define!();
```

`derive_env_portal.rs`

```rust
macro_rules! define {
    () => {
        /// ...
        #[proc_macro_error]
        #[proc_macro_derive(EnvPortal, attributes(env_portal))]
        pub fn derive_env_portal(input: TokenStream) -> TokenStream {
            derive_env_portal::exec(input.into()).into()
        }
    };
}

pub(crate) use define;

// use statements...

pub fn exec(input: TokenStream2) -> TokenStream2 {
    // implementation here...
}
```
