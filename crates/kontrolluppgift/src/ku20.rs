use std::borrow::Cow;
use kontrolluppgift_macro::{KontrolluppgiftRead, KontrolluppgiftWrite};

/// Kontrolluppgift 20
#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("KU20"))]
pub struct KU20Type<'a> {
    #[ku(name(b"AvdragenSkatt"), code("001"))]
    pub avdragen_skatt: Option<i32>,
    #[ku(name(b"Delagare"), code("061"))]
    pub delagare: Option<bool>,
    #[ku(name(b"Inkomstar"), code("203"), required(true))]
    pub inkomstar: Cow<'a, str>,
    #[ku(name(b"Borttag"), code("205"))]
    pub borttag: Option<bool>,
    #[ku(name(b"Ranteinkomst"), code("500"))]
    pub ranteinkomst: Option<i32>,
    #[ku(name(b"Forfogarkonto"), code("502"))]
    pub forfogarkonto: Option<bool>,
    #[ku(name(b"RanteinkomstEjKonto"), code("503"))]
    pub ranteinkomst_ej_konto: Option<i32>,
    #[ku(name(b"AnnanInkomst"), code("504"))]
    pub annan_inkomst: Option<i32>,
    #[ku(name(b"Specifikationsnummer"), code("570"), required(true))]
    pub specifikationsnummer: i32,
    #[ku(name(b"InkomsttagareKU20"), required(true), inner_ty(true))]
    pub inkomsttagare: InkomsttagareKU20<'a>,
    #[ku(name(b"UppgiftslamnareKU20"), required(true), inner_ty(true))]
    pub uppgiftslamnare: UppgiftslamnareKU20<'a>,

}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("InkomsttagareKU20"))]
pub struct InkomsttagareKU20<'a> {
    #[ku(name(b"LandskodTIN"), code("076"))]
    pub landskod_tin: Option<Cow<'a, str>>,
    #[ku(name(b"Fodelseort"), code("077"))]
    pub fodelseort: Option<Cow<'a, str>>,
    #[ku(name(b"LandskodFodelseort"), code("078"))]
    pub landskod_fodelseort: Option<Cow<'a, str>>,
    #[ku(name(b"Inkomsttagare"), code("215"))]
    pub inkomsttagare: Option<Cow<'a, str>>,
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
    pub landskod_postort: Option<Cow<'a, str>>,
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
#[ku(name("UppgiftslamnareKU20"))]
pub struct UppgiftslamnareKU20<'a> {
    #[ku(name(b"UppgiftslamnarId"), code("201"), required(true))]
    pub uppgiftslamnar_id: Cow<'a, str>,
    #[ku(name(b"NamnUppgiftslamnare"), code("202"))]
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{Arendeinformation, Avsandare, Blankett, Blankettgemensamt, from_str, Kontaktperson, Kontrolluppgift, TekniskKontaktperson, to_string, Uppgiftslamnare};
    use crate::KontrolluppgiftType::KU20;
    use crate::ku20::{InkomsttagareKU20, KU20Type, UppgiftslamnareKU20};

    #[test]
    fn ku20_is_read() {
        let xml = fs::read_to_string("./EXEMPELFIL KONTROLLUPPGIFTER RÄNTA, UTDELNING M.M. KU20 FÖR_2022.xml").unwrap();

        let parsed = from_str(&*xml).unwrap();
        let unparsed = to_string(&parsed).unwrap();
        let parsed2 = from_str(&*unparsed).unwrap();
        assert_eq!(parsed, parsed2);
    }

    #[test]
    fn ku20_is_parsed_to_and_back() {
        let ku20 = Kontrolluppgift {
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
                }
            },
            blanketter: vec![
                Blankett {
                    nummer: 0,
                    arendeinformation: Arendeinformation {
                        ..Default::default()
                    },
                    blankettinnehall: KU20(KU20Type {
                        avdragen_skatt: Some(1),
                        delagare: Some(true),
                        inkomstar: "2022".into(),
                        borttag: Some(true),
                        ranteinkomst: Some(2),
                        forfogarkonto: Some(false),
                        ranteinkomst_ej_konto: Some(3),
                        annan_inkomst: Some(4),
                        specifikationsnummer: 5,
                        inkomsttagare: InkomsttagareKU20 {
                            landskod_tin: Some("landskod tin".into()),
                            fodelseort: Some("Ort".into()),
                            landskod_fodelseort: Some("SE".into()),
                            inkomsttagare: Some("202301062382".into()),
                            fornamn: Some("Test".into()),
                            efternamn: Some("Testsson".into()),
                            gatuadress: Some("Gata".into()),
                            postnummer: Some("7456".into()),
                            postort: Some("Postort".into()),
                            landskod_postort: Some("FI".into()),
                            fodelsetid: Some("20230106".into()),
                            annat_id_nr: Some("202".into()),
                            org_namn: Some("Organization".into()),
                            gatuadress2: Some("Gata2".into()),
                            fri_adress: Some("Storgatan 3".into()),
                            tin: Some("TIN".into()),
                        },
                        uppgiftslamnare: UppgiftslamnareKU20 {
                            uppgiftslamnar_id: "165599990602".into(),
                            namn_uppgiftslamnare: Some("Foretag 1".into()),
                        },
                    }),
                }
            ],
        };
        let unparsed = to_string(&ku20).unwrap();
        let re_parsed = from_str(&*unparsed).unwrap();
        assert_eq!(ku20, re_parsed);
    }
}
