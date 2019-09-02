#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use crate::proc_macro::TokenStream;
use syn::{DeriveInput, Data, Field, Path, Type};
use proc_macro::Ident;

#[proc_macro_derive(Entity)]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    precondition(&input).expect("Entity procedural macro failed preconditions");

    // type name
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

use syn::Error;
use syn::parse::ParseStream;

fn precondition(input: &DeriveInput) -> Result<(), syn::Error> {
    check_id_field(input)?;
    check_version_field(input)?;

    Ok(())
}

fn check_id_field(input: &DeriveInput) -> Result<(), syn::Error> {
    if !has_id_field(&input.data) {
        let input_span = input.ident.span();
        return Err(Error::new(input_span, "expected `id` field with type Uuid"));
    }

    Ok(())
}

fn check_version_field(input: &DeriveInput) -> Result<(), syn::Error> {
    if !has_version_field(&input.data) {
        let input_span = input.ident.span();
        return Err(Error::new(input_span, "expected `version` field with integer type."));
    }

    Ok(())
}

fn has_id_field(data: &Data) -> bool {
    match data {
        Data::Struct(st) => {
            st.fields.iter().any(|f| {
                f.clone().ident.unwrap() == "id"
                     && is_uuid_type(f)
            })
        },
        _ => false,
    }
}

fn has_version_field(data: &Data) -> bool {
    match data {
        Data::Struct(st) => {
            st.fields.iter().any(|f| {
                f.clone().ident.unwrap() == "version"
                    && is_int_type(f)
            })
        },
        _ => false,
    }
}



fn is_uuid_type(field: &Field) -> bool {
    fn path_is_uuid(path: &Path) -> bool {
        path.segments.iter().next().unwrap().ident.to_string().to_lowercase().contains("uuid")
    }
    match &field.ty {
        syn::Type::Path(type_path) if path_is_uuid(&type_path.path) => {
            true
        },
        _ => false,
    }
}

fn is_int_type(field: &Field) -> bool {
    fn path_is_int(path: &Path) -> bool {
        let path_str = path.segments.iter().next().unwrap().ident.to_string();
        println!("{}", path_str);
        &path_str == "u128"
            || &path_str == "u64"
            || &path_str == "u32"
            || &path_str == "u16"
            || &path_str == "u8"
            || &path_str == "i128"
            || &path_str == "i64"
            || &path_str == "i32"
            || &path_str == "i16"
            || &path_str == "i8"
    }
    match &field.ty {
        syn::Type::Path(type_path) if path_is_int(&type_path.path) => {
            true
        },
        _ => false,
    }
}
