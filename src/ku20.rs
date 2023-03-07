use crate::{Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KU20 {
    avdragen_skatt: Option<Value<i64>>,
    delagare: Option<Value<String>>,
    inkomstar: Value<String>,
    borttag: Option<Value<i64>>,
    ranteinkomst: Option<Value<i64>>,
    forfogarkonto: Option<Value<i64>>,
    ranteinkomst_ej_konto: Option<Value<i64>>,
    annan_inkomst: Option<Value<i64>>,
    specifikationsnummer: Option<Value<i64>>,
    inkomsttagare_KU20: InkomsttagareKU20,
    uppgiftslamnare_KU20: UppgiftslamnareKU20,

}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InkomsttagareKU20 {
    landskod_TIN: Option<Value<String>>,
    fodelseort: Option<Value<String>>,
    landskod_fodelseort: Option<Value<String>>,
    inkomsttagare: Option<Value<String>>,
    fornamn: Option<Value<String>>,
    efternamn: Option<Value<String>>,
    gatuadress: Option<Value<String>>,
    postort: Option<Value<String>>,
    landskod_postort: Option<Value<String>>,
    fodelsetid: Option<Value<String>>,
    annat_ID_nr: Option<Value<String>>,
    org_namn: Option<Value<String>>,
    gatuadress2: Option<Value<String>>,
    fri_adress: Option<Value<String>>,
    TIN: Option<Value<String>>

}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UppgiftslamnareKU20 {
    uppgiftslamnar_id: Value<String>,
    namn_uppgiftslamnare: Option<Value<String>>
}