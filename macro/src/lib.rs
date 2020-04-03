use self::lambda::Lambda;
use proc_macro::TokenStream;
#[cfg(not(feature = "nightly"))]
use proc_macro_hack::proc_macro_hack;
use quote::ToTokens;
use syn::parse_macro_input;

#[cfg_attr(feature = "nightly", proc_macro)]
#[cfg_attr(not(feature = "nightly"), proc_macro_hack)]
pub fn lambda(input: TokenStream) -> TokenStream {
    let lambda = parse_macro_input!(input as Lambda);
    lambda.into_token_stream().into()
}

mod lambda;
