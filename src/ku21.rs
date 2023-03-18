use std::borrow::Cow;
use crate::{Value};
use serde::{Deserialize, Serialize};

/// Kontrolluppgift 21
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KU21<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avdragen_skatt: Option<Value<'a, i32>>,
    pub inkomstar: Value<'a, Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borttag: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annan_inkomst: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranta_fodringsratter: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utbetalt_i_vissa_fall: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depanummer: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub andel_av_depan: Option<Value<'a, f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erhallen_rantekompensation: Option<Value<'a, f32>>,
    pub specifikationsnummer: Value<'a, i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "VPNamn")]
    pub vp_namn: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ISIN")]
    pub isin: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "AvyttradTillISK")]
    pub avyttrad_till_isk: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okand_varde: Option<Value<'a, bool>>,
    #[serde(rename = "InkomsttagareKU21")]
    pub inkomsttagare: InkomsttagareKU21<'a>,
    #[serde(rename = "UppgiftslamnareKU21")]
    pub uppgiftslamnare: UppgiftslamnareKU21<'a>,

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InkomsttagareKU21<'a> {
    #[serde(rename = "LandskodTIN")]
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
pub struct UppgiftslamnareKU21<'a> {
    pub uppgiftslamnar_id: Value<'a, Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namn_uppgiftslamnare: Option<Value<'a, Cow<'a, str>>>,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{from_str};

    #[test]
    fn ku21_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFTER RÄNTA, UTDELNING M.M. KU21 FÖR_2022.xml").unwrap();

        let parsed = from_str(&*xml);

        println!("{:?}", &parsed);
        assert!(parsed.is_ok())
    }
}
