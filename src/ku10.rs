use crate::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KU10 {
    pub kontant_bruttolon_mm: Option<Value<i64>>,
    pub forman_utom_bil_drivmedel: Option<Value<i64>>,
    pub drivmedel_utom_drivmedel: Option<Value<i64>>,
    pub andra_kostnadsers: Option<Value<i64>>,
    pub underlag_rutarbete: Option<Value<i64>>,
    pub underlag_rotarbete: Option<Value<i64>>,
    pub ers_m_egenavgifter: Option<Value<i64>>,
    pub tjanstepension: Option<Value<i64>>,
    pub ers_ej_soc_avg: Option<Value<i64>>,
    pub ers_ej_soc_avg_ej_jobbavd: Option<Value<i64>>,
    pub forsarskattenamnden: Option<Value<i64>>,
    pub vissa_avdrag: Option<Value<i64>>,
    pub hyresersattning: Option<Value<i64>>,
    pub bostad_smahus: Option<Value<bool>>,
    pub bostad_ej_smahus: Option<Value<bool>>,
    pub forman_har_justerats: Option<Value<bool>>,
    pub forman_som_pension: Option<Value<bool>>,
    pub bilersattning: Option<Value<bool>>,
    pub traktamente: Option<Value<bool>>,
    pub personaloption_forvarv_andel: Option<Value<bool>>,
    pub arbetsstallenummer: Option<Value<String>>,
    pub delagare: Option<Value<bool>>,
    pub social_avgifts_avtal: Option<Value<bool>>,
    pub inkomstar: Value<String>,
    pub borttag: Option<Value<bool>>,
    pub specifikationsnummer: Value<i64>,
    #[serde(rename = "InkomsttagareKU10")]
    pub inkomsttagare: InkomsttagareKU10,
    #[serde(rename = "UppgiftslamnareKU10")]
    pub uppgiftslamnare: UppgiftslamnareKU10,

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UppgiftslamnareKU10 {
    pub uppgiftslamnar_id: Value<String>,
    pub namn_uppgiftslamnare: Option<Value<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InkomsttagareKU10 {
    #[serde(rename = "LandskodTin")]
    pub landskod_tin: Option<Value<String>>,
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

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_xml_rs::from_str;
    use crate::Kontrolluppgift;

    #[test]
    fn ku10_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFT FÖR ARBETSGIVARE MED SOCIALAVGIFTSAVTAL (KU10)_2022.xml").unwrap();

        let parsed = from_str::<Kontrolluppgift>(&*xml);
        assert!(parsed.is_ok())
    }
}
