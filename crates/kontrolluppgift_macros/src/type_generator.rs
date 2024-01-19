mod complex_type_generator;
mod parser_serde;
mod simple_type_generator;

use std::borrow::Cow;

use crate::type_generator::simple_type_generator::generate_simple_types;

use self::{complex_type_generator::generate_complex_types, parser_serde::parse_types};

pub fn convert_name(str: &str) -> Cow<str> {
    return if str.contains("-") {
        Cow::Owned(str.replace("-", "to"))
    } else {
        Cow::Borrowed(str)
    };
}

pub fn generate_types(string: &str) -> proc_macro2::TokenStream {
    let types = parse_types(string);
    let simple_types = generate_simple_types(types.simple_type);
    let complex_types = generate_complex_types(types.complex_type, types.element);
    return simple_types.into_iter().chain(complex_types).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let string = include_str!("Kontrolluppgifter_COMPONENT_9.0.xsd");
        println!("{:#}", generate_types(string));
    }

    #[test]
    fn test_2() {
        let string = include_str!("SKV_COMMON_9.0.xsd");
        println!("{:#}", generate_types(string));
    }
}
