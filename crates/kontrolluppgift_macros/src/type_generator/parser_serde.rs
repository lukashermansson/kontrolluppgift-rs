use quick_xml::de::from_str;
use serde::{Deserialize, Deserializer};

pub fn parse_types(string: &str) -> Schema {
    return from_str(string).unwrap();
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub element: Vec<ElementDef>,
    pub complex_type: Vec<ComplexType>,
    pub simple_type: Vec<SimpleType>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElementDef {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub ty: Option<String>,
    #[serde(default, rename = "@abstract")]
    pub is_abstract: bool,
    #[serde(rename = "@substitutionGroup")]
    pub substitution_group: Option<String>,
    pub annotation: Option<Annotationtype>,
    pub complex_type: Option<ComplexTypeInElement>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComplexTypeInElement {
    simple_content: Option<SimpleContent>,
    complex_type: Option<ComplexType>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleContent {
    extension: Extension,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    #[serde(rename = "@base")]
    base: String,
    attribute: Attribute,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@fixed")]
    fixed: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComplexType {
    #[serde(rename = "@name")]
    pub name: String,
    pub all: Option<AllElements>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AllElements {
    #[serde(default)]
    pub element: Vec<ComplexTypeElement>
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComplexContent {
    #[serde(default)]
    pub element: Vec<ComplexTypeElement>,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleType {
    #[serde(rename = "@name")]
    pub name: String,
    pub restriction: Option<SimpleTypeRestriction>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleTypeRestriction {
    #[serde(rename = "@base")]
    pub base: String,
    pub length: Option<Value>,
    pub min_length: Option<Value>,
    pub max_length: Option<Value>,
    pub min_inclusive: Option<Value>,
    pub max_inclusive: Option<Value>,
    pub pattern: Option<Value>,

    #[serde(default)]
    pub enumeration: Vec<EnumerationType>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumerationType {
    #[serde(rename = "@value")]
    pub value: String,
    pub annotation: Option<Annotationtype>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Annotationtype {
    pub documentation: Documentation,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Documentation {
    #[serde(rename = "$text")]
    pub text: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComplexTypeElement {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@type")]
    pub ty: Option<String>,
    #[serde(rename = "@ref")]
    pub reference: Option<String>,
    #[serde(rename = "@minOccurs")]
    pub min_occurs: String,
    #[serde(rename = "@maxOccurs")]
    pub max_occurs: String,
}




mod tests {
    use crate::type_generator::parser_serde::parse_types;

    #[test]
    fn test() {
        let string = include_str!("../Kontrolluppgifter_COMPONENT_9.0.xsd");

        dbg!(parse_types(string));
    }
}
