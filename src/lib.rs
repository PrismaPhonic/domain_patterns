#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod entity;

use crate::proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Entity)]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    entity::precondition(&input).expect("Entity procedural macro failed preconditions");

    // Struct name
    let name = &input.ident;

    let expanded = quote! {
        impl domain_patterns::models::Entity for #name {
            fn id(&self) -> Uuid {
                self.id.clone()
            }

            fn version(&self) -> u64 {
                self.version as u64
            }
        }
    };

    TokenStream::from(expanded)
}