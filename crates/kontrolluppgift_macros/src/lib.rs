use proc_macro::{TokenStream};
use syn::{Attribute, Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Ident, LitBool, LitByteStr, LitStr, parenthesized, parse_macro_input, Type};
use quote::{quote};
use syn::__private::Span;
use syn::spanned::Spanned;


#[proc_macro_derive(KontrolluppgiftRead, attributes(ku))]
pub fn read_macro(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = ast.ident;
    let str_name = match parse_name_input(name.span(), ast.attrs) {
        Ok(n) => {
            n
        }
        Err(e) => {
            return e.into_compile_error().into();
        }
    };

    let fields = get_fields(ast.data);

    let field_data: Result<Vec<(LitByteStr, Ident, Ident, Type, bool, Option<LitStr>, bool)>, Error> = fields.named.into_iter().enumerate().map(|(index, field)| {
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
                    Err(meta.error("unrecognized ku attributes"))
                })?;
            }
        }
        let ident = field.ident.clone().ok_or_else(|| Error::new(field.span(), "Expected a named identifier"))?;
        let name = name.ok_or_else(|| Error::new(field.span(), "must have a name ku(name=\"\")"))?;
        Ok((name,
            Ident::new(&format!("f_{}", index), ident.span()),
            ident,
            field.ty,
            required,
            code,
            is_inner_typ))
    }).collect();

    match field_data {
        Err(err) => {
            err.to_compile_error().into()
        }
        Ok(field) => {
            let struct_assignments: Vec<_> = field.iter().map(|(name, temp, og, _, req, _, _)| {
                if *req {
                    let str = String::from_utf8(name.value()).expect("Expected name to contain valid utf-8");
                    quote! {
                        #og: #temp.ok_or_else(|| crate::Error::MissingElement(#str.into()))?,
                    }
                } else {
                    quote! {
                        #og: #temp,
                    }
                }
            }).collect();

            let match_branches: Vec<_> = field.iter().map(|(name, temp, _, typ, _, code, is_inner)| {
                if *is_inner {
                    let Type::Path(type_path) = typ.clone() else { panic!("expected path") };
                    let type_name = &type_path.path.segments.last().unwrap().ident;
                    quote! {
                        #name => { #temp = Some(#type_name::read(reader, &element)?) },
                    }
                } else {
                    if code.is_some() {
                        quote! {
                            #name => reader.read_node_into_with_code(element, #code, &mut #temp)?,
                        }
                    } else {
                        quote! {
                            #name => reader.read_node_into(element, &mut #temp)?,
                        }
                    }
                }
            }).collect();

            let variable_definitions: Vec<_> = field.iter().map(|(_, temp, _, _, _, _, _)| {
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
                                        #match_branches
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
                                quick_xml::events::Event::Eof => return Err(crate::error::Error::UnexpectedEof(format!("While reading {}", #str_name).into())),
                                _ => {}
                            }
                        }
                    }
                }
            };
            let tokens = TokenStream::from(expanded);

            tokens
        }
    }
}

#[proc_macro_derive(KontrolluppgiftWrite, attributes(ku))]
pub fn write_macro(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = ast.ident;

    let str_name = match parse_name_input(name.span(), ast.attrs) {
        Ok(n) => {
            n
        }
        Err(e) => {
            return e.into_compile_error().into();
        }
    };

    let fields = get_fields(ast.data);

    let field_data: Result<Vec<(String, Ident, Option<LitStr>, bool)>, Error> = fields.named.into_iter().map(|field| {
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
                })?;
            }
        }

        let ident = field.ident.clone().ok_or_else(|| Error::new(field.span(), "Expected a named field"))?;
        let name = name.ok_or_else(|| Error::new(field.span(), "must have a name ku(name=\"\")"))?;

        return Ok((name, ident, code, is_inner_type));
    }).collect();

    match field_data {
        Err(err) => {
            err.to_compile_error().into()
        }
        Ok(field) => {
            let write_operations: Vec<_> = field.iter().map(|(name, og, code, is_inner_type)| {
                match code {
                    None => {
                        if *is_inner_type {
                            quote! {
                                self.#og.write(w)?;
                            }
                        } else {
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
    }
}


fn get_fields(data: Data) -> FieldsNamed {
    return match data {
        | Data::Enum(_) | Data::Union(_) => {
            panic!("Only structs are suported")
        }
        | Data::Struct(DataStruct { fields: Fields::Named(it), .. }) => it,
        | Data::Struct(_) => {
            panic!("Not a named struct")
        }
    };
}

fn parse_name_input(span: Span, attrs: Vec<Attribute>) -> Result<LitStr, Error> {
    let mut str_name: Option<LitStr> = None;
    for attr in attrs {
        if attr.path().is_ident("ku") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let content;
                    parenthesized!(content in meta.input);
                    let lit: LitStr = content.parse()?;
                    str_name = Some(lit);
                    return Ok(());
                }

                Err(meta.error("Expected a name attribute on the struct #[ku(name(\"...\"))]"))
            })?;
        }
    };
    match str_name {
        None => {
            Err(Error::new(span, "Expected a name attribute on the struct #[ku(name(\"...\"))]"))
        }
        Some(str_name) => {
            Ok(str_name)
        }
    }
}
