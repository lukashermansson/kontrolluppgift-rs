pub mod ku20;
pub mod ku10;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Kontrolluppgift {
    pub avsandare: Avsandare,
    pub blankettgemensamt: Blankettgemensamt,
    #[serde(rename = "Blankett")]
    pub blanketter: Vec<Blankett>,
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
    KU10(ku10::KU10),
    KU20(ku20::KU20),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Value<T> {
    pub faltkod: String,
    #[serde(rename = "$value")]
    pub value: T,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Arendeinformation {
    pub arendeagare: String,
    pub period: String,
    pub arendenummer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Kontaktperson {
    pub namn: String,
    pub telefon: String,
    pub epostadress: String,
    pub sakomrade: Option<String>,
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


