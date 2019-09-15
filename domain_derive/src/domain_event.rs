use syn::{DeriveInput, Data, Field, Path, Error};
use crate::type_checks::*;

/// `precondition` checks all invariants for the Struct structure that the macro is being applied to.
/// The following conditions must be true:
/// 1. There needs to be an `id` field of type `Uuid`.
/// 2. There needs to be a version field of any integer type (floating point not allowed).
/// 3. There needs to be an `aggregate_id` field of type `Uuid`.
/// 4. There needs to be an `occurred` field of type `i64`.
pub fn precondition(input: &DeriveInput) -> Result<(), syn::Error> {
    check_id_field(input)?;
    check_aggregate_id_field(input)?;
    check_occurred_field(input)?;
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

fn has_id_field(data: &Data) -> bool {
    match data {
        Data::Struct(st) => {
            st.fields.iter().any(|f| {
                f.clone().ident.unwrap() == "id"
            })
        },
        _ => false,
    }
}

fn check_aggregate_id_field(input: &DeriveInput) -> Result<(), syn::Error> {
    if !has_id_field(&input.data) {
        let input_span = input.ident.span();
        return Err(Error::new(input_span, "expected `aggregate_id` field with type Uuid"));
    }

    Ok(())
}

fn has_aggregate_id_field(data: &Data) -> bool {
    match data {
        Data::Struct(st) => {
            st.fields.iter().any(|f| {
                f.clone().ident.unwrap() == "aggregate_id"
                    && is_uuid_type(f)
            })
        },
        _ => false,
    }
}

fn check_occurred_field(input: &DeriveInput) -> Result<(), syn::Error> {
    if !has_id_field(&input.data) {
        let input_span = input.ident.span();
        return Err(Error::new(input_span, "expected `occurred` field with type i64"));
    }

    Ok(())
}

fn has_occurred_field(data: &Data) -> bool {
    match data {
        Data::Struct(st) => {
            st.fields.iter().any(|f| {
                f.clone().ident.unwrap() == "occurred"
                    && is_timestamp_type(f)
            })
        },
        _ => false,
    }
}

fn check_version_field(input: &DeriveInput) -> Result<(), syn::Error> {
    if !has_version_field(&input.data) {
        let input_span = input.ident.span();
        return Err(Error::new(input_span, "expected `version` field with integer type."));
    }

    Ok(())
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
