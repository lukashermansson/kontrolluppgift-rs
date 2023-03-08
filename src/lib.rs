mod ku20;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Kontrolluppgift {
    pub avsandare: Avsandare,
    pub blankettgemensamt: Blankettgemensamt,
    #[serde(rename = "Blankett")]
    pub blanketter: Vec<Blankett>
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Blankettgemensamt {
    pub uppgiftslamnare: Uppgiftslamnare,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Uppgiftslamnare {
    pub uppgiftslamnare_pers_orgnr: String,
    pub kontaktperson: Kontaktperson,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Blankett {
    #[serde(rename = "nummer")]
    pub nummer: i64,
    pub arendeinformation: Arendeinformation,
    pub blankettinnehall: KontrolluppgiftType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum KontrolluppgiftType {
   KU20(ku20::KU20),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Value<T> {
    pub faltkod: String,
    #[serde(rename = "$value")]
    pub value: T
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Arendeinformation {
    pub arendeagare: String,
    pub period: String,
    pub arendenummer: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Kontaktperson {
    pub namn: String,
    pub telefon: String,
    pub epostadress: String,
    pub sakomrade: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Avsandare {
    pub programnamn: String,
    pub organisationsnummer: String,
    pub teknisk_kontaktperson: TekniskKontaktperson,
    pub skapad: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TekniskKontaktperson {
    pub namn: String,
    pub telefon: String,
    pub epostadress: String,
    pub utdelningsadress1: Option<String>,
    pub utdelningsadress2: Option<String>,
    pub postnummer: Option<String>,
    pub postort: Option<String>,
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::read_to_string;
    use serde_xml_rs::from_str;
    use super::*;

    #[test]
    fn it_works() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFTER RÄNTA, UTDELNING M.M.(KU20-25, KU30-50, KU71-72, KU80-81) FÖR_2022.xml").unwrap();

        let parsed : Kontrolluppgift = from_str(&*xml).unwrap();

        println!("{:?}", parsed)
    }
}
