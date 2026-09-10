macro_rules! define {
    () => {
        ///
        #[proc_macro_error]
        #[proc_macro_derive(EnvEnum)]
        pub fn derive_env_enum(input: TokenStream) -> TokenStream {
            derive_env_enum::exec(input.into()).into()
        }
    };
}

pub(crate) use define;

use proc_macro_error::abort;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, parse2, spanned::Spanned};

pub fn exec(input: TokenStream2) -> TokenStream2 {
    let span = input.span();
    let DeriveInput {
        ident,
        data: Data::Enum(DataEnum { variants, .. }),
        ..
    } = parse2(input).unwrap_or_else(|_| {
        abort!(span, "invalid input");
    })
    else {
        abort!(span, "EnvEnum can only be derived for enums");
    };

    let variant_match_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        let vname = vident.to_string();
        quote! {
            #vname => Ok(#ident::#vident),
            concat!(stringify!(#ident), "::", #vname) => Ok(#ident::#vident),
        }
    });

    quote! (
        impl portalenv::FromEnvStrValue for #ident {
            fn from_env_str(s: portalenv::EnvStrValue) -> portalenv::Result<Self> {
                match s.as_ref() {
                    #(#variant_match_arms)*
                    _ => Err(portalenv::Error::ValConversionError(format!(
                        "Failed to convert value to {}: {:?}",
                        stringify!(#ident),
                        s
                    ))),
                }
            }
        }
    )
}
