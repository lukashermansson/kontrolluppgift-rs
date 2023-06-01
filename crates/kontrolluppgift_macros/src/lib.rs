use proc_macro::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, LitBool, LitByteStr, LitStr, parenthesized, parse_macro_input, Type};
use quote::{quote};
use syn::__private::Span;


#[proc_macro_derive(KontrolluppgiftRead, attributes(ku))]
pub fn read_macro(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = ast.ident;
    let mut str_name: Option<LitStr> = None;
    for attr in &ast.attrs {
        if attr.path().is_ident("ku") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let content;
                    parenthesized!(content in meta.input);
                    let lit: LitStr = content.parse()?;
                    str_name = Some(lit);
                    return Ok(());
                }

                Err(meta.error("no name provided"))
            }).expect("Error");
        }
    };
    let str_name = str_name.expect("expected to have a name attribute on the struct");

    let fields = match ast.data {
        | Data::Enum(_)
        | Data::Union(_) => {
            panic!("Only structs are suported")
        }
        | Data::Struct(DataStruct { fields: Fields::Named(it), .. }) => it,
        | Data::Struct(_) => {
            panic!("Not a named struct")
        }
    };


    let field_name: Vec<(LitByteStr, Ident, Ident, Type, bool, Option<LitStr>, bool)> = fields.named.into_iter().enumerate().map(|(index, field)| {
        let mut name = None;
        let mut code = None;
        let mut required = false;
        let mut is_inner_typ = false;
        for attr in &field.attrs {
            if attr.path().is_ident("ku") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let content;
                        parenthesized!(content in meta.input);
                        let lit: LitByteStr = content.parse()?;
                        name = Some(lit);
                        return Ok(());
                    }
                    if meta.path.is_ident("code") {
                        let content;
                        parenthesized!(content in meta.input);
                        let lit: LitStr = content.parse()?;
                        code = Some(lit);
                        return Ok(());
                    }
                    if meta.path.is_ident("required") {
                        let content;
                        parenthesized!(content in meta.input);
                        let _: LitBool = content.parse()?;
                        required = true;
                        return Ok(());
                    }
                    if meta.path.is_ident("inner_ty") {
                        let content;
                        parenthesized!(content in meta.input);
                        let _: LitBool = content.parse()?;
                        is_inner_typ = true;
                        return Ok(());
                    }
                    Err(meta.error("unrecognized ku attrs"))
                }).unwrap();
            }
        }

        (name.expect("Requires name"), Ident::new(&format!("f_{}", index), Span::call_site()), field.ident.expect("its not an ident"), field.ty, required, code, is_inner_typ)
    }).collect();

    let struct_assignments: Vec<_> = field_name.iter().map(|(names, temp, ogs, _, req, _, _)| {
        if *req {
            let str = String::from_utf8(names.value()).unwrap();
            quote! {
               #ogs: #temp.ok_or_else(|| crate::Error::MissingElement(#str.to_string()))?,
           }
        } else {
            quote! {
               #ogs: #temp,
           }
        }
    }).collect();
    let match_brances: Vec<_> = field_name.iter().map(|(names, temp, _, typ, _, codes, is_inner)| {
        if *is_inner {
            let Type::Path(type_path) = typ.clone() else { panic!("expected path") };
            let t = &type_path.path.segments.last().unwrap().ident;
            quote! {
               #names => { #temp = Some(#t::read(reader, &element)?) },
           }
        }else {
            if codes.is_some() {
                quote! {
                    #names => reader.read_node_into_with_code(element, #codes, &mut #temp)?,
                }
            } else {
                quote! {
                    #names => reader.read_node_into(element, &mut #temp)?,
                }
            }
        }

    }).collect();

    let variable_definitions: Vec<_> = field_name.iter().map(|(_, temp, _, _, _, _, _)| {
        quote! {
           let mut #temp = None;
      }
    }).collect();


    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl<'a> crate::KontrolluppgiftRead<'a> for #name<'a> {
            fn read(reader: &mut quick_xml::NsReader<&'a [u8]>, tag: &quick_xml::events::BytesStart) -> Result<Self, crate::error::Error> {
                #(
                    #variable_definitions
                )*
                loop {
                    use crate::Reader;
                    match reader.read_event()? {
                       quick_xml::events::Event::Start(element) => match element.local_name().as_ref() {
                            #(
                                #match_brances
                            )*
                            &_ => crate::unexpected_element(&element)?
                    }
                    quick_xml::events::Event::End(element) => {
                        if element.name() == tag.name() {
                            return Ok(Self {
                             #(
                                #struct_assignments
                            )*
                            });
                        }
                    }
                    quick_xml::events::Event::Eof => return Err(crate::error::Error::UnexpectedEof(format!("While reading {}", #str_name).to_string())),
                    _ => {}
                    }
                }

            }
        }
    };
    let tokens = TokenStream::from(expanded);

    tokens
}

