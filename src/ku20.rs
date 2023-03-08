use crate::{Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KU20 {
    pub avdragen_skatt: Option<Value<i64>>,
    pub delagare: Option<Value<String>>,
    pub inkomstar: Value<String>,
    pub borttag: Option<Value<i64>>,
    pub ranteinkomst: Option<Value<i64>>,
    pub forfogarkonto: Option<Value<i64>>,
    pub ranteinkomst_ej_konto: Option<Value<i64>>,
    pub annan_inkomst: Option<Value<i64>>,
    pub specifikationsnummer: Option<Value<i64>>,
    pub inkomsttagare_KU20: InkomsttagareKU20,
    pub uppgiftslamnare_KU20: UppgiftslamnareKU20,

}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InkomsttagareKU20 {
    pub landskod_TIN: Option<Value<String>>,
    pub fodelseort: Option<Value<String>>,
    pub landskod_fodelseort: Option<Value<String>>,
    pub inkomsttagare: Option<Value<String>>,
    pub fornamn: Option<Value<String>>,
    pub efternamn: Option<Value<String>>,
    pub gatuadress: Option<Value<String>>,
    pub postort: Option<Value<String>>,
    pub landskod_postort: Option<Value<String>>,
    pub fodelsetid: Option<Value<String>>,
    pub annat_ID_nr: Option<Value<String>>,
    pub org_namn: Option<Value<String>>,
    pub gatuadress2: Option<Value<String>>,
    pub fri_adress: Option<Value<String>>,
    pub TIN: Option<Value<String>>

}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UppgiftslamnareKU20 {
    pub uppgiftslamnar_id: Value<String>,
    pub namn_uppgiftslamnare: Option<Value<String>>
}