use crate::{error, IdentitetsbeteckningForPerson, Landskod};
use kontrolluppgift_macros::{KUStringEnum, KontrolluppgiftRead, KontrolluppgiftWrite};
use std::borrow::Cow;

extern crate self as kontrolluppgift;

/// Kontrolluppgift 14
#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("KU14"))]
pub struct KU14Type<'a> {
    #[ku(name(b"KontantBruttolonMm"), code("011"))]
    pub kontant_bruttolon_mm: Option<i32>,
    #[ku(name(b"FormanUtomBilDrivmedel"), code("012"))]
    pub forman_utom_bil_drivmedel: Option<i32>,
    #[ku(name(b"BilformanUtomDrivmedel"), code("013"))]
    pub bilforman_utom_drivmedel: Option<i32>,
    #[ku(name(b"DrivmedelVidBilforman"), code("018"))]
    pub drivmedel_vid_bilforman: Option<i32>,
    #[ku(name(b"AndraKostnadsers"), code("020"))]
    pub andra_kostnadsers: Option<i32>,
    #[ku(name(b"UnderlagRutarbete"), code("021"))]
    pub underlag_rutarbete: Option<i32>,
    #[ku(name(b"UnderlagRotarbete"), code("022"))]
    pub underlag_rotarbete: Option<i32>,
    #[ku(name(b"ErsMEgenavgifter"), code("025"))]
    pub ers_m_egenavgifter: Option<i32>,
    #[ku(name(b"Tjanstepension"), code("030"))]
    pub tjanstepension: Option<i32>,
    #[ku(name(b"ErsEjSocAvg"), code("031"))]
    pub ers_ej_soc_avg: Option<i32>,
    #[ku(name(b"Forskarskattenamnden"), code("035"))]
    pub forsarskattenamnden: Option<i32>,
    #[ku(name(b"BostadSmahus"), code("041"))]
    pub bostad_smahus: Option<bool>,
    #[ku(name(b"BostadEjSmahus"), code("043"))]
    pub bostad_ej_smahus: Option<bool>,
    #[ku(name(b"FormanHarJusterats"), code("048"))]
    pub forman_har_justerats: Option<bool>,
    #[ku(name(b"FormanSomPension"), code("049"))]
    pub forman_som_pension: Option<bool>,
    #[ku(name(b"Bilersattning"), code("050"))]
    pub bilersattning: Option<bool>,
    #[ku(name(b"Traktamente"), code("051"))]
    pub traktamente: Option<bool>,
    #[ku(name(b"PersonaloptionForvarvAndel"), code("059"))]
    pub personaloption_forvarv_andel: Option<bool>,
    #[ku(name(b"Arbetsstallenummer"), code("060"))]
    pub arbetsstallenummer: Option<Cow<'a, str>>,
    #[ku(name(b"Delagare"), code("061"))]
    pub delagare: Option<bool>,
    #[ku(name(b"LandskodArbetsland"), code("090"))]
    pub landskod_arbetsland: Option<Cow<'a, str>>,
    #[ku(name(b"UtsandUnderTid"), code("091"))]
    pub utsand_under_tid: Option<KU14UtsandUnderTid>,
    #[ku(name(b"Kategori"), code("092"))]
    pub kategori: Option<KU14Kategori>,
    #[ku(name(b"SocialAvgiftsAvtal"), code("093"))]
    pub social_avgifts_avtal: Option<bool>,
    #[ku(name(b"Inkomstar"), code("203"), required(true))]
    pub inkomstar: Cow<'a, str>,
    #[ku(name(b"Borttag"), code("205"))]
    pub borttag: Option<bool>,
    #[ku(name(b"Specifikationsnummer"), code("570"), required(true))]
    pub specifikationsnummer: i32,
    #[ku(name(b"InkomsttagareKU14"), required(true), inner_ty(true))]
    pub inkomsttagare: InkomsttagareKU14<'a>,
    #[ku(name(b"UppgiftslamnareKU14"), required(true), inner_ty(true))]
    pub uppgiftslamnare: UppgiftslamnareKU14<'a>,
}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("UppgiftslamnareKU14"))]
pub struct UppgiftslamnareKU14<'a> {
    #[ku(name(b"UppgiftslamnarId"), code("201"), required(true))]
    pub uppgiftslamnar_id: Cow<'a, str>,
    #[ku(name(b"NamnUppgiftslamnare"), code("202"))]
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("InkomsttagareKU14"))]
pub struct InkomsttagareKU14<'a> {
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

#[derive(Debug, PartialEq, KUStringEnum)]
pub enum KU14Kategori {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Debug, PartialEq, KUStringEnum)]
pub enum KU14UtsandUnderTid {
    A,
    B,
    C,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KontrolluppgiftType::KU14;
    use crate::{
        from_str, to_string, Arendeinformation, Avsandare, Blankett, Blankettgemensamt,
        Kontaktperson, Kontrolluppgift, TekniskKontaktperson, Uppgiftslamnare,
    };

    #[test]
    fn ku14_is_parsed_to_and_back() {
        let ku14 = Kontrolluppgift {
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
                blankettinnehall: KU14(KU14Type {
                    kontant_bruttolon_mm: Some(1),
                    forman_utom_bil_drivmedel: Some(2),
                    bilforman_utom_drivmedel: Some(3),
                    drivmedel_vid_bilforman: Some(4),
                    andra_kostnadsers: Some(5),
                    underlag_rutarbete: Some(6),
                    underlag_rotarbete: Some(7),
                    ers_m_egenavgifter: Some(8),
                    tjanstepension: Some(9),
                    ers_ej_soc_avg: Some(10),
                    forsarskattenamnden: None,
                    bostad_smahus: Some(true),
                    bostad_ej_smahus: Some(false),
                    forman_har_justerats: Some(true),
                    forman_som_pension: Some(true),
                    bilersattning: Some(true),
                    traktamente: Some(true),
                    personaloption_forvarv_andel: Some(true),
                    arbetsstallenummer: Some("12".into()),
                    delagare: Some(false),
                    landskod_arbetsland: Some("FI".into()),
                    utsand_under_tid: Some(KU14UtsandUnderTid::A),
                    kategori: Some(KU14Kategori::B),
                    social_avgifts_avtal: Some(true),
                    inkomstar: "2022".into(),
                    borttag: Some(false),

                    specifikationsnummer: 5,
                    inkomsttagare: InkomsttagareKU14 {
                        landskod_tin: Some(Landskod::SE),
                        landskod_medborgare: Some(Landskod::FI),
                        inkomsttagare: Some("191612299279".try_into().unwrap()),
                        fornamn: Some("Test".into()),
                        efternamn: Some("Testsson".into()),
                        gatuadress: Some("Gata".into()),
                        postnummer: Some("7456".into()),
                        postort: Some("Postort".into()),
                        landskod_postort: Some(Landskod::AF),
                        fodelsetid: Some("20230106".into()),
                        annat_id_nr: Some("202".into()),
                        org_namn: Some("Organization".into()),
                        gatuadress2: Some("Gata2".into()),
                        fri_adress: Some("Storgatan 3".into()),
                        tin: Some("Tin".into()),
                    },
                    uppgiftslamnare: UppgiftslamnareKU14 {
                        uppgiftslamnar_id: "165599990602".into(),
                        namn_uppgiftslamnare: Some("Foretag 1".into()),
                    },
                }),
            }],
        };
        let unparsed = to_string(&ku14).unwrap();
        let re_parsed = from_str(&*unparsed).unwrap();
        assert_eq!(ku14, re_parsed);
    }
}
