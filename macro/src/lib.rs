use proc_macro::TokenStream;

/// Handle derive macro.
#[proc_macro_derive(Handle, attributes(handle))]
pub fn handle(input: TokenStream) -> TokenStream {
    handle::derive(input)
}

mod handle;
