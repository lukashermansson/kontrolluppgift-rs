use kontrolluppgift_macro::{KontrolluppgiftRead, KontrolluppgiftWrite};
use std::borrow::Cow;

/// Kontrolluppgift 21
#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("KU21"))]
pub struct KU21Type<'a> {
    #[ku(name(b"AvdragenSkatt"), code("001"))]
    pub avdragen_skatt: Option<i32>,
    #[ku(name(b"Inkomstar"), code("203"), required(true))]
    pub inkomstar: Cow<'a, str>,
    #[ku(name(b"Borttag"), code("205"))]
    pub borttag: Option<bool>,
    #[ku(name(b"AnnanInkomst"), code("504"))]
    pub annan_inkomst: Option<i32>,
    #[ku(name(b"RantaFordringsratter"), code("520"))]
    pub ranta_fordringsratter: Option<i32>,
    #[ku(name(b"UtbetaltIVissaFall"), code("522"))]
    pub utbetalt_i_vissa_fall: Option<i32>,
    #[ku(name(b"Depanummer"), code("523"))]
    pub depanummer: Option<i32>,
    #[ku(name(b"AndelAvDepan"), code("524"))]
    pub andel_av_depan: Option<f32>,
    #[ku(name(b"ErhallenRantekompensation"), code("525"))]
    pub erhallen_rantekompensation: Option<f32>,
    #[ku(name(b"Specifikationsnummer"), code("570"), required(true))]
    pub specifikationsnummer: i32,
    #[ku(name(b"VPNamn"), code("571"))]
    pub vp_namn: Option<Cow<'a, str>>,
    #[ku(name(b"ISIN"), code("572"))]
    pub isin: Option<Cow<'a, str>>,
    #[ku(name(b"AvyttradTillISK"), code("573"))]
    pub avyttrad_till_isk: Option<bool>,
    #[ku(name(b"OkandVarde"), code("599"))]
    pub okand_varde: Option<bool>,

    #[ku(name(b"InkomsttagareKU21"), required(true))]
    pub inkomsttagare: InkomsttagareKU21<'a>,
    #[ku(name(b"UppgiftslamnareKU21"), required(true))]
    pub uppgiftslamnare: UppgiftslamnareKU21<'a>,
}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("InkomsttagareKU21"))]
pub struct InkomsttagareKU21<'a> {
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
#[ku(name("UppgiftslamnareKU21"))]
pub struct UppgiftslamnareKU21<'a> {
    #[ku(name(b"UppgiftslamnarId"), code("201"), required(true))]
    pub uppgiftslamnar_id: Cow<'a, str>,
    #[ku(name(b"NamnUppgiftslamnare"), code("202"))]
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

#[cfg(test)]
mod tests {
    use crate::ku21::{InkomsttagareKU21, KU21Type, UppgiftslamnareKU21};
    use crate::KontrolluppgiftType::KU21;
    use crate::{
        from_str, to_string, Arendeinformation, Avsandare, Blankett, Blankettgemensamt,
        Kontaktperson, Kontrolluppgift, TekniskKontaktperson, Uppgiftslamnare,
    };
    use std::fs;

    #[test]
    fn ku21_is_read() {
        let xml = fs::read_to_string(
            "./EXEMPELFIL KONTROLLUPPGIFTER RÄNTA, UTDELNING M.M. KU21 FÖR_2022.xml",
        )
        .unwrap();

        let parsed = from_str(&*xml).unwrap();
        let unparsed = to_string(&parsed).unwrap();
        let parsed2 = from_str(&*unparsed).unwrap();
        assert_eq!(parsed, parsed2);
    }

    #[test]
    fn ku21_is_parsed_to_and_back() {
        let ku21 = Kontrolluppgift {
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
                blankettinnehall: KU21(KU21Type {
                    avdragen_skatt: Some(1),
                    inkomstar: "2022".into(),
                    borttag: Some(true),
                    annan_inkomst: Some(4),
                    ranta_fordringsratter: Some(5),
                    utbetalt_i_vissa_fall: Some(6),
                    depanummer: Some(7),
                    andel_av_depan: Some(8.0),
                    erhallen_rantekompensation: Some(9.0),
                    specifikationsnummer: 5,
                    vp_namn: Some("vp namn".into()),
                    isin: Some("isin".into()),
                    avyttrad_till_isk: Some(false),
                    okand_varde: Some(true),
                    inkomsttagare: InkomsttagareKU21 {
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
                    uppgiftslamnare: UppgiftslamnareKU21 {
                        uppgiftslamnar_id: "165599990602".into(),
                        namn_uppgiftslamnare: Some("Foretag 1".into()),
                    },
                }),
            }],
        };
        let unparsed = to_string(&ku21).unwrap();
        let re_parsed = from_str(&*unparsed).unwrap();
        assert_eq!(ku21, re_parsed);
    }
}
