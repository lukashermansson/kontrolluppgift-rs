use std::borrow::Cow;
use kontrolluppgift_macros::{KontrolluppgiftRead, KontrolluppgiftWrite};
use crate::{IdentitetsbeteckningForPerson, Landskod, NarfartFjarrfart};

extern crate self as kontrolluppgift;

/// Kontrolluppgift 16
#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("KU16"))]
pub struct KU16Type<'a> {
    #[ku(name(b"KontantBruttolonMm"), code("011"))]
    pub kontant_bruttolon_mm: Option<i32>,
    #[ku(name(b"FormanUtomBilDrivmedel"), code("012"))]
    pub forman_utom_bil_drivmedel: Option<i32>,
    #[ku(name(b"AndraKostnadsers"), code("020"))]
    pub andra_kostnadsers: Option<i32>,
    #[ku(name(b"UnderlagRutarbete"), code("021"))]
    pub underlag_rutarbete: Option<i32>,
    #[ku(name(b"UnderlagRotarbete"), code("022"))]
    pub underlag_rotarbete: Option<i32>,
    #[ku(name(b"Fartygssignal"), code("026"))]
    pub fartygssignal: Option<Cow<'a, str>>,
    #[ku(name(b"AntalDagarSjoinkomst"), code("027"))]
    pub antal_dagar_sjoinkomst: Option<i32>, // todo restrict to 0-366 days
    #[ku(name(b"NarfartFjarrfart"), code("028"))]
    pub narfart_fjarrfart: Option<NarfartFjarrfart>,
    #[ku(name(b"ErsEjSocAvg"), code("031"))]
    pub ers_ej_soc_avg: Option<i32>,
    #[ku(name(b"Traktamente"), code("051"))]
    pub traktamente: Option<bool>,
    #[ku(name(b"Arbetsstallenummer"), code("060"))]
    pub arbetsstallenummer: Option<Cow<'a, str>>,
    #[ku(name(b"Delagare"), code("061"))]
    pub delagare: Option<bool>,
    #[ku(name(b"SocialAvgiftsAvtal"), code("093"))]
    pub social_avgifts_avtal: Option<bool>,
    #[ku(name(b"Inkomstar"), code("203"), required(true))]
    pub inkomstar: Cow<'a, str>,
    #[ku(name(b"Borttag"), code("205"))]
    pub borttag: Option<bool>,
    #[ku(name(b"FartygetsNamn"), code("223"))]
    pub fartygets_namn: Option<Cow<'a, str>>,
    #[ku(name(b"Specifikationsnummer"), code("570"), required(true))]
    pub specifikationsnummer: i32,
    #[ku(name(b"InkomsttagareKU16"), required(true), inner_ty(true))]
    pub inkomsttagare: InkomsttagareKU16<'a>,
    #[ku(name(b"UppgiftslamnareKU16"), required(true), inner_ty(true))]
    pub uppgiftslamnare: UppgiftslamnareKU16<'a>,
}


#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("UppgiftslamnareKU16"))]
pub struct UppgiftslamnareKU16<'a> {
    #[ku(name(b"UppgiftslamnarId"), code("201"), required(true))]
    pub uppgiftslamnar_id: Cow<'a, str>,
    #[ku(name(b"NamnUppgiftslamnare"), code("202"))]
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

#[derive(Debug, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("InkomsttagareKU16"))]
pub struct InkomsttagareKU16<'a> {
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



#[cfg(test)]
mod tests {
    use crate::{*};
    use crate::ku16::*;

    #[test]
    fn ku16_is_parsed_to_and_back() {
        let ku16 = Kontrolluppgift {
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
                    blankettinnehall: KU16(KU16Type {
                        kontant_bruttolon_mm: Some(1),
                        forman_utom_bil_drivmedel: Some(2),
                        andra_kostnadsers: Some(5),
                        underlag_rutarbete: Some(6),
                        underlag_rotarbete: Some(7),
                        fartygssignal: Some("TYPE".into()),
                        antal_dagar_sjoinkomst: Some(8),
                        narfart_fjarrfart: Some(NarfartFjarrfart::N),
                        ers_ej_soc_avg: Some(10),
                        traktamente: Some(true),
                        arbetsstallenummer: Some("12".into()),
                        delagare: Some(false),
                        social_avgifts_avtal: Some(true),
                        inkomstar: "2022".into(),
                        borttag: Some(false),
                        fartygets_namn: Some("Ship".into()),
                        specifikationsnummer: 5,
                        inkomsttagare: InkomsttagareKU16 {
                            landskod_tin: Some(Landskod::AI),
                            inkomsttagare: Some("191612299279".try_into().unwrap()),
                            fornamn: Some("Test".into()),
                            efternamn: Some("Testsson".into()),
                            gatuadress: Some("Gata".into()),
                            postnummer: Some("7456".into()),
                            postort: Some("Postort".into()),
                            landskod_postort: Some(Landskod::AD),
                            fodelsetid: Some("20230106".into()),
                            annat_id_nr: Some("202".into()),
                            org_namn: Some("Organization".into()),
                            gatuadress2: Some("Gata2".into()),
                            fri_adress: Some("Storgatan 3".into()),
                            tin: Some("Tin".into()),
                        },
                        uppgiftslamnare: UppgiftslamnareKU16 {
                            uppgiftslamnar_id: "165599990602".into(),
                            namn_uppgiftslamnare: Some("Foretag 1".into()),
                        },
                    }),
                }
            ],
        };
        let unparsed = to_string(&ku16).unwrap();
        let re_parsed = from_str(&*unparsed).unwrap();
        assert_eq!(ku16, re_parsed);
    }
}
