macro_rules! define {
    () => {
        /// Derive macro that implements the [`EnvPortal`] trait for a struct.
        ///
        /// It generates [`EnvPortal::from_env_with`], which builds an
        /// instance of the struct field by field according to the `mapping`
        /// given in the `#[env_portal(...)]` attribute, as well as
        /// [`EnvPortal::env_name_key`] and [`EnvPortal::dotenv_file`], which
        /// control which `.env` file(s) [`EnvPortal::from_env`] loads before
        /// construction.
        ///
        /// Can only be derived for structs with named fields.
        ///
        /// # Attribute arguments
        ///
        /// All arguments are optional and are passed as
        /// `#[env_portal(key = value, ...)]`:
        ///
        /// - `mapping = { field: value, ... }` — describes how each field of
        ///   the struct is produced. See "Mapping value syntax" below. Every
        ///   field of the struct must appear in the mapping (the generated
        ///   code is a plain struct literal), otherwise it fails to compile
        ///   with a "missing field" error; omitting `mapping` entirely only
        ///   works for a struct with no fields.
        /// - `dotenv_file = "path"` — path to the `.env` file to load,
        ///   relative to the current working directory. Defaults to
        ///   `"<CARGO_MANIFEST_DIR>/.env"`. If `env_name_key` resolves to a
        ///   value (say `"local"`), a second file named `"path.local"` is
        ///   loaded afterwards (when present), overriding keys from the base
        ///   file.
        /// - `env_name_key = "ENV_VAR_NAME"` — name of the environment
        ///   variable that identifies the current environment (e.g.
        ///   `"APP_ENV"`). Its value is required by `env_match` /
        ///   `env_partial_match` mapping values and by the environment-specific
        ///   `.env` file described above. When omitted, both `env_match` and
        ///   `env_partial_match` fields always fail with
        ///   [`Error::MissingEnvName`], since no environment value is
        ///   available to match against.
        ///
        /// # Mapping value syntax
        ///
        /// Each `field: value` entry in `mapping` accepts one of the
        /// following forms for `value`:
        ///
        /// - A literal (string, integer, float, bool, array or tuple), e.g.
        ///   `foo: "10"`, `foo: 10`, `foo: true`, `foo: [1, 2, 3]`,
        ///   `foo: (1, "a")`. The literal is rendered to its string form and
        ///   passed through [`EnvStrValue::convert`], so both a raw literal
        ///   (`10`) and its string equivalent (`"10"`) parse to the same
        ///   value. A brace block like `{ a: 1, b: 2 }` is treated the same
        ///   way and is typically used to build map-like fields
        ///   (`HashMap<K, V>`).
        /// - `env_var::IDENT` — reads the environment variable named `IDENT`
        ///   at runtime with `std::env::var` and converts it with
        ///   [`EnvStrValue::convert`]. Fails with the underlying [`VarError`]
        ///   if the variable is not set.
        /// - `None` — the literal Rust value `None`, useful for `Option<T>`
        ///   fields that should default to empty.
        /// - Any other Rust expression, e.g. `foo: 1 + 2`,
        ///   `foo: MyEnum::Variant`, `foo: if cond { a } else { b }` — spliced
        ///   verbatim into the generated code.
        /// - `Type { field: value, ... }` — builds a nested struct inline by
        ///   recursively applying this same mapping syntax; `Type` must not
        ///   itself derive [`EnvPortal`] for this form (it is constructed
        ///   directly, not via `from_env`).
        /// - `env_include<Type>` — delegates entirely to `Type::from_env()`
        ///   (where `Type` derives [`EnvPortal`]), embedding its result as
        ///   this field's value. Useful for composing independently-configured
        ///   sub-configs.
        /// - `env_include<Type> { field: value, ... }` — same as above, but
        ///   the listed fields override the corresponding fields of the
        ///   value returned by `Type::from_env()` (via struct update syntax),
        ///   while every other field of `Type` keeps the value produced by
        ///   its own `from_env`.
        /// - `env_match { pat => value, ... }` — matches the current
        ///   environment name (from `env_name_key`, required — otherwise
        ///   fails with [`Error::MissingEnvName`]) against string patterns
        ///   (e.g. `"local"`, `"stg" | "prd"`) and evaluates the mapping
        ///   value of the first matching arm (recursively, so a match arm's
        ///   value can itself be any of the forms described here, including
        ///   a nested `Type { ... }`). Fails with [`Error::InvalidEnvName`]
        ///   if no arm matches.
        /// - `env_partial_match { pat => value, ... }` — like `env_match`,
        ///   but the field type must be `Option<T>`: a matching arm produces
        ///   `Some(value)`, and no arm matching produces `None` instead of an
        ///   error.
        ///
        /// # Example
        ///
        /// ```ignore
        /// #[derive(EnvPortal)]
        /// #[env_portal(
        ///     env_name_key = "APP_ENV",
        ///     dotenv_file = ".env",
        ///     mapping = {
        ///         app_name: "Demo",
        ///         server: ServerConfig {
        ///             host: env_var::SERVER_HOST,
        ///             port: env_var::SERVER_PORT,
        ///         },
        ///         mail_to: env_match {
        ///             "local" | "stg" => "test@example.com",
        ///             "prd" => "prod@example.com",
        ///         },
        ///         extra: env_include<ExtraConfig> {
        ///             overridden_field: "custom-value",
        ///         },
        ///     }
        /// )]
        /// pub struct EnvConfig {
        ///     pub app_name: String,
        ///     pub server: ServerConfig,
        ///     pub mail_to: String,
        ///     pub extra: ExtraConfig,
        /// }
        /// ```
        ///
        /// [`EnvPortal`]: https://docs.rs/portalenv-core/latest/portalenv_core/env_portal/trait.EnvPortal.html
        /// [`EnvPortal::from_env_with`]: https://docs.rs/portalenv-core/latest/portalenv_core/env_portal/trait.EnvPortal.html#tymethod.from_env_with
        /// [`EnvPortal::env_name_key`]: https://docs.rs/portalenv-core/latest/portalenv_core/env_portal/trait.EnvPortal.html#tymethod.env_name_key
        /// [`EnvPortal::dotenv_file`]: https://docs.rs/portalenv-core/latest/portalenv_core/env_portal/trait.EnvPortal.html#tymethod.dotenv_file
        /// [`EnvPortal::from_env`]: https://docs.rs/portalenv-core/latest/portalenv_core/env_portal/trait.EnvPortal.html#method.from_env
        /// [`EnvStrValue::convert`]: https://docs.rs/portalenv-core/latest/portalenv_core/env_str_value/struct.EnvStrValue.html#method.convert
        /// [`VarError`]: https://doc.rust-lang.org/std/env/enum.VarError.html
        /// [`Error::MissingEnvName`]: https://docs.rs/portalenv-core/latest/portalenv_core/error/enum.Error.html#variant.MissingEnvName
        /// [`Error::InvalidEnvName`]: https://docs.rs/portalenv-core/latest/portalenv_core/error/enum.Error.html#variant.InvalidEnvName
        #[proc_macro_error]
        #[proc_macro_derive(EnvPortal, attributes(env_portal))]
        pub fn derive_env_portal(input: TokenStream) -> TokenStream {
            derive_env_portal::exec(input.into()).into()
        }
    };
}

