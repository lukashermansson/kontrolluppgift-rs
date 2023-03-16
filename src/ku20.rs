use std::borrow::Cow;
use crate::{Value};
use serde::{Deserialize, Serialize};

/// Kontrolluppgift 20
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KU20<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avdragen_skatt: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delagare: Option<Value<'a, bool>>,
    pub inkomstar: Value<'a, Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borttag: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranteinkomst: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forfogarkonto: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranteinkomst_ej_konto: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annan_inkomst: Option<Value<'a, i32>>,
    pub specifikationsnummer: Value<'a, i32>,
    #[serde(rename = "InkomsttagareKU20")]
    pub inkomsttagare: InkomsttagareKU20<'a>,
    #[serde(rename = "UppgiftslamnareKU20")]
    pub uppgiftslamnare: UppgiftslamnareKU20<'a>,

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InkomsttagareKU20<'a> {
    #[serde(rename = "LandskodTin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landskod_tin: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fodelseort: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landskod_fodelseort: Option<Value<'a, Cow<'a, str>>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "AnnatIDNr")]
    pub annat_id_nr: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_namn: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatuadress2: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fri_adress: Option<Value<'a, Cow<'a, str>>>,
    #[serde(rename = "TIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tin: Option<Value<'a, Cow<'a, str>>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UppgiftslamnareKU20<'a> {
    pub uppgiftslamnar_id: Value<'a, Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namn_uppgiftslamnare: Option<Value<'a, Cow<'a, str>>>,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{from_str};

    #[test]
    fn ku20_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFTER RÄNTA, UTDELNING M.M. KU20 FÖR_2022.xml").unwrap();

        let parsed = from_str(&*xml);

        assert!(parsed.is_ok())
    }
}
