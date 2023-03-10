use crate::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KU25 {
    pub delagare: Option<Value<bool>>,
    pub inkomstar: Option<Value<String>>,
    pub borttag: Option<Value<bool>>,
    pub avdragsgill_ranta: Option<Value<i64>>,
    pub total_inbetald_ranta: Option<Value<i64>>,
    pub betald_rantekompensation: Option<Value<i64>>,
    pub gemensamt_lan: Option<Value<i64>>,
    pub specifikationsnummer: Value<i64>,
    #[serde(rename = "InkomsttagareKU25")]
    pub inkomsttagare: InkomsttagareKU25,
    #[serde(rename = "UppgiftslamnareKU25")]
    pub uppgiftslamnare: UppgiftslamnareKU25,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InkomsttagareKU25 {
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UppgiftslamnareKU25 {
    pub uppgiftslamnar_id: Value<String>,
    pub namn_uppgiftslamnare: Option<Value<String>>,
}


#[cfg(test)]
mod tests {
    use std::fs;
    use serde_xml_rs::from_str;
    use crate::Kontrolluppgift;

    #[test]
    fn ku25_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFTER RÄNTA, UTDELNING M.M. KU25 FÖR_2022.xml").unwrap();

        let parsed = from_str::<Kontrolluppgift>(&*xml);

        assert!(parsed.is_ok());
        println!("{:?}", parsed.unwrap())
    }
}