use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

pub(super) fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let ident = &derive_input.ident;
    let (impl_generics, type_generics, where_clause) = derive_input.generics.split_for_impl();
    let inner_type = match derive_input.data {
        Data::Struct(DataStruct { fields, .. }) if fields.len() == 1 => {
            fields.into_iter().next().unwrap().ty
        }
        _ => unimplemented!(),
    };
    let tokens = quote! {
        impl #impl_generics crate::utils::Transparent for #ident #type_generics #where_clause {
            type Target = #inner_type;
        }

        impl crate::utils::Transparent for #inner_type {
            type Target = #inner_type;
        }
    };
    tokens.into()
}
