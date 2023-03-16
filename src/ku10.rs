use std::borrow::Cow;
use crate::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KU10<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kontant_bruttolon_mm: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forman_utom_bil_drivmedel: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivmedel_utom_drivmedel: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub andra_kostnadsers: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlag_rutarbete: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlag_rotarbete: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ers_m_egenavgifter: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tjanstepension: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ers_ej_soc_avg: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ers_ej_soc_avg_ej_jobbavd: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forsarskattenamnden: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vissa_avdrag: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyresersattning: Option<Value<'a, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bostad_smahus: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bostad_ej_smahus: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forman_har_justerats: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forman_som_pension: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bilersattning: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traktamente: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personaloption_forvarv_andel: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arbetsstallenummer: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delagare: Option<Value<'a, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_avgifts_avtal: Option<Value<'a, bool>>,
    pub inkomstar: Value<'a, Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borttag: Option<Value<'a, bool>>,
    pub specifikationsnummer: Value<'a, i32>,
    #[serde(rename = "InkomsttagareKU10")]
    pub inkomsttagare: InkomsttagareKU10<'a>,
    #[serde(rename = "UppgiftslamnareKU10")]
    pub uppgiftslamnare: UppgiftslamnareKU10<'a>,

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UppgiftslamnareKU10<'a> {
    pub uppgiftslamnar_id: Value<'a, Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namn_uppgiftslamnare: Option<Value<'a, Cow<'a, str>>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InkomsttagareKU10<'a> {
    #[serde(rename = "LandskodTIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landskod_tin: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inkomsttagare: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fornamn: Option<Value<'a, Cow<'a, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efternamn: Option<Value<'a,Cow<'a, str>>>,
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

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{from_str};

    #[test]
    fn ku10_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFT FÖR ARBETSGIVARE MED SOCIALAVGIFTSAVTAL (KU10)_2022.xml").unwrap();

        let parsed = from_str(&*xml);
        assert!(parsed.is_ok())
    }
}
