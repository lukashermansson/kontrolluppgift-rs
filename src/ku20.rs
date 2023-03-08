use crate::{Value};
use serde::{Deserialize, Serialize};

/// Kontrolluppgift 20
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KU20 {
    pub avdragen_skatt: Option<Value<i64>>,
    pub delagare: Option<Value<bool>>,
    pub inkomstar: Value<String>,
    pub borttag: Option<Value<bool>>,
    pub ranteinkomst: Option<Value<i64>>,
    pub forfogarkonto: Option<Value<bool>>,
    pub ranteinkomst_ej_konto: Option<Value<i64>>,
    pub annan_inkomst: Option<Value<i64>>,
    pub specifikationsnummer: Value<i64>,
    #[serde(rename = "InkomsttagareKU20")]
    pub inkomsttagare: InkomsttagareKU20,
    #[serde(rename = "UppgiftslamnareKU20")]
    pub uppgiftslamnare: UppgiftslamnareKU20,

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InkomsttagareKU20 {
    #[serde(rename = "LandskodTin")]
    pub landskod_tin: Option<Value<String>>,
    pub fodelseort: Option<Value<String>>,
    pub landskod_fodelseort: Option<Value<String>>,
    pub inkomsttagare: Option<Value<String>>,
    pub fornamn: Option<Value<String>>,
    pub efternamn: Option<Value<String>>,
    pub gatuadress: Option<Value<String>>,
    pub postnummer: Option<Value<String>>,
    pub postort: Option<Value<String>>,
    pub landskod_postort: Option<Value<String>>,
    pub fodelsetid: Option<Value<String>>,
    #[serde(rename = "AnnatIDNr")]
    pub annat_id_nr: Option<Value<String>>,
    pub org_namn: Option<Value<String>>,
    pub gatuadress2: Option<Value<String>>,
    pub fri_adress: Option<Value<String>>,
    #[serde(rename = "TIN")]
    pub tin: Option<Value<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UppgiftslamnareKU20 {
    pub uppgiftslamnar_id: Value<String>,
    pub namn_uppgiftslamnare: Option<Value<String>>,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_xml_rs::from_str;
    use crate::Kontrolluppgift;

    #[test]
    fn ku20_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFTER RÄNTA, UTDELNING M.M.(KU20-25, KU30-50, KU71-72, KU80-81) FÖR_2022.xml").unwrap();

        let parsed = from_str::<Kontrolluppgift>(&*xml);

        assert!(parsed.is_ok())
    }
}
