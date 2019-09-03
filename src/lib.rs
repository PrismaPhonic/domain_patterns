//! This crate provides `domain_patterns` derive macros.
//!
//! # Entity macro
//! The `Entity` derive macro can be used to automatically implement all methods of the `Entity` trait
//! from the `domain_patterns` crate.  This only works if certain preconditions are met:
//!
//! 1. You are applying this to a struct.
//! 2. Your struct has an `id` field of type `Uuid`.
//! 3. Your struct has a `version` field which is some integer type.
//!
//! ```edition2018
//! #[macro_use]
//! extern crate domain_derive;
//! use uuid::Uuid;
//!
//! #[derive(Entity)]
//! struct User {
//!     id: Uuid,
//!     version: u64
//! };
//! ```

#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod entity;
mod value_object;

use crate::proc_macro::TokenStream;
use syn::DeriveInput;
use syn::spanned::Spanned;

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

#[proc_macro_derive(ValueSetup)]
pub fn value_object_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // Struct name
    let name = &input.ident;

    value_object::precondition(&input).expect("ValueSetup macro failed preconditions");

    // safe to unwrap because we check for existence of value field in precondition.
    let type_name = &value_object::value_type_name(&input.data).unwrap();

    let error_struct_name = &value_object::error_name_from_type(name, input.span());

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        #[derive(Debug)]
        pub struct #error_struct_name;

        impl std::fmt::Display for #error_struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "Email failed to validate.")
            }
        }

        impl std::error::Error for #error_struct_name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                None
            }
        }

        impl TryFrom<#type_name> for #name {
            type Error = #error_struct_name;

            fn try_from(value: #type_name) -> Result<Self, Self::Error> {
                if !Self::validate(&value) {
                    return Err(#error_struct_name)
                }

                Ok(#name {
                    value,
                })
            }
        }
    };

    TokenStream::from(expanded)
}
