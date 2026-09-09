//! proc-macros for generating trait implementations.

pub(crate) mod helper;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod derive_env_portal;
derive_env_portal::define!();

mod derive_env_enum;
derive_env_enum::define!();
