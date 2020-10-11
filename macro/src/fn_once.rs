use proc_macro2::Span;
use quote::quote;
use std::default::Default;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    token::Comma,
    AngleBracketedGenericArguments, Data, DataStruct, DeriveInput, Expr, Field, GenericArgument,
    Ident,
};

pub(super) fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let ident = &derive_input.ident;
    let builder_ident = Ident::new(&format!("{}Builder", derive_input.ident), Span::call_site());
    let builder_optional_ident = Ident::new(
        &format!("{}Builder_Optional", derive_input.ident),
        Span::call_site(),
    );
    let (impl_generics, type_generics, where_clause) = derive_input.generics.split_for_impl();
    let (generic_arguments, fields_argumet) = {
        let mut generic_arguments: Vec<GenericArgument> = Vec::new();
        let mut fields_tuples: Vec<GenericArgument> = Vec::new();
        let fields = match derive_input.data {
            Data::Struct(DataStruct { fields, .. }) => fields,
            _ => unimplemented!(),
        };
        'fields: for Field {
            attrs, ident, ty, ..
        } in fields
        {
            for attr in attrs {
                if attr.path == parse_quote!(builder) {
                    let attributes: Attributes = attr
                        .parse_args_with(Attributes::parse)
                        .expect("Parse attributes.");
                    if attributes.setter.skip {
                        continue 'fields;
                    }
                    if attributes.default {
                        let ident = ident.as_ref().map(|ident| {
                            Ident::new(&format!("__{}", ident.to_string()), ident.span())
                        });
                        generic_arguments.push(parse_quote!(#ident: #builder_optional_ident<#ty>));
                        fields_tuples.push(parse_quote!(#ident));
                        continue 'fields;
                    }
                }
            }
            fields_tuples.push(parse_quote!((#ty,)));
        }
        (
            generic_arguments,
            GenericArgument::Type(parse_quote!((#(#fields_tuples,)*))),
        )
    };
    let (mut impl_generics_arguments, mut type_generic_arguments): (
        AngleBracketedGenericArguments,
        AngleBracketedGenericArguments,
    ) = if !derive_input.generics.params.is_empty() {
        (parse_quote!(#impl_generics), parse_quote!(#type_generics))
    } else {
        (parse_quote!(<>), parse_quote!(<>))
    };
    for generic_argument in generic_arguments {
        impl_generics_arguments.args.push(generic_argument);
    }
    type_generic_arguments.args.insert(0, fields_argumet);
    let tokens = quote! {
        #[allow(non_camel_case_types)]
        impl #impl_generics_arguments std::ops::FnOnce<()> for #builder_ident #type_generic_arguments #where_clause {
            type Output = <#ident #type_generics as std::ops::FnOnce<()>>::Output;

            #[inline]
            extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
                self.build()()
            }
        }
    };
    tokens.into()
}

/// Attributes.
#[derive(Clone, Debug, Default)]
struct Attributes {
    default: bool,
    setter: Setter,
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes: Attributes = Default::default();
        let punctuated = input.parse_terminated::<_, Comma>(Expr::parse)?;
        for expr in punctuated {
            match expr {
                Expr::Assign(expr_assign) if expr_assign.left == parse_quote!(default) => {
                    attributes.default = true;
                }
                Expr::Path(expr_path) if expr_path.path == parse_quote!(default) => {
                    attributes.default = true;
                }
                Expr::Call(expr_call) if expr_call.func == parse_quote!(setter) => {
                    for arg in expr_call.args {
                        if arg == parse_quote!(skip) {
                            attributes.setter.skip = true;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(attributes)
    }
}

/// Setter.
#[derive(Clone, Debug, Default)]
struct Setter {
    skip: bool,
}
