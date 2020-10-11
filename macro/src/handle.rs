use darling::{
    ast::{Data, Style},
    FromDeriveInput,
};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Ident};

pub(super) fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let attribute: Attribute =
        FromDeriveInput::from_derive_input(&derive_input).expect(r#"can't parse attribute"#);
    let ident = &attribute.ident;
    let fields = &attribute.data.take_struct().unwrap();
    assert!(
        fields.style == Style::Tuple && fields.fields.len() == 1,
        "Handle derive supports only tuple structures with a single field",
    );
    let r#type = &fields.fields[0].ty;
    let tokens = quote! {
        impl std::os::windows::io::AsRawHandle for #ident {
            fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
                self.0 as _
            }
        }

        impl crate::utils::AsStrictRawHandle for #ident {
            fn as_strict_raw_handle(&self) -> Self::StrictRawHandle {
                self.0
            }
        }

        impl std::os::windows::io::FromRawHandle for #ident {
            unsafe fn from_raw_handle(handle: std::os::windows::io::RawHandle) -> Self {
                Self(handle as _)
            }
        }

        impl crate::utils::FromStrictRawHandle for #ident {
            type StrictRawHandle = #r#type;

            unsafe fn from_strict_raw_handle(handle: Self::StrictRawHandle) -> Self {
                Self(handle)
            }
        }

        impl std::os::windows::io::IntoRawHandle for #ident {
            fn into_raw_handle(self) -> std::os::windows::io::RawHandle {
                let handle = self.0;
                std::mem::forget(self);
                handle as _
            }
        }

        impl crate::utils::IntoStrictRawHandle for #ident {
            fn into_strict_raw_handle(self) -> Self::StrictRawHandle {
                let handle = self.0;
                std::mem::forget(self);
                handle
            }
        }
    };
    tokens.into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(handle))]
struct Attribute {
    ident: Ident,
    data: Data<(), Field>,
}
