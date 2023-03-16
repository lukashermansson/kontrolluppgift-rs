use std::borrow::Cow;
use crate::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KU25<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delagare: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inkomstar: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borttag: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avdragsgill_ranta: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_inbetald_ranta: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betald_rantekompensation: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gemensamt_lan: Option<Value<'a, i32>>,
    pub specifikationsnummer: Value<'a, i32>,
    #[serde(rename = "InkomsttagareKU25")]
    pub inkomsttagare: InkomsttagareKU25<'a>,
    #[serde(rename = "UppgiftslamnareKU25")]
    pub uppgiftslamnare: UppgiftslamnareKU25<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InkomsttagareKU25<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inkomsttagare: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fornamn: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efternamn: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatuadress: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postnummer: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postort: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landskod_postort: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fodelsetid: Option<Value<'a, Cow<'a, str>>>,
    #[serde(rename = "AnnatIDNr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annat_id_nr: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_namn: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatuadress2: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fri_adress: Option<Value<'a, Cow<'a, str>>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UppgiftslamnareKU25<'a> {
    pub uppgiftslamnar_id: Value<'a, Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namn_uppgiftslamnare: Option<Value<'a, Cow<'a, str>>>,
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{from_str};

    #[test]
    fn ku25_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFTER RÄNTA, UTDELNING M.M. KU25 FÖR_2022.xml").unwrap();

        let parsed = from_str(&*xml);
        assert!(parsed.is_ok());
    }
}