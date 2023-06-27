#![feature(let_chains)]
#![cfg_attr(feature = "deny-warnings", deny(warnings))]
// warn on lints that are included in `rust-lang/rust`s bootstrap
#![warn(rust_2018_idioms, unused_lifetimes)]

pub(crate) mod paperclip;

use paperclip::expand_paperclip;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::Error;

#[proc_macro_derive(Paperclip, attributes(paperclip))]
pub fn derive_paperclip(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    expand_paperclip(derive_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
