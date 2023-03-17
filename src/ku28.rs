use std::borrow::Cow;
use crate::{Value};
use serde::{Deserialize, Serialize};

/// Kontrolluppgift 28
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KU28<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delagare: Option<Value<'a, bool>>,
    pub inkomstar: Value<'a, Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borttag: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlag_for_investeraravdrag: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tot_underlag_investeraravdrag: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betalningsar: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aterforing_avyttring: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aterforing_utflyttning: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aterforing_hog_vardeoverforing: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aterforing_interna_forvarv: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datum_forvarv: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verksamhetsomrade: Option<Value<'a, Cow<'a, str>>>,
    pub specifikationsnummer: Value<'a, i32>,
    #[serde(rename = "InkomsttagareKU28")]
    pub inkomsttagare: InkomsttagareKU28<'a>,
    #[serde(rename = "UppgiftslamnareKU28")]
    pub uppgiftslamnare: UppgiftslamnareKU28<'a>,

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InkomsttagareKU28<'a> {
    #[serde(rename = "LandskodTIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landskod_tin: Option<Value<'a, Cow<'a, str>>>,
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
pub struct UppgiftslamnareKU28<'a> {
    pub uppgiftslamnar_id: Value<'a, Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namn_uppgiftslamnare: Option<Value<'a, Cow<'a, str>>>,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{from_str};

    #[test]
    fn ku28_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFT INVESTERARAVDRAG (KU28)_2022.xml").unwrap();

        let parsed = from_str(&*xml);
        assert!(parsed.is_ok())
    }
}
