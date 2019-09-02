use syn::{DeriveInput, Data, Field, Path, Error};

/// `precondition` checks all invariants for the Struct structure that the macro is being applied to.
/// The following conditions must be true:
/// 1. There needs to be an `id` field of type `Uuid`.
/// 2. There needs to be a version field of any integer type (floating point not allowed).
pub fn precondition(input: &DeriveInput) -> Result<(), syn::Error> {
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
