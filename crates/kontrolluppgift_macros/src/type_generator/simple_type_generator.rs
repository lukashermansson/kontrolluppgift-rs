use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use super::{parser_serde::SimpleType, convert_name};

pub fn generate_simple_types(types: Vec<SimpleType>) -> TokenStream {
    types
        .iter()
        .filter_map(|t| {
            if t.restriction
                .as_ref()
                .is_some_and(|v| v.enumeration.len() > 0)
            {
                return Some(generate_enumeration(t));
            } else {
                return match t.restriction.as_ref().map(|f| f.base.as_str()) {
                    Some("xs:string") => Some(generate_string_newtype(t)),
                    Some("xs:int") => Some(generate_int_newtype(t)),
                    Some("xs:integer") => Some(generate_long_newtype(t)), // not to spec, but might be enough
                    Some("xs:long") => Some(generate_long_newtype(t)),
                    Some("xs:decimal") => Some(generate_decimal_newtype(t)),
                    Some("xs:boolean") => Some(generate_boolean_newtype(t)),
                    Some("xs:dateTime") => Some(generate_datetime_newtype(t)),
                    None => None,
                    _=> panic!("Unexpected base encountered")
                };
            }
        })
        .collect::<TokenStream>()
}
fn generate_string_newtype(ty: &SimpleType) -> TokenStream {
    let name = Ident::new(&ty.name.clone(), Span::call_site());
    let lenght = ty
        .restriction
        .as_ref()
        .map(|v| v.length.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::usize_suffixed(v.value.parse().unwrap());
            quote! { if(value.len() != #val) {
                return Err(format!("invalid lenght, expected: {}, got: {}", #val, value.len()))
            } }
        });
    let min_lenght = ty
        .restriction
        .as_ref()
        .map(|v| v.min_length.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::usize_suffixed(v.value.parse().unwrap());
            quote! { if(value.len() <= #val) {
                return Err(format!("invalid lenght, expected atleast {}, got: {}", #val, value.len()))
            } }
        });
    let max_lenght = ty
        .restriction
        .as_ref()
        .map(|v| v.max_length.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::usize_suffixed(v.value.parse().unwrap());
            quote! { if(value.len() >= #val) {
                return Err(format!("invalid lenght, expected at most {}, got: {}", #val, value.len()))
            } }
        });
    let pattern = ty
        .restriction
        .as_ref()
        .map(|v| v.pattern.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::string(&v.value);
            quote! { 
                let re = regex::Regex::new(#val).unwrap();
                if(!re.is_match(value)) {
                return Err(format!("invalid input must match pattern: {}, with value: {}", #val, value))
            } }
        });

    quote!(
            #[derive(Debug, PartialEq)]
    pub struct #name<'a> (Cow<'a, str>);

    impl<'a, 'b: 'a> Readable<'a, 'b> for #name<'a> {
        fn get_str(data: Cow<'b, str>) -> Result<Self, Error> {
            data.as_ref()
                .try_into()
                .map_err(|e: &str| Error::UnexpectedToken(e.to_string()))
        }
    }

    impl TryFrom<&str> for #name<'_> {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            #lenght
            #min_lenght
            #max_lenght
            #pattern

            return Ok(#name (Cow::Owned(value.to_string())));
        }
    }

    impl From<&#name<'_>> for String {
        fn from(value: &#name) -> Self {
            value.0.to_string()
        }
    }

    impl Writable for #name<'_> {
        fn get_str(&self) -> Option<String> {
            Some(self.0.to_string())
        }
    }
    )
    .into()
}

fn generate_datetime_newtype(ty: &SimpleType) -> TokenStream {
    let name = Ident::new(convert_name(&ty.name).as_ref(), Span::call_site());

    quote!(
            #[derive(Debug, PartialEq)]
    pub struct #name (chrono::DateTime<chrono::offset::Utc>);

    impl<'a, 'b: 'a> Readable<'a, 'b> for #name {
        fn get_str(data: Cow<str>) -> Result<Self, Error> {
            data.as_ref()
                .try_into()
                .map_err(|e: &str| Error::UnexpectedToken(e.to_string()))
        }
    }

    impl TryFrom<&str> for #name {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let value = DateTime::parse_from_rfc3339(value).map_err("Cant parse value as datetime")?;

            return Ok(#name (value));
        }
    }
    impl From<chrono::DateTime<chrono::offset::Utc>> for #name {

        fn from(value: chrono::DateTime<chrono::offset::Utc>) -> Self {
            return #name(value);
        }
    }

    impl From<&#name> for String {
        fn from(value: &#name) -> Self {
            value.0.to_string()
        }
    }

    impl Writable for #name {
        fn get_str(&self) -> Option<String> {
            Some(self.0.to_rfc3339())
        }
    }
    )
    .into()
}
fn generate_boolean_newtype(ty: &SimpleType) -> TokenStream {
    let name = Ident::new(convert_name(&ty.name).as_ref(), Span::call_site());

    quote!(
            #[derive(Debug, PartialEq)]
    pub struct #name (bool);

    impl<'a, 'b: 'a> Readable<'a, 'b> for #name {
        fn get_str(data: Cow<str>) -> Result<Self, Error> {
            data.as_ref()
                .try_into()
                .map_err(|e: &str| Error::UnexpectedToken(e.to_string()))
        }
    }

    impl TryFrom<&str> for #name {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let value = value::parse().map_err("Cant parse value as boolean")?;

            return Ok(#name (value));
        }
    }

    impl From<&#name> for String {
        fn from(value: &#name) -> Self {
            value.0.to_string()
        }
    }

    impl Writable for #name {
        fn get_str(&self) -> Option<String> {
            Some(self.0.to_string())
        }
    }
    )
    .into()
}
fn generate_decimal_newtype(ty: &SimpleType) -> TokenStream {
    let name = Ident::new(convert_name(&ty.name).as_ref(), Span::call_site());

    let min_inclusive = ty
        .restriction
        .as_ref()
        .map(|v| v.min_inclusive.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::f64_unsuffixed(v.value.parse().unwrap());
            quote! { if(value <= #val) {
                return Err(format!("invalid lenght, expected atleast {}, got: {}", #val, value.len()))
            } }
        });
    let max_inclusive = ty
        .restriction
        .as_ref()
        .map(|v| v.max_inclusive.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::f64_unsuffixed(v.value.parse().unwrap());
            quote! { if(value >= #val) {
                return Err(format!("invalid lenght, expected at most {}, got: {}", #val, value.len()))
            } }
        });
    let pattern = ty
        .restriction
        .as_ref()
        .map(|v| v.pattern.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::string(&v.value);
            quote! { 
                let re = regex::Regex::new(#val).unwrap();
                if(!re.is_match(value.to_string())) {
                return Err(format!("invalid inout must match pattern: {}, with value: {}", #val, value))
            } }
        });

    quote!(
            #[derive(Debug, PartialEq)]
    pub struct #name (f64);

    impl<'a, 'b: 'a> Readable<'a, 'b> for #name {
        fn get_str(data: Cow<str>) -> Result<Self, Error> {
            data.as_ref()
                .try_into()
                .map_err(|e: &str| Error::UnexpectedToken(e.to_string()))
        }
    }

    impl TryFrom<&str> for #name {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let value = value::parse().map_err("Cant parse value as long")?;
            #min_inclusive
            #max_inclusive
            #pattern

            return Ok(#name (value));
        }
    }

    impl From<&#name> for String {
        fn from(value: &#name) -> Self {
            value.0.to_string()
        }
    }

    impl Writable for #name {
        fn get_str(&self) -> Option<String> {
            Some(self.0.to_string())
        }
    }
    )
    .into()
}
fn generate_long_newtype(ty: &SimpleType) -> TokenStream {
    let name = Ident::new(&ty.name.clone(), Span::call_site());

    let min_inclusive = ty
        .restriction
        .as_ref()
        .map(|v| v.min_inclusive.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::i64_unsuffixed(v.value.parse().unwrap());
            quote! { if(value <= #val) {
                return Err(format!("invalid lenght, expected atleast {}, got: {}", #val, value.len()))
            } }
        });
    let max_inclusive = ty
        .restriction
        .as_ref()
        .map(|v| v.max_inclusive.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::i64_unsuffixed(v.value.parse().unwrap());
            quote! { if(value >= #val) {
                return Err(format!("invalid lenght, expected at most {}, got: {}", #val, value.len()))
            } }
        });
    let pattern = ty
        .restriction
        .as_ref()
        .map(|v| v.pattern.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::string(&v.value);
            quote! { 
                let re = regex::Regex::new(#val).unwrap();
                if(!re.is_match(value.to_string())) {
                return Err(format!("invalid inout must match pattern: {}, with value: {}", #val, value))
            } }
        });

    quote!(
            #[derive(Debug, PartialEq)]
    pub struct #name (i64);

    impl<'a, 'b: 'a> Readable<'a, 'b> for #name {
        fn get_str(data: Cow<str>) -> Result<Self, Error> {
            data.as_ref()
                .try_into()
                .map_err(|e: &str| Error::UnexpectedToken(e.to_string()))
        }
    }

    impl TryFrom<&str> for #name {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let value = value::parse().map_err("Cant parse value as long")?;
            #min_inclusive
            #max_inclusive
            #pattern

            return Ok(#name (value));
        }
    }

    impl From<&#name> for String {
        fn from(value: &#name) -> Self {
            value.0.to_string()
        }
    }

    impl Writable for #name {
        fn get_str(&self) -> Option<String> {
            Some(self.0.to_string())
        }
    }
    )
    .into()
}
fn generate_int_newtype(ty: &SimpleType) -> TokenStream {
    let name = Ident::new(&ty.name.clone(), Span::call_site());

    let min_inclusive = ty
        .restriction
        .as_ref()
        .map(|v| v.min_inclusive.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::i32_unsuffixed(v.value.parse().unwrap());
            quote! { if(value <= #val) {
                return Err(format!("invalid lenght, expected atleast {}, got: {}", #val, value.len()))
            } }
        });
    let max_inclusive = ty
        .restriction
        .as_ref()
        .map(|v| v.max_inclusive.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::i32_unsuffixed(v.value.parse().unwrap());
            quote! { if(value >= #val) {
                return Err(format!("invalid lenght, expected at most {}, got: {}", #val, value.len()))
            } }
        });
    let pattern = ty
        .restriction
        .as_ref()
        .map(|v| v.pattern.as_ref())
        .flatten()
        .map(|v| {
            let val = Literal::string(&v.value);
            quote! { 
                let re = regex::Regex::new(#val).unwrap();
                if(!re.is_match(value.to_string())) {
                return Err(format!("invalid inout must match pattern: {}, with value: {}", #val, value))
            } }
        });

    quote!(
            #[derive(Debug, PartialEq)]
    pub struct #name (i32);

    impl<'a, 'b: 'a> Readable<'a, 'b> for #name {
        fn get_str(data: Cow<str>) -> Result<Self, Error> {
            data.as_ref()
                .try_into()
                .map_err(|e: &str| Error::UnexpectedToken(e.to_string()))
        }
    }

    impl TryFrom<&str> for #name {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let value = value::parse().map_err("Cant parse value as integer");
            #min_inclusive
            #max_inclusive
            #pattern

            return Ok(#name (value));
        }
    }

    impl From<&#name> for String {
        fn from(value: &#name) -> Self {
            value.0.to_string()
        }
    }

    impl Writable for #name {
        fn get_str(&self) -> Option<String> {
            Some(self.0.to_string())
        }
    }
    )
    .into()
}
fn generate_enumeration(ty: &SimpleType) -> TokenStream {
    let name = Ident::new(&ty.name, Span::call_site());
    let restriction = ty.restriction.as_ref().unwrap();
    let strings: Vec<_> = restriction.enumeration.iter().map(|s| &s.value).collect();
    let enums: Vec<(_, _)> = restriction
        .enumeration
        .iter()
        .map(|e| {
            if e.value.starts_with(|d: char| d.is_digit(10)) {
                let fname = format!("E{}", e.value);
                return (Ident::new(&fname, Span::call_site()), &e.annotation);
            }
            return (Ident::new(&e.value, Span::call_site()), &e.annotation);
        })
        .collect();
    let enum_names: Vec<_> = enums.iter().map(|f| &f.0).collect();
    let definitions: Vec<_> = enums
        .iter()
        .map(|e| {
            let value = &e.0;
            if let Some(documentation) = &e.1 {
                let documentation = &documentation.documentation.text;
                quote!(
                    #[doc=#documentation]
                    #value
                )
            } else {
                quote!(
                    #value
                )
            }
        })
        .collect();

    quote!(
        pub enum #name {
            #(#definitions),*
        }

        impl<'a, 'b> crate::Readable<'a, 'b> for #name {
            fn get_str(data: Cow<str>) -> Result<Self, error::Error> {
                match data.as_ref() {
                            #(#strings => Ok(#name::#enum_names),)*
                    &_ => Err(error::Error::UnexpectedToken(format!("expected enumerated value got: {}", &data)))
                }
            }
        }

        impl crate::Writable for #name {
            fn get_str(&self) -> Option<String> {
                Some(match *self {
                    #(#name::#enum_names => #strings.to_string(),)*
                })
            }
        }

         impl std::convert::TryFrom<String> for #name {
            type Error = ();

            fn try_from(value: String) -> Result<Self, Self::Error> {
              match value.as_ref() {
                    #(#strings => Ok(#name::#enum_names),)*
                    _ => Err(())
                }
            }
        }
        impl std::convert::TryFrom<&str> for #name {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, Self::Error> {
              match value {
                    #(#strings => Ok(#name::#enum_names),)*
                    _ => Err(())
                }
            }
        }
    ).into()
}