pub(crate) use define;

use proc_macro_error::abort;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data, DataStruct, DeriveInput, Expr, ExprLit, Fields, Ident, Lit, LitStr, Pat, Token, Type,
    parse::{Parse, ParseStream},
    parse2,
    spanned::Spanned,
    token::{self},
};

use crate::helper::kw::{self};

pub fn exec(input: TokenStream2) -> TokenStream2 {
    let span = input.span();
    let DeriveInput {
        data:
            Data::Struct(DataStruct {
                fields: Fields::Named(_),
                ..
            }),
        ident,
        attrs,
        ..
    } = parse2(input).unwrap_or_else(|_| {
        abort!(span, "invalid input");
    })
    else {
        abort!(
            span,
            "EnvPortal can only be derived for structs with named fields"
        );
    };

    let env_portal_attr = attrs.iter().find(|attr| attr.path().is_ident("env_portal"));
    let attr_args = env_portal_attr
        .map(|attr| attr.parse_args_with(EnvPortalAttrArgs::parse))
        .transpose()
        .unwrap_or_else(|e| {
            abort!(
                env_portal_attr,
                "failed to parse 'env_portal' attribute: {}",
                e
            );
        })
        .unwrap_or_default();

    let constructor = make_constructor(None, None, &attr_args.mapping);
    let dotenv_file = attr_args
        .dotenv_file
        .map(|lit| quote!(Some(#lit)))
        .unwrap_or_else(|| quote!(Some(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"))));
    let env_name_key = attr_args
        .env_name_key
        .map(|lit| quote!(Some(#lit)))
        .unwrap_or_else(|| quote!(None));

    quote! {
        impl portalenv::EnvPortal for #ident {
            fn env_name_key() -> Option<&'static str> {
                #env_name_key
            }

            fn dotenv_file() -> Option<&'static str> {
                #dotenv_file
            }

            fn from_env_with(env: Option<String>) -> portalenv::Result<Self>
            where
                Self: Sized,
            {
                Ok(#constructor)
            }
        }
    }
}

fn make_constructor(
    parent_name: Option<&str>,
    type_: Option<&Type>,
    mapping: &Mapping,
) -> TokenStream2 {
    fn make_mapping_value(name: &str, value: &MappingValue) -> TokenStream2 {
        match value {
            MappingValue::None => quote!(None),
            MappingValue::EnvStr(s) => {
                quote!(
                    portalenv::EnvStrValue::new(#name, #s).convert()?
                )
            }
            MappingValue::EnvVar(var) => {
                let var_name = var.to_string();
                quote!(
                    portalenv::EnvStrValue::new(#name, std::env::var(#var_name)?).convert()?
                )
            }
            MappingValue::Expr(expr) => {
                quote!(#expr)
            }
            MappingValue::Nested { ty, mapping } => make_constructor(Some(name), Some(ty), mapping),

            MappingValue::EnvInclude { ty, mapping } => {
                if let Some(mapping) = mapping {
                    let field_constructs = make_field_constructs(mapping, Some(name));
                    quote!(
                        #ty {
                            #(#field_constructs),*
                            , ..(#ty::from_env()?)
                        }
                    )
                } else {
                    quote!(
                        #ty::from_env()?
                    )
                }
            }
            MappingValue::EnvMatch { arms, partial } => {
                let arms = arms.iter().map(|(pat, value)| {
                    let value = make_mapping_value(name, value);
                    let value = if *partial {
                        quote!(Some(#value))
                    } else {
                        quote!(#value)
                    };
                    quote!(
                        #pat => #value,
                    )
                });

                let fallback_arm = if *partial {
                    quote!(_ => None)
                } else {
                    quote!(_ => return Err(portalenv::Error::InvalidEnvName))
                };

                quote!({
                    let Some(env) = env.as_ref() else {
                        return Err(portalenv::Error::MissingEnvName);
                    };
                    match env.as_str() {
                        #(#arms)*
                        #fallback_arm,
                    }
                })
            }
        }
    }

    fn make_field_constructs(
        mapping: &Mapping,
        parent_name: Option<&str>,
    ) -> impl Iterator<Item = TokenStream2> {
        mapping.fields.iter().map(move |(f_ident, value)| {
            let field_name = if let Some(parent) = parent_name {
                format!("{}.{}", parent, f_ident)
            } else {
                f_ident.to_string()
            };
            let value = make_mapping_value(&field_name, value);
            quote!(
                #f_ident: #value
            )
        })
    }

    let type_ = type_.map(|t| quote!(#t)).unwrap_or_else(|| quote!(Self));
    let field_constructs = make_field_constructs(mapping, parent_name);

    quote!(
        #type_ {
            #(#field_constructs),*
        }
    )
}

#[derive(Default, Debug)]
struct EnvPortalAttrArgs {
    mapping: Mapping,
    dotenv_file: Option<LitStr>,
    env_name_key: Option<LitStr>,
}

impl Parse for EnvPortalAttrArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut mapping = Mapping::default();
        let mut dotenv_file = None;
        let mut env_name_key = None;

        while !input.is_empty() {
            if input.peek(kw::mapping) {
                input.parse::<kw::mapping>()?;
                input.parse::<Token![=]>()?;
                mapping = input.parse()?;
            } else if input.peek(kw::dotenv_file) {
                input.parse::<kw::dotenv_file>()?;
                input.parse::<Token![=]>()?;
                dotenv_file = Some(input.parse()?);
            } else if input.peek(kw::env_name_key) {
                input.parse::<kw::env_name_key>()?;
                input.parse::<Token![=]>()?;
                env_name_key = Some(input.parse()?);
            }
            input.parse::<Option<Token![,]>>()?;
        }

        Ok(EnvPortalAttrArgs {
            mapping,
            dotenv_file,
            env_name_key,
        })
    }
}

impl EnvPortalAttrArgs {}

#[derive(Default, Debug)]
struct Mapping {
    fields: Vec<(Ident, MappingValue)>,
}

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::braced!(content in input);

        let mut fields = Vec::new();
        while !content.is_empty() {
            let key: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            let value: MappingValue = content.parse()?;

            fields.push((key, value));
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(Mapping { fields })
    }
}

#[derive(Debug)]
enum MappingValue {
    EnvStr(TokenStream2),
    EnvVar(Ident),
    Expr(Expr),
    None,
    Nested {
        ty: Type,
        mapping: Mapping,
    },
    EnvMatch {
        arms: Vec<(Pat, MappingValue)>,
        partial: bool,
    },
    EnvInclude {
        ty: Type,
        mapping: Option<Mapping>,
    },
}

impl Parse for MappingValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::None) {
            let _ = input.parse::<kw::None>()?;
            Ok(MappingValue::None)
        } else if input.peek(kw::env_var) && input.peek2(token::PathSep) {
            input.parse::<kw::env_var>()?;
            input.parse::<token::PathSep>()?;
            let ident = input.parse()?;
            Ok(MappingValue::EnvVar(ident))
        } else if (input.peek(kw::env_match) || input.peek(kw::env_partial_match))
            && input.peek2(token::Brace)
        {
            let partial = if input.peek(kw::env_partial_match) {
                input.parse::<kw::env_partial_match>()?;
                true
            } else {
                input.parse::<kw::env_match>()?;
                false
            };

            let content;
            syn::braced!(content in input);

            let mut arms = Vec::new();
            while !content.is_empty() {
                let pat = Pat::parse_multi(&content)?;
                // let key: Ident = content.parse()?;
                content.parse::<Token![=>]>()?;
                let value: MappingValue = content.parse()?;
                arms.push((pat, value));
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }

            Ok(MappingValue::EnvMatch { arms, partial })
        } else if input.peek(kw::env_include) {
            input.parse::<kw::env_include>()?;
            input.parse::<token::Lt>()?;
            let ty = input.parse()?;
            input.parse::<token::Gt>()?;

            let mapping = if input.peek(token::Brace) {
                Some(input.parse()?)
            } else {
                None
            };

            Ok(MappingValue::EnvInclude { ty, mapping })
        } else if input.peek(Ident) && input.peek2(token::Brace) {
            let ty = input.parse()?;
            let mapping = input.parse()?;
            Ok(MappingValue::Nested { ty, mapping })
        } else if input.peek(token::Brace) {
            // { a: 1, b: 2 } のような Mapの定義
            let content;
            syn::braced!(content in input);

            let value: TokenStream2 = content.parse()?;
            let s = format!("{{{}}}", value);
            Ok(MappingValue::EnvStr(quote!(#s)))
        } else {
            let expr = input.parse()?;
            match expr {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => Ok(MappingValue::EnvStr(quote!(#s))),
                Expr::Array(a) => {
                    let s = format!("{}", quote!(#a));
                    Ok(MappingValue::EnvStr(quote!(#s)))
                }
                Expr::Tuple(t) => {
                    let s = format!("{}", quote!(#t));
                    Ok(MappingValue::EnvStr(quote!(#s)))
                }
                _ => Ok(MappingValue::Expr(expr)),
            }
        }
    }
}
