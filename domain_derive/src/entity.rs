use syn::{DeriveInput, Data, Field, Path, Error, Attribute, DataStruct};
use crate::type_checks::*;
use syn::export::TokenStream2;

/// `precondition` checks all invariants for the Struct structure that the macro is being applied to.
/// The following conditions must be true:
/// 1. There needs to be an `id` field of type `Uuid`.
/// 2. There needs to be a version field of any integer type (floating point not allowed).
pub fn precondition(input: &DeriveInput) -> Result<(), syn::Error> {
    check_id_field(input)?;
    check_version_field(input)?;
    check_public_fields(input)?;

    Ok(())
}

pub fn produce_getters(input: &DeriveInput) -> Result<TokenStream2, syn::Error> {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Is it a struct?
    if let syn::Data::Struct(DataStruct { ref fields, .. }) = input.data {
        let generated = fields
            .iter()
            // We already generate getters for id and version in Entity impl, so don't do it here.
            .filter(|f| {
                let field_name = f.ident.clone().unwrap();
                field_name != "id" && field_name != "version"
            })
            .map(|f| implement_getter(f))
            .collect::<Vec<_>>();

        return Ok(quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #(#generated)*
            }
        })
    };

    let input_span = input.ident.span();
    Err(Error::new(input_span, "cannot produce getters for an enum"))
}

fn implement_getter(field: &Field) -> TokenStream2 {
    let field_name = field
        .clone()
        .ident
        .expect("Expected the field to have a name");

    let ty = field.ty.clone();

    let docs: Vec<&Attribute> = field
        .attrs
        .iter()
        .filter_map(|v| {
            let meta = v.parse_meta().expect("attribute");
            if meta.path().is_ident("doc") {
                return Some(v);
            }
            None
        }).collect();

        return quote! {
            #(#docs)*
            #[inline(always)]
            pub fn #field_name(&self) -> &#ty {
                &self.#field_name
            }
        };
}

fn check_public_fields(input: &DeriveInput) -> Result<(), syn::Error> {
    if has_public_field(&input.data) {
        let input_span = input.ident.span();
        return Err(Error::new(input_span, "cant have any public fields. Interior mutability should be limited to methods only"));
    }

    Ok(())
}

fn has_public_field(data: &Data) -> bool {
    match data {
        Data::Struct(st) => {
            st.fields.iter().any(|f| {
                is_public(f)
            })
        },
        _ => false,
    }
}

fn is_public(field: &Field) -> bool {
    match field.vis {
        syn::Visibility::Public(_) => true,
        _ => false,
    }
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




