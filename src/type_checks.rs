use syn::{DeriveInput, Data, Field, Path, Error};

pub(crate) fn is_uuid_type(field: &Field) -> bool {
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

pub(crate) fn is_int_type(field: &Field) -> bool {
    fn path_is_int(path: &Path) -> bool {
        let path_str = path.segments.iter().next().unwrap().ident.to_string();
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

pub(crate) fn is_timestamp_type(field: &Field) -> bool {
    fn path_is_i64(path: &Path) -> bool {
        let path_str = path.segments.iter().next().unwrap().ident.to_string();
        &path_str == "u64"
    }
    match &field.ty {
        syn::Type::Path(type_path) if path_is_i64(&type_path.path) => {
            true
        },
        _ => false,
    }
}
