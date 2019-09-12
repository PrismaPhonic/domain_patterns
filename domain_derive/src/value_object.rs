use syn::{DeriveInput, Data, Error};
use syn::Ident;
use syn::export::Span;

/// `precondition` checks all invariants for the Struct structure that the macro is being applied to.
/// The following conditions must be true:
/// 1. There needs to be a value field.
pub fn precondition(input: &DeriveInput) -> Result<(), syn::Error> {
    check_value_field(input)?;

    Ok(())
}

fn check_value_field(input: &DeriveInput) -> Result<(), syn::Error> {
    if has_value_field(&input.data) && has_one_field(&input.data) {
        return Ok(());
    }

    let input_span = input.ident.span();
    Err(Error::new(input_span, "expected a struct with a single field named `value`."))
}

fn has_one_field(data: &Data) -> bool {
    match data {
        Data::Struct(st) => {
            st.fields.iter().len() == 1
        },
        _ => false,
    }
}

fn has_value_field(data: &Data) -> bool {
    match data {
        Data::Struct(st) => {
            st.fields.iter().any(|f| {
                f.clone().ident.unwrap() == "value"
            })
        },
        _ => false,
    }
}

// returns the path name of the type used for the value field.
pub fn value_type_name(data: &Data) -> Option<Ident> {
    if let Data::Struct(st) = data {
        let result = st.fields.iter().find(|f| {
            let ident = f.clone().ident.clone().unwrap();
            ident.to_string().contains("value")
        });

        match &result.unwrap().ty {
            syn::Type::Path(type_path) => {
                let path_name = type_path.path.segments.iter().next().unwrap().ident.clone();
                return Some(path_name);
            },
            _ => return None,
        }
    }

    None
}

pub fn error_name_from_type(type_name: &Ident, span: Span) -> Ident {
    let name_str = type_name.to_string();
    let error_name_str = format!("{}ValidationError", name_str);
    Ident::new(&error_name_str, span)
}
