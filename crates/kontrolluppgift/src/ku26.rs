use std::borrow::Cow;
use kontrolluppgift_macro::{KontrolluppgiftRead, KontrolluppgiftWrite};

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("KU26"))]
pub struct KU26Type<'a> {
    #[ku(name(b"Inkomstar"), code("203"), required(true))]
    pub inkomstar: Cow<'a, str>,
    #[ku(name(b"Borttag"), code("205"))]
    pub borttag: Option<bool>,
    #[ku(name(b"BetaldTomtrattsavgald"), code("560"))]
    pub betald_tomttsavgald: Option<i32>,
    #[ku(name(b"Fastighetsbeteckning"), code("561"))]
    pub fastighetsbeteckning: Option<Cow<'a, str>>,
    #[ku(name(b"Specifikationsnummer"), code("570"), required(true))]
    pub specifikationsnummer: i32,
    #[ku(name(b"InkomsttagareKU26"), required(true), inner_ty(true))]
    pub inkomsttagare: InkomsttagareKU26<'a>,
    #[ku(name(b"UppgiftslamnareKU26"), required(true), inner_ty(true))]
    pub uppgiftslamnare: UppgiftslamnareKU26<'a>,
}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("InkomsttagareKU26"))]
pub struct InkomsttagareKU26<'a> {
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
}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("UppgiftslamnareKU26"))]
pub struct UppgiftslamnareKU26<'a> {
    #[ku(name(b"UppgiftslamnarId"), code("201"), required(true))]
    pub uppgiftslamnar_id: Cow<'a, str>,
    #[ku(name(b"NamnUppgiftslamnare"), code("202"))]
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{Arendeinformation, Avsandare, Blankett, Blankettgemensamt, from_str, Kontaktperson, Kontrolluppgift, TekniskKontaktperson, to_string, Uppgiftslamnare};
    use crate::KontrolluppgiftType::{ KU26};
    use crate::ku26::{InkomsttagareKU26, KU26Type, UppgiftslamnareKU26};

    #[test]
    fn ku26_is_read() {
        let xml = fs::read_to_string("./EXEMPELFIL KONTROLLUPPGIFT TOMTRÄTTSAVGÄLD (KU26) FÖR KOMMUNER M.FL._2022.xml").unwrap();

        let parsed = from_str(&*xml).unwrap();
        let unparsed = to_string(&parsed).unwrap();
        let parsed2 = from_str(&*unparsed).unwrap();
        assert_eq!(parsed, parsed2);
    }

    #[test]
    fn ku26_is_parsed_to_and_back() {
        let ku26 = Kontrolluppgift {
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
                    blankettinnehall: KU26(KU26Type {
                        inkomstar: "2022".into(),
                        borttag: Some(true),
                        betald_tomttsavgald: Some(1),
                        fastighetsbeteckning: Some("fastighetsbeteckning".into()),
                        specifikationsnummer: 5,
                        inkomsttagare: InkomsttagareKU26 {
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
                        },
                        uppgiftslamnare: UppgiftslamnareKU26 {
                            uppgiftslamnar_id: "165599990602".into(),
                            namn_uppgiftslamnare: Some("Foretag 1".into()),
                        },
                    }),
                }
            ],
        };
        let unparsed = to_string(&ku26).unwrap();
        let re_parsed = from_str(&*unparsed).unwrap();
        assert_eq!(ku26, re_parsed);
    }
}
