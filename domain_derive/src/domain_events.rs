use syn::{DeriveInput, Data, Field, Path, Error, Ident};
use syn::export::TokenStream2;
use std::process::abort;
use proc_macro2::Span;
use quote::quote;


/// `precondition` checks all invariants for the Struct structure that the macro is being applied to.
/// The following conditions must be true:
/// 1. The data structure the macro is being applied to must be an enum.
pub fn precondition(input: &DeriveInput) -> Result<(), syn::Error> {
    check_if_enum(input)?;
    // TODO: Add check that all enum variants implement DomainEvent

    Ok(())
}

fn check_if_enum(input: &DeriveInput) -> Result<(), syn::Error> {
    if !is_enum(&input.data) {
        let input_span = input.ident.span();
        return Err(Error::new(input_span, "expected data structure to be an enum"));
    }

    Ok(())
}

fn is_enum(data: &Data) -> bool {
    match data {
        Data::Enum(_) => true,
        _ => false,
    }
}

// For use by calling inner getters that take no arguments.  Don't try to use for other things
// in current state.
pub fn create_inner_match_for_getter(input: &DeriveInput, func_name: String) -> TokenStream2 {
    let parent = &input.ident;
    let func = Ident::new(&func_name, Span::call_site());
    let variants = match &input.data {
        syn::Data::Enum(e) => &e.variants,
        _ => abort(),
    };

    let arms = variants.iter()
        .map(|v| &v.ident )
        .map(|name| quote! {
                #parent::#name(child) => child.#func(),
            })
        .collect::<Vec<_>>();

    return quote! {
        match self {
            #(#arms)*
        }
    };
}
