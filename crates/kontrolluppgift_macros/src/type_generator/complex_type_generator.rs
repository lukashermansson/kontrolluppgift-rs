use proc_macro2::{TokenStream, Ident, Span};
use quote::quote;
use super::parser_serde::{ComplexType, ElementDef};


pub fn generate_complex_types(types: Vec<ComplexType>, elements: Vec<ElementDef>) -> TokenStream {

    types
        .iter()
        .filter_map(|t| {
          Some(generate_complex_type(t, &elements))
        })
        .collect::<TokenStream>()
}
pub fn generate_complex_type(ty: &ComplexType, elements: &Vec<ElementDef>) -> TokenStream {
    dbg!(ty);
    let name = Ident::new(&ty.name, Span::call_site());

    let elements: Vec<_> = ty.all.as_ref().map(|f| f.element.iter().map(|f| {
        (f.name.as_ref().unwrap_or_else(|| &lookup_element_by_ref(elements, f.reference.as_ref().unwrap()).name),
        f.ty.as_ref().unwrap_or_else(|| &lookup_element_by_ref(elements, f.reference.as_ref().unwrap()).ty.as_ref().unwrap())
     )
    }).collect()).unwrap_or_default();

    let names : Vec<_>= elements.iter().map(|f| Ident::new(f.0, Span::call_site())).collect();
    let types : Vec<_>= elements.iter().map(|f| Ident::new(&f.1.replace("gm:", ""), Span::call_site())).collect();
    if (names.len() > 0) {
    quote!(
        pub struct #name<'a> {
            #(
                   pub #types<'a> #names;
                )*
        }

    )
} else {
    quote!()
}
}

fn lookup_element_by_ref<'a>(elements: &'a Vec<ElementDef>, reference: &'_ str) -> &'a ElementDef {

    dbg!(elements.iter().find(|f| f.name == reference).unwrap())
}