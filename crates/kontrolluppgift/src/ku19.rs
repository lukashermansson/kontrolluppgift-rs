use crate::{IdentitetsbeteckningForPerson, Landskod};
use kontrolluppgift_macros::{KontrolluppgiftRead, KontrolluppgiftWrite};
use std::borrow::Cow;

extern crate self as kontrolluppgift;

/// Kontrolluppgift 19
#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("KU19"))]
pub struct KU19Type<'a> {
    #[ku(name(b"AvdragenSkatt"), code("001"))]
    pub avdragen_skatt: Option<i32>,
    // this can be an enum
    #[ku(name(b"Ersattningskod"), code("004"))]
    pub ersattningskod: Option<Cow<'a, str>>,
    #[ku(name(b"ErsattningBelopp"), code("005"))]
    pub ersattning_belopp: Option<i32>,
    #[ku(name(b"Inkomstar"), code("203"), required(true))]
    pub inkomstar: Cow<'a, str>,
    #[ku(name(b"Borttag"), code("205"))]
    pub borttag: Option<bool>,
    #[ku(name(b"Specifikationsnummer"), code("570"), required(true))]
    pub specifikationsnummer: i32,
    #[ku(name(b"InkomsttagareKU19"), required(true), inner_ty(true))]
    pub inkomsttagare: InkomsttagareKU19<'a>,
    #[ku(name(b"UppgiftslamnareKU19"), required(true), inner_ty(true))]
    pub uppgiftslamnare: UppgiftslamnareKU19<'a>,
}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("UppgiftslamnareKU19"))]
pub struct UppgiftslamnareKU19<'a> {
    #[ku(name(b"UppgiftslamnarId"), code("201"), required(true))]
    pub uppgiftslamnar_id: Cow<'a, str>,
    #[ku(name(b"NamnUppgiftslamnare"), code("202"))]
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("InkomsttagareKU19"))]
pub struct InkomsttagareKU19<'a> {
    #[ku(name(b"LandskodTIN"), code("076"))]
    pub landskod_tin: Option<Landskod>,
    #[ku(name(b"LandskodMedborgare"), code("081"))]
    pub landskod_medborgare: Option<Landskod>,
    #[ku(name(b"Inkomsttagare"), code("215"))]
    pub inkomsttagare: Option<IdentitetsbeteckningForPerson<'a>>,
    #[ku(name(b"Fornamn"), code("216"))]
    pub fornamn: Option<Cow<'a, str>>,
    #[ku(name(b"Efternamn"), code("217"))]
    pub efternamn: Option<Cow<'a, str>>,
    #[ku(name(b"Gatuadress"), code("218"))]
    pub gatuadress: Option<Cow<'a, str>>,
    #[ku(name(b"Postnummer"), code("219"))]
    pub postnummer: Option<Cow<'a, str>>,
    #[ku(name(b"Postort"), code("220"))]
    pub postort: Option<Cow<'a, str>>,
    #[ku(name(b"LandskodPostort"), code("221"))]
    pub landskod_postort: Option<Landskod>,
    #[ku(name(b"Fodelsetid"), code("222"))]
    pub fodelsetid: Option<Cow<'a, str>>,
    #[ku(name(b"AnnatIDNr"), code("224"))]
    pub annat_id_nr: Option<Cow<'a, str>>,
    #[ku(name(b"OrgNamn"), code("226"))]
    pub org_namn: Option<Cow<'a, str>>,
    #[ku(name(b"Gatuadress2"), code("228"))]
    pub gatuadress2: Option<Cow<'a, str>>,
    #[ku(name(b"FriAdress"), code("230"))]
    pub fri_adress: Option<Cow<'a, str>>,
    #[ku(name(b"TIN"), code("252"))]
    pub tin: Option<Cow<'a, str>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KontrolluppgiftType::KU19;
    use crate::{
        from_str, to_string, Arendeinformation, Avsandare, Blankett, Blankettgemensamt,
        Kontaktperson, Kontrolluppgift, TekniskKontaktperson, Uppgiftslamnare,
    };

    #[test]
    fn ku19_is_parsed_to_and_back() {
        let ku19 = Kontrolluppgift {
            avsandare: Avsandare {
                teknisk_kontaktperson: TekniskKontaktperson {
                    ..Default::default()
                },
                ..Default::default()
            },
            blankettgemensamt: Blankettgemensamt {
                uppgiftslamnare: Uppgiftslamnare {
                    kontaktperson: Kontaktperson {
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            blanketter: vec![Blankett {
                nummer: 0,
                arendeinformation: Arendeinformation {
                    ..Default::default()
                },
                blankettinnehall: KU19(KU19Type {
                    avdragen_skatt: Some(1),
                    ersattningskod: Some("402".into()),
                    ersattning_belopp: Some(19),
                    inkomstar: "2022".into(),
                    borttag: Some(false),
                    specifikationsnummer: 5,
                    inkomsttagare: InkomsttagareKU19 {
                        landskod_tin: Some(Landskod::SE),
                        landskod_medborgare: Some(Landskod::SE),
                        inkomsttagare: Some("191612299279".try_into().unwrap()),
                        fornamn: Some("Test".into()),
                        efternamn: Some("Testsson".into()),
                        gatuadress: Some("Gata".into()),
                        postnummer: Some("7456".into()),
                        postort: Some("Postort".into()),
                        landskod_postort: Some(Landskod::FI),
                        fodelsetid: Some("20230106".into()),
                        annat_id_nr: Some("202".into()),
                        org_namn: Some("Organization".into()),
                        gatuadress2: Some("Gata2".into()),
                        fri_adress: Some("Storgatan 3".into()),
                        tin: Some("Tin".into()),
                    },
                    uppgiftslamnare: UppgiftslamnareKU19 {
                        uppgiftslamnar_id: "165599990602".into(),
                        namn_uppgiftslamnare: Some("Foretag 1".into()),
                    },
                }),
            }],
        };
        let unparsed = to_string(&ku19).unwrap();
        let re_parsed = from_str(&*unparsed).unwrap();
        assert_eq!(ku19, re_parsed);
    }
}
