pub mod ku20;
pub mod ku10;
pub mod ku25;
pub mod ku28;

use std::borrow::Cow;
use std::io::Cursor;
use quick_xml::{Reader, Writer};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use serde::{Deserialize, Serialize};
use crate::DeError::InvalidXml;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Kontrolluppgift<'a> {
    pub avsandare: Avsandare<'a>,
    pub blankettgemensamt: Blankettgemensamt<'a>,
    #[serde(rename = "Blankett")]
    pub blanketter: Vec<Blankett<'a>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Blankettgemensamt<'a> {
    pub uppgiftslamnare: Uppgiftslamnare<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Uppgiftslamnare<'a> {
    pub uppgiftslamnare_pers_orgnr: Cow<'a, str>,
    pub kontaktperson: Kontaktperson<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Blankett<'a> {
    #[serde(rename = "@nummer")]
    pub nummer: i64,
    pub arendeinformation: Arendeinformation<'a>,
    pub blankettinnehall: Blankettinnehall<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Blankettinnehall<'a> {
    #[serde(rename = "$value")]
    pub value: KontrolluppgiftType<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum KontrolluppgiftType<'a> {
    KU10(ku10::KU10<'a>),
    KU20(ku20::KU20<'a>),
    KU25(ku25::KU25<'a>),
    KU28(ku28::KU28<'a>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Value<'a, T> {
    #[serde(rename = "@faltkod")]
    pub faltkod: Cow<'a, str>,
    #[serde(rename = "$value")]
    pub value: T,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Arendeinformation<'a> {
    pub arendeagare: Cow<'a, str>,
    pub period: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arendenummer: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Kontaktperson<'a> {
    pub namn: Cow<'a, str>,
    pub telefon: Cow<'a, str>,
    pub epostadress: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sakomrade: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Avsandare<'a> {
    pub programnamn: Cow<'a, str>,
    pub organisationsnummer: Cow<'a, str>,
    pub teknisk_kontaktperson: TekniskKontaktperson<'a>,
    pub skapad: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TekniskKontaktperson<'a> {
    pub namn: Cow<'a, str>,
    pub telefon: Cow<'a, str>,
    pub epostadress: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utdelningsadress1: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utdelningsadress2: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postnummer: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postort: Option<Cow<'a, str>>,
}


/// Deserialize xml into rust types
/// Does not validate data contained according to specification
/// It also does not currently validate any namespace information ( this may be subject to change before 1.0)
pub fn from_str(str: &str) -> Result<Kontrolluppgift, DeError> {
    let parsed = quick_xml::de::from_str::<Kontrolluppgift>(str).map_err(|x| InvalidXml(x.to_string()));
    return parsed;
}

#[derive(Clone, Debug)]
pub enum DeError {
    /// Serde custom error
    Custom(String),
    /// Xml parsing error
    InvalidXml(String),
}

/// To serialized xml. Provides namespaces.
/// Currently, it does not include the header with encoding, this may be subject to change before 1.0 however
pub fn to_string(kontrolluppgift: &Kontrolluppgift) -> Result<String, DeError> {
    let serialized = quick_xml::se::to_string_with_root("Skatteverket", kontrolluppgift).map_err(|e| InvalidXml(e.to_string()))?;

    // rewrite root element to include namespace information
    let mut reader = Reader::from_str(&*serialized);
    reader.trim_text(true);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name().as_ref() == b"Skatteverket" => {
                let mut elem = BytesStart::new("i:Skatteverket");

                elem.push_attribute(("xmlns:i", "http://xmls.skatteverket.se/se/skatteverket/ai/instans/infoForBeskattning/8.0"));
                elem.push_attribute(("xmlns", "http://xmls.skatteverket.se/se/skatteverket/ai/komponent/infoForBeskattning/8.0"));
                elem.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));
                elem.push_attribute(("omrade", "Kontrolluppgifter"));
                elem.push_attribute(("xsi:schemaLocation", "http://xmls.skatteverket.se/se/skatteverket/ai/instans/infoForBeskattning/8.0 http://xmls.skatteverket.se/se/skatteverket/ai/kontrolluppgift/instans/Kontrolluppgifter_8.0.xsd"));

                assert!(writer.write_event(Event::Start(elem)).is_ok());
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"Skatteverket" => {
                assert!(writer.write_event(Event::End(BytesEnd::new("i:Skatteverket"))).is_ok());
            }
            Ok(Event::Eof) => break,
            Ok(e) => assert!(writer.write_event(e).is_ok()),
            Err(e) => return Err(DeError::Custom(format!("Error at position {}: {:?}", reader.buffer_position(), e))),
        }
    }
    let result = String::from_utf8(writer.into_inner().into_inner()).expect("This should always be valid utf-8");
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use crate::{Avsandare, Blankettgemensamt, Kontaktperson, Kontrolluppgift, TekniskKontaktperson, to_string, Uppgiftslamnare};

    #[test]
    fn serializes_with_namespace() {
        let parsed = to_string(&Kontrolluppgift {
            avsandare: Avsandare {
                programnamn: Default::default(),
                organisationsnummer: Default::default(),
                teknisk_kontaktperson: TekniskKontaktperson {
                    namn: "Namn".into(),
                    telefon: Default::default(),
                    epostadress: Default::default(),
                    utdelningsadress1: None,
                    utdelningsadress2: None,
                    postnummer: None,
                    postort: None,
                },
                skapad: Default::default(),
            },
            blankettgemensamt: Blankettgemensamt {
                uppgiftslamnare: Uppgiftslamnare {
                    uppgiftslamnare_pers_orgnr: Default::default(),
                    kontaktperson: Kontaktperson {
                        namn: Default::default(),
                        telefon: Default::default(),
                        epostadress: Default::default(),
                        sakomrade: None,
                    } } },
            blanketter: vec![],
        });
        assert!(parsed.is_ok());
        let data = parsed.unwrap();
        println!("{}", &data);
        assert_eq!(data, r#"<i:Skatteverket xmlns:i="http://xmls.skatteverket.se/se/skatteverket/ai/instans/infoForBeskattning/8.0" xmlns="http://xmls.skatteverket.se/se/skatteverket/ai/komponent/infoForBeskattning/8.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" omrade="Kontrolluppgifter" xsi:schemaLocation="http://xmls.skatteverket.se/se/skatteverket/ai/instans/infoForBeskattning/8.0 http://xmls.skatteverket.se/se/skatteverket/ai/kontrolluppgift/instans/Kontrolluppgifter_8.0.xsd"><Avsandare><Programnamn/><Organisationsnummer/><TekniskKontaktperson><Namn>Namn</Namn><Telefon/><Epostadress/></TekniskKontaktperson><Skapad/></Avsandare><Blankettgemensamt><Uppgiftslamnare><UppgiftslamnarePersOrgnr/><Kontaktperson><Namn/><Telefon/><Epostadress/></Kontaktperson></Uppgiftslamnare></Blankettgemensamt></i:Skatteverket>"#);
    }
}
