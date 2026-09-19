macro_rules! define {
    () => {
        /// Derive macro that implements [`FromEnvStrValue`] for a fieldless
        /// enum.
        ///
        /// It lets the enum be used directly as a field type in a struct
        /// deriving [`EnvPortal`] (`#[derive(EnvPortal)]`): the raw string
        /// obtained from a configuration value or an environment variable is
        /// matched against each variant name and converted to that variant.
        ///
        /// A value matches a variant if it is equal to either:
        /// - the bare variant name (e.g. `"B"`), or
        /// - the variant name qualified with the enum name (e.g. `"MyEnum::B"`).
        ///
        /// The match is case-sensitive and exact (no trimming or normalization
        /// beyond what [`EnvStrValue`] already performs). If the value does
        /// not match any variant, conversion fails with
        /// [`Error::ValConversionError`].
        ///
        /// # Example
        ///
        /// ```ignore
        /// #[derive(EnvEnum)]
        /// pub enum SizeType {
        ///     Small,
        ///     Medium,
        ///     Large,
        /// }
        ///
        /// #[derive(EnvPortal)]
        /// #[env_portal(mapping = {
        ///     size: "Small",              // or "SizeType::Small"
        /// })]
        /// pub struct EnvConfig {
        ///     pub size: SizeType,
        /// }
        /// ```
        ///
        /// # Limitations
        ///
        /// Only enums with unit (fieldless) variants are supported; the
        /// generated code is a plain string match on the variant name.
        ///
        /// [`FromEnvStrValue`]: https://docs.rs/portalenv-core/latest/portalenv_core/env_str_value/trait.FromEnvStrValue.html
        /// [`EnvPortal`]: https://docs.rs/portalenv-core/latest/portalenv_core/env_portal/trait.EnvPortal.html
        /// [`EnvStrValue`]: https://docs.rs/portalenv-core/latest/portalenv_core/env_str_value/struct.EnvStrValue.html
        /// [`Error::ValConversionError`]: https://docs.rs/portalenv-core/latest/portalenv_core/error/enum.Error.html#variant.ValConversionError
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
