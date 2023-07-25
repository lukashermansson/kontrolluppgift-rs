use proc_macro::{TokenStream};
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields, FieldsNamed, Ident, LitBool, LitByteStr, LitStr, parenthesized, parse_macro_input, Type, Variant};
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
        let ret = parse_field_attribute_data(&field)?;

        let ident = field.ident.clone().ok_or_else(|| Error::new(field.span(), "Expected a named identifier"))?;
        Ok((ret.name,
            Ident::new(&format!("g_field_{}", index), ident.span()),
            ident,
            field.ty,
            ret.required,
            ret.code,
            ret.is_inner_type))
    }).collect();

    match field_data {
        Err(err) => {
            err.to_compile_error().into()
        }
        Ok(field) => {
            let struct_assignments: Result<Vec<_>, Error> = field.iter().map(|(name, temp, og, _, req, _, _)| {
                if *req {
                    let str = String::from_utf8(name.value())
                        .map_err(|_| Error::new(og.span(), "name attribute was not valid utf-8 ku(name=\"...\")"))?;
                    let element = str_name.value();
                    Ok(quote! {
                        #og: #temp.ok_or_else(|| crate::Error::MissingElement { missing: #str.to_string(), reading: #element.to_string() })?,
                    })
                } else {
                    Ok(quote! {
                        #og: #temp,
                    })
                }
            }).collect();

            let struct_assignments = match struct_assignments {
                Ok(res) => res,
                Err(e) => return e.to_compile_error().into()
            };

            let match_branches: Vec<_> = field.iter().map(|(name, temp, _, typ, _, code, is_inner)| {
                if *is_inner {
                    let Type::Path(type_path) = typ.clone() else { panic!("expected path") };
                    let type_name = &type_path.path.segments.last().expect("Path should always be at least one element").ident;
                    quote! {
                        #name => { #temp = Some(#type_name::read(reader, &element)?) },
                    }
                } else if code.is_some() {
                    quote! {
                        #name => reader.read_node_into_with_code(element, #code, &mut #temp)?,
                    }
                } else {
                    quote! {
                        #name => reader.read_node_into(element, &mut #temp)?,
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
            TokenStream::from(expanded)
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
        let ret = parse_field_attribute_data(&field)?;

        let ident = field.ident.clone().ok_or_else(|| Error::new(field.span(), "Expected a named field"))?;

        let name_str = String::from_utf8(ret.name.value()).map_err(|_| Error::new(field.span(), "the name is required to be valid utf-8"))?;

        Ok((name_str, ident, ret.code, ret.is_inner_type))
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
            TokenStream::from(expanded)
        }
    }
}

fn get_fields(data: Data) -> FieldsNamed {
    match data {
        | Data::Enum(_) | Data::Union(_) => {
            panic!("Only structs are suported")
        }
        | Data::Struct(DataStruct { fields: Fields::Named(it), .. }) => it,
        | Data::Struct(_) => {
            panic!("Not a named struct")
        }
    }
}

fn get_enum_fields(data: Data) -> Vec<Variant> {
    match data {
        | Data::Enum(DataEnum { variants: it, .. }) => it.into_iter().collect(),
        | Data::Union(_) => {
            panic!("Only enums are suported")
        }
        | Data::Struct(_) => panic!("Not a enum"),
    }
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

fn parse_field_attribute_data(field: &Field) -> Result<FieldAttributeData, Error> {
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

    let name = name.ok_or_else(|| Error::new(field.span(), "must have a name ku(name(b\"...\"))"))?;
    Ok(FieldAttributeData {
        name,
        code,
        required,
        is_inner_type: is_inner_typ,
    })
}


struct FieldAttributeData {
    name: LitByteStr,
    code: Option<LitStr>,
    required: bool,
    is_inner_type: bool,
}

#[proc_macro_derive(KUStringEnum)]
pub fn string_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let variants = get_enum_fields(ast.data);

    let name = ast.ident;
    let stuff: Vec<_> = variants.into_iter().map(|e| (e.ident.clone(), e.ident.to_string())).collect();
    let strings: Vec<_> = stuff.iter().map(|e| &e.1).collect();
    let idents: Vec<_> = stuff.iter().map(|e| &e.0).collect();

    let expanded = quote! {

        impl<'a, 'b> crate::Readable<'a, 'b> for #name {
            fn get_str(data: Cow<str>) -> Result<Self, error::Error> {
                match data.as_ref() {
                            #(#strings => Ok(#name::#idents),)*
                    &_ => Err(error::Error::UnexpectedToken(format!("expected enumerated value got: {}", &data)))
                }
            }
        }

        impl crate::Writable for #name {
            fn get_str(&self) -> Option<String> {
                Some(match *self {
                    #(#name::#idents => #strings.to_string(),)*
                })
            }
        }

         impl std::convert::TryFrom<String> for #name {
            type Error = ();

            fn try_from(value: String) -> Result<Self, Self::Error> {
              match value.as_ref() {
                    #(#strings => Ok(#name::#idents),)*
                    _ => Err(())
                }
            }
        }
        impl std::convert::TryFrom<&str> for #name {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, Self::Error> {
              match value {
                    #(#strings => Ok(#name::#idents),)*
                    _ => Err(())
                }
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(KUVariantsEnum)]
pub fn variants_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let variants = get_enum_fields(ast.data);

    let stuff: Vec<_> = variants.into_iter().map(|e| (e.ident.clone(), e.ident.to_string(), e.fields.into_iter().next().unwrap().ty)).collect();
    let lits: Vec<_> = stuff.iter().map(|e| LitByteStr::new(&e.1.as_bytes(), Span::call_site())).collect();
    let idents: Vec<_> = stuff.iter().map(|e| &e.0).collect();
    let types: Vec<_> = stuff.iter().map(|e| match &e.2 {
        Type::Path(t) => (t.path.clone()).segments.last().unwrap().clone().ident,
        _ => panic!()
    }).collect();

    let expanded = quote! {

        impl<'a> KontrolluppgiftType<'a> {
            fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> {
                match self {
                    #(#idents(v) => {
                        v.write(w)?;
                    })*
                }

                Ok(())
            }
        }


        impl<'a> KontrolluppgiftType<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>) -> Result<Option<Self>, Error> {
        let mut blankettinnehall = None;

        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                            #(#lits => {
                                blankettinnehall = Some(#idents(#types::read(reader, &element)?));
                                break
                            })*
                    &_ => unexpected_element(&element)?
                },
                Event::End(_) => break,
                _ => {}
            }
        }
        return Ok(blankettinnehall)
    }
}
    };
    TokenStream::from(expanded)
}
