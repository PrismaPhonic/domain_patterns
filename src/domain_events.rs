use syn::{DeriveInput, Data, Field, Path, Error};

/// `precondition` checks all invariants for the Struct structure that the macro is being applied to.
/// The following conditions must be true:
/// 1. The data structure the macro is being applied to must be an enum.
pub fn precondition(input: &DeriveInput) -> Result<(), syn::Error> {
    check_if_enum(input)?;

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
