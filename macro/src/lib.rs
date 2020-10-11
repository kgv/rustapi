#![feature(bindings_after_at)]

use proc_macro::TokenStream;

/// FnOnce derive macro.
#[proc_macro_derive(FnOnce, attributes(fn_once))]
pub fn fn_once(input: TokenStream) -> TokenStream {
    fn_once::derive(input)
}

/// Handle derive macro.
#[proc_macro_derive(Handle, attributes(handle))]
pub fn handle(input: TokenStream) -> TokenStream {
    handle::derive(input)
}

/// Transparent derive macro.
#[proc_macro_derive(Transparent)]
pub fn transparent(input: TokenStream) -> TokenStream {
    transparent::derive(input)
}

mod fn_once;
mod handle;
mod transparent;