#[proc_macro_derive(KontrolluppgiftWrite, attributes(ku))]
pub fn write_macro(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = ast.ident;
    let mut str_name: Option<LitStr> = None;
    for attr in &ast.attrs {
        if attr.path().is_ident("ku") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let content;
                    parenthesized!(content in meta.input);
                    let lit: LitStr = content.parse()?;
                    str_name = Some(lit);
                    return Ok(());
                }

                Err(meta.error("no name provided"))
            }).expect("Error");
        }
    };
    let str_name = str_name.expect("expected to have a name attribute on the struct");

    let fields = match ast.data {
        | Data::Enum(_) | Data::Union(_) => {
            panic!("Only structs are suported")
        }
        | Data::Struct(DataStruct { fields: Fields::Named(it), .. }) => it,
        | Data::Struct(_) => {
            panic!("Not a named struct")
        }
    };


    let field_name: Vec<(String, Ident, Option<LitStr>, bool)> = fields.named.into_iter().map(|field| {
        let mut name: Option<String> = None;
        let mut code = None;
        let mut is_inner_type = false;
        for attr in &field.attrs {
            if attr.path().is_ident("ku") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let content;
                        parenthesized!(content in meta.input);
                        let lit: LitByteStr = content.parse()?;
                        name = Some(String::from_utf8(lit.value()).unwrap());
                        return Ok(());
                    }
                    if meta.path.is_ident("code") {
                        let content;
                        parenthesized!(content in meta.input);
                        let lit: LitStr = content.parse()?;
                        code = Some(lit);
                        return Ok(());
                    }
                    if meta.path.is_ident("required") {
                        let content;
                        parenthesized!(content in meta.input);
                        let _: LitBool = content.parse()?;
                        return Ok(());
                    }
                    if meta.path.is_ident("inner_ty") {
                        let content;
                        parenthesized!(content in meta.input);
                        let _: LitBool = content.parse()?;
                        is_inner_type = true;
                        return Ok(());
                    }
                    Err(meta.error("unrecognized ku attribute"))
                }).unwrap();
            }
        }

        (name.unwrap(), field.ident.unwrap(), code, is_inner_type)
    }).collect();

    let write_operations: Vec<_> = field_name.iter().map(|(name, og, code, is_inner_type)| {
        match code {
            None => {
                if *is_inner_type {
                    quote! {
                        self.#og.write(w)?;
                    }
                }else {
                    quote! {
                        w.write_node(#name, &self.#og)?;
                    }
                }
            }
            Some(code) => {
                quote! {
                w.write_node_with_code(#name, #code, &self.#og)?;
           }
            }
        }
    }).collect();

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl<'a> crate::KontrolluppgiftWrite for #name<'a> {
            fn write<W>(&self, w: &mut crate::Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
                w.create_element(#str_name).write_inner_content(|w| {
                        use crate::Write;
                            #(
                               #write_operations
                            )*

                    Ok(())
                })?;

                Ok(())
            }
        }
    };
    let tokens = TokenStream::from(expanded);

    tokens
}
