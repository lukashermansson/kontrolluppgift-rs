mod ku20;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Kontrolluppgift {
    avsandare: Avsandare,
    blankettgemensamt: Blankettgemensamt,
    #[serde(rename = "Blankett")]
    blanketter: Vec<Blankett>
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Blankettgemensamt {
    uppgiftslamnare: Uppgiftslamnare,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Uppgiftslamnare {
    uppgiftslamnare_pers_orgnr: String,
    kontaktperson: Kontaktperson,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Blankett {
    #[serde(rename = "nummer")]
    nummer: i64,
    arendeinformation: Arendeinformation,
    blankettinnehall: KontrolluppgiftType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum KontrolluppgiftType {
   KU20(ku20::KU20),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Value<T> {
    faltkod: String,
    #[serde(rename = "$value")]
    value: T
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Arendeinformation {
    arendeagare: String,
    period: String,
    arendenummer: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Kontaktperson {
    namn: String,
    telefon: String,
    epostadress: String,
    sakomrade: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Avsandare {
    programnamn: String,
    organisationsnummer: String,
    teknisk_kontaktperson: TekniskKontaktperson,
    skapad: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TekniskKontaktperson {
    namn: String,
    telefon: String,
    epostadress: String,
    utdelningsadress1: Option<String>,
    utdelningsadress2: Option<String>,
    postnummer: Option<String>,
    postort: Option<String>,
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
