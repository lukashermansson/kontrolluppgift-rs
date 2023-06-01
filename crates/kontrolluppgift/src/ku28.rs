use std::borrow::Cow;
use kontrolluppgift_macro::{KontrolluppgiftRead, KontrolluppgiftWrite};

/// Kontrolluppgift 28
#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("KU28"))]
pub struct KU28Type<'a> {
    #[ku(name(b"Delagare"), code("061"))]
    pub delagare: Option<bool>,
    #[ku(name(b"Inkomstar"), code("203"), required(true))]
    pub inkomstar: Cow<'a, str>,
    #[ku(name(b"Borttag"), code("205"))]
    pub borttag: Option<bool>,
    #[ku(name(b"UnderlagForInvesteraravdrag"), code("528"))]
    pub underlag_for_investeraravdrag: Option<i32>,
    #[ku(name(b"TotUnderlagInvesteraravdrag"), code("529"))]
    pub tot_underlag_investeraravdrag: Option<i32>,
    #[ku(name(b"Betalningsar"), code("530"))]
    pub betalningsar: Option<Cow<'a, str>>,
    #[ku(name(b"AterforingAvyttring"), code("531"))]
    pub aterforing_avyttring: Option<bool>,
    #[ku(name(b"AterforingUtflyttning"), code("532"))]
    pub aterforing_utflyttning: Option<bool>,
    #[ku(name(b"AterforingHogVardeoverforing"), code("533"))]
    pub aterforing_hog_vardeoverforing: Option<bool>,
    #[ku(name(b"AterforingInternaForvarv"), code("534"))]
    pub aterforing_interna_forvarv: Option<bool>,
    #[ku(name(b"DatumForvarv"), code("535"))]
    pub datum_forvarv: Option<Cow<'a, str>>,
    #[ku(name(b"Region"), code("536"))]
    pub region: Option<Cow<'a, str>>,
    #[ku(name(b"Verksamhetsomrade"), code("537"))]
    pub verksamhetsomrade: Option<Cow<'a, str>>,
    #[ku(name(b"Specifikationsnummer"), code("570"), required(true))]
    pub specifikationsnummer: i32,
    #[ku(name(b"InkomsttagareKU28"), required(true), inner_ty(true))]
    pub inkomsttagare: InkomsttagareKU28<'a>,
    #[ku(name(b"UppgiftslamnareKU28"), required(true), inner_ty(true))]
    pub uppgiftslamnare: UppgiftslamnareKU28<'a>,

}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("InkomsttagareKU28"))]
pub struct InkomsttagareKU28<'a> {
    #[ku(name(b"LandskodTIN"), code("076"))]
    pub landskod_tin: Option<Cow<'a, str>>,
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
#[ku(name("UppgiftslamnareKU28"))]
pub struct UppgiftslamnareKU28<'a> {
    #[ku(name(b"UppgiftslamnarId"), code("201"), required(true))]
    pub uppgiftslamnar_id: Cow<'a, str>,
    #[ku(name(b"NamnUppgiftslamnare"), code("202"))]
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{Arendeinformation, Avsandare, Blankett, Blankettgemensamt, from_str, Kontaktperson, Kontrolluppgift, TekniskKontaktperson, to_string, Uppgiftslamnare};
    use crate::KontrolluppgiftType::KU28;
    use crate::ku28::{InkomsttagareKU28, KU28Type, UppgiftslamnareKU28};

    #[test]
    fn ku28_is_read() {
        let xml = fs::read_to_string("./EXEMPELFIL KONTROLLUPPGIFT INVESTERARAVDRAG (KU28)_2022.xml").unwrap();

        let parsed = from_str(&*xml).unwrap();
        let unparsed = to_string(&parsed).unwrap();
        let parsed2 = from_str(&*unparsed).unwrap();
        assert_eq!(parsed, parsed2);
    }

    #[test]
    fn ku28_is_parsed_to_and_back() {
        let ku28 = Kontrolluppgift {
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
                    blankettinnehall: KU28(KU28Type {
                        delagare: Some(false),
                        inkomstar: "2022".into(),
                        borttag: Some(true),
                        underlag_for_investeraravdrag: Some(1),
                        tot_underlag_investeraravdrag: Some(2),
                        betalningsar: Some("2023".into()),
                        aterforing_avyttring: Some(true),
                        aterforing_utflyttning: Some(false),
                        aterforing_hog_vardeoverforing: Some(true),
                        aterforing_interna_forvarv: Some(false),
                        datum_forvarv: Some("2021-01-01".into()),
                        region: Some("region".into()),
                        verksamhetsomrade: Some("verksamhetsomrade".into()),
                        specifikationsnummer: 5,
                        inkomsttagare: InkomsttagareKU28 {
                            landskod_tin: Some("landskod tin".into()),
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
                            tin: Some("Tin".into()),
                        },
                        uppgiftslamnare: UppgiftslamnareKU28 {
                            uppgiftslamnar_id: "165599990602".into(),
                            namn_uppgiftslamnare: Some("Foretag 1".into()),
                        },
                    }),
                }
            ],
        };
        let unparsed = to_string(&ku28).unwrap();
        let re_parsed = from_str(&*unparsed).unwrap();
        assert_eq!(ku28, re_parsed);
    }
}
