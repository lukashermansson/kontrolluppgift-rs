use crate::{IdentitetsbeteckningForPerson, Landskod};
use kontrolluppgift_macros::{KontrolluppgiftRead, KontrolluppgiftWrite};
use std::borrow::Cow;

/// Kontrolluppgift 32
#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("KU32"))]
pub struct KU32Type<'a> {
    #[ku(name(b"Delagare"), code("061"))]
    pub delagare: Option<bool>,
    #[ku(name(b"Inkomstar"), code("203"), required(true))]
    pub inkomstar: Cow<'a, str>,
    #[ku(name(b"Borttag"), code("205"))]
    pub borttag: Option<bool>,
    #[ku(name(b"Depanummer"), code("523"))]
    pub depanummer: Option<i32>,
    #[ku(name(b"AndelAvDepan"), code("524"))]
    pub andel_av_depan: Option<f32>,
    #[ku(name(b"Specifikationsnummer"), code("570"), required(true))]
    pub specifikationsnummer: i32,
    #[ku(name(b"VPNamn"), code("571"))]
    pub vp_namn: Option<Cow<'a, str>>,
    #[ku(name(b"ISIN"), code("572"))]
    pub isin: Option<Cow<'a, str>>,
    #[ku(name(b"AvyttradTillISK"), code("573"))]
    pub avyttrad_till_isk: Option<bool>,
    #[ku(name(b"AntalAvyttrade"), code("576"))]
    pub antal_avyttrade: Option<i32>,
    #[ku(name(b"OkandVarde"), code("599"))]
    pub okand_varde: Option<bool>,
    #[ku(name(b"ErhallenErsattning"), code("810"))]
    pub erhallen_ersattning: Option<i32>,
    #[ku(name(b"InkomsttagareKU32"), required(true), inner_ty(true))]
    pub inkomsttagare: InkomsttagareKU32<'a>,
    #[ku(name(b"UppgiftslamnareKU32"), required(true), inner_ty(true))]
    pub uppgiftslamnare: UppgiftslamnareKU32<'a>,
}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("InkomsttagareKU32"))]
pub struct InkomsttagareKU32<'a> {
    #[ku(name(b"LandskodTIN"), code("076"))]
    pub landskod_tin: Option<Landskod>,
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

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("UppgiftslamnareKU32"))]
pub struct UppgiftslamnareKU32<'a> {
    #[ku(name(b"UppgiftslamnarId"), code("201"), required(true))]
    pub uppgiftslamnar_id: Cow<'a, str>,
    #[ku(name(b"NamnUppgiftslamnare"), code("202"))]
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KontrolluppgiftType::KU32;
    use crate::{
        from_str, to_string, Arendeinformation, Avsandare, Blankett, Blankettgemensamt,
        Kontaktperson, Kontrolluppgift, Landskod, TekniskKontaktperson, Uppgiftslamnare,
    };

    #[test]
    fn ku32_is_parsed_to_and_back() {
        let ku32 = Kontrolluppgift {
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
                blankettinnehall: KU32(KU32Type {
                    delagare: Some(false),
                    inkomstar: "2022".into(),
                    borttag: Some(true),
                    depanummer: Some(4),
                    andel_av_depan: Some(0.2),
                    specifikationsnummer: 5,
                    vp_namn: Some("test".into()),
                    isin: Some("isin".into()),
                    okand_varde: Some(false),
                    inkomsttagare: InkomsttagareKU32 {
                        landskod_tin: Some(Landskod::AF),
                        inkomsttagare: Some("191612299279".try_into().unwrap()),
                        fornamn: Some("Test".into()),
                        efternamn: Some("Testsson".into()),
                        gatuadress: Some("Gata".into()),
                        postnummer: Some("7456".into()),
                        postort: Some("Postort".into()),
                        landskod_postort: Some(Landskod::AI),
                        fodelsetid: Some("20230106".into()),
                        annat_id_nr: Some("202".into()),
                        org_namn: Some("Organization".into()),
                        gatuadress2: Some("Gata2".into()),
                        fri_adress: Some("Storgatan 3".into()),
                        tin: Some("some tin".into()),
                    },
                    uppgiftslamnare: UppgiftslamnareKU32 {
                        uppgiftslamnar_id: "165599990602".into(),
                        namn_uppgiftslamnare: Some("Foretag 1".into()),
                    },
                    avyttrad_till_isk: Some(true),
                    antal_avyttrade: Some(6),
                    erhallen_ersattning: Some(7),
                }),
            }],
        };
        let unparsed = to_string(&ku32).unwrap();
        let re_parsed = from_str(&*unparsed).unwrap();
        assert_eq!(ku32, re_parsed);
    }
}
