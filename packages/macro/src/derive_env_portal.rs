macro_rules! define {
    () => {
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

            MappingValue::EnvInclude(ty) => {
                quote!(
                    #ty::from_env()?
                )
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

    let field_constructs = mapping.fields.iter().map(|(f_ident, value)| {
        let field_name = if let Some(parent) = parent_name {
            format!("{}.{}", parent, f_ident)
        } else {
            f_ident.to_string()
        };
        let value = make_mapping_value(&field_name, value);
        quote!(
            #f_ident: #value
        )
    });

    let type_ = type_.map(|t| quote!(#t)).unwrap_or_else(|| quote!(Self));

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
    EnvInclude(Type),
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
            Ok(MappingValue::EnvInclude(ty))
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
