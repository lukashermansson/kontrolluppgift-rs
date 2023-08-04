use crate::{IdentitetsbeteckningForPerson, KUDate, Landskod};
use kontrolluppgift_macros::{KontrolluppgiftRead, KontrolluppgiftWrite};
use std::borrow::Cow;

/// Kontrolluppgift 31
#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("KU31"))]
pub struct KU31Type<'a> {
    #[ku(name(b"AvdragenSkatt"), code("001"))]
    pub avdragen_skatt: Option<i32>,
    #[ku(name(b"AvdragenUtlandskSkatt"), code("002"))]
    pub avdragen_utlandsk_skatt: Option<i32>,
    #[ku(name(b"AvdragenKupongskatt"), code("003"))]
    pub avdragen_kupongskatt: Option<i32>,
    #[ku(name(b"Delagare"), code("061"))]
    pub delagare: Option<bool>,
    #[ku(name(b"Inkomstar"), code("203"), required(true))]
    pub inkomstar: Cow<'a, str>,
    #[ku(name(b"Borttag"), code("205"))]
    pub borttag: Option<bool>,
    #[ku(name(b"AnnanInkomst"), code("504"))]
    pub annan_inkomst: Option<i32>,
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
    #[ku(name(b"UtbetaldUtdelning"), code("574"))]
    pub utbetald_utdelning: Option<i32>,
    #[ku(name(b"AnnanKupongErsattning"), code("581"))]
    pub annan_kupong_ersattning: Option<i32>,
    #[ku(name(b"OkandVarde"), code("599"))]
    pub okand_varde: Option<bool>,
    #[ku(name(b"Avstamningsdag"), code("853"))]
    pub avstamningsdag: Option<KUDate>,
    #[ku(name(b"InkomsttagareKU31"), required(true), inner_ty(true))]
    pub inkomsttagare: InkomsttagareKU31<'a>,
    #[ku(name(b"UppgiftslamnareKU31"), required(true), inner_ty(true))]
    pub uppgiftslamnare: UppgiftslamnareKU31<'a>,
}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("InkomsttagareKU31"))]
pub struct InkomsttagareKU31<'a> {
    #[ku(name(b"LandskodTIN"), code("076"))]
    pub landskod_tin: Option<Landskod>,
    #[ku(name(b"Fodelseort"), code("077"))]
    pub fodelseort: Option<Cow<'a, str>>,
    #[ku(name(b"LandskodFodelseort"), code("077"))]
    pub landskod_fodelseort: Option<Landskod>,
    #[ku(name(b"LandskodHemvist"), code("079"))]
    pub landskod_hemvist: Option<Landskod>,
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
#[ku(name("UppgiftslamnareKU31"))]
pub struct UppgiftslamnareKU31<'a> {
    #[ku(name(b"UppgiftslamnarId"), code("201"), required(true))]
    pub uppgiftslamnar_id: Cow<'a, str>,
    #[ku(name(b"NamnUppgiftslamnare"), code("202"))]
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KontrolluppgiftType::{KU31};
    use crate::{
        from_str, to_string, Arendeinformation, Avsandare, Blankett, Blankettgemensamt,
        Kontaktperson, Kontrolluppgift, Landskod, TekniskKontaktperson, Uppgiftslamnare,
    };

    #[test]
    fn ku31_is_parsed_to_and_back() {
        let ku31 = Kontrolluppgift {
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
                blankettinnehall: KU31(KU31Type {
                    avdragen_skatt: Some(0),
                    avdragen_utlandsk_skatt: Some(1),
                    avdragen_kupongskatt: Some(2),
                    delagare: Some(false),
                    inkomstar: "2022".into(),
                    borttag: Some(true),
                    annan_inkomst: Some(3),
                    depanummer: Some(4),
                    andel_av_depan: Some(0.2),
                    specifikationsnummer: 5,
                    vp_namn: Some("test".into()),
                    isin: Some("isin".into()),
                    utbetald_utdelning: Some(6),
                    annan_kupong_ersattning: Some(7),
                    okand_varde: Some(false),
                    inkomsttagare: InkomsttagareKU31 {
                        landskod_tin: Some(Landskod::AF),
                        fodelseort: Some("Ort".into()),
                        landskod_fodelseort: Some(Landskod::AE),
                        landskod_hemvist: Some(Landskod::AD),
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
                    uppgiftslamnare: UppgiftslamnareKU31 {
                        uppgiftslamnar_id: "165599990602".into(),
                        namn_uppgiftslamnare: Some("Foretag 1".into()),
                    },
                    avstamningsdag: Some("20220804".parse().unwrap()),
                }),
            }],
        };
        let unparsed = to_string(&ku31).unwrap();
        let re_parsed = from_str(&*unparsed).unwrap();
        assert_eq!(ku31, re_parsed);
    }
}
