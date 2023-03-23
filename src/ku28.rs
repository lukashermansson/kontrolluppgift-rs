use std::borrow::Cow;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{NsReader, Writer};
use crate::{DeError, to_bool, Write};
use crate::DeError::{MissingField, UnexpectedElement};

/// Kontrolluppgift 28
#[derive(Debug, PartialEq)]
pub struct KU28Type<'a> {
    pub delagare: Option<bool>,
    pub inkomstar: Cow<'a, str>,
    pub borttag: Option<bool>,
    pub underlag_for_investeraravdrag: Option<i32>,
    pub tot_underlag_investeraravdrag: Option<i32>,
    pub betalningsar: Option<Cow<'a, str>>,
    pub aterforing_avyttring: Option<bool>,
    pub aterforing_utflyttning: Option<bool>,
    pub aterforing_hog_vardeoverforing: Option<bool>,
    pub aterforing_interna_forvarv: Option<bool>,
    pub datum_forvarv: Option<Cow<'a, str>>,
    pub region: Option<Cow<'a, str>>,
    pub verksamhetsomrade: Option<Cow<'a, str>>,
    pub specifikationsnummer: i32,
    pub inkomsttagare: InkomsttagareKU28<'a>,
    pub uppgiftslamnare: UppgiftslamnareKU28<'a>,

}

impl<'a> KU28Type<'a> {
    pub(crate) fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("KU28").write_inner_content(|w| {
            w.write_node_with_code("Delagare", "061", &self.delagare)?;
            w.write_node_with_code("Inkomstar", "203", &self.inkomstar)?;
            w.write_node_with_code("Borttag", "205", &self.borttag)?;
            w.write_node_with_code("UnderlagForInvesteraravdrag", "528", &self.underlag_for_investeraravdrag)?;
            w.write_node_with_code("TotUnderlagInvesteraravdrag", "529", &self.tot_underlag_investeraravdrag)?;
            w.write_node_with_code("Betalningsar", "530", &self.betalningsar)?;
            w.write_node_with_code("AterforingAvyttring", "531", &self.aterforing_avyttring)?;
            w.write_node_with_code("AterforingUtflyttning", "532", &self.aterforing_utflyttning)?;
            w.write_node_with_code("AterforingHogVardeoverforing", "533", &self.aterforing_hog_vardeoverforing)?;
            w.write_node_with_code("AterforingInternaForvarv", "534", &self.aterforing_interna_forvarv)?;
            w.write_node_with_code("DatumForvarv", "535", &self.datum_forvarv)?;
            w.write_node_with_code("Region", "536", &self.region)?;
            w.write_node_with_code("Verksamhetsomrade", "537", &self.verksamhetsomrade)?;
            w.write_node_with_code("Specifikationsnummer", "570", &self.specifikationsnummer)?;

            self.inkomsttagare.write(w)?;
            self.uppgiftslamnare.write(w)?;
            Ok(())
        })?;
        Ok(())
    }
}

impl<'a> KU28Type<'a> {
    pub(crate) fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {
        let mut delagare = None;
        let mut inkomstar = None;
        let mut borttag = None;
        let mut underlag_for_investeraravdrag = None;
        let mut tot_underlag_investeraravdrag = None;
        let mut betalningsar = None;
        let mut aterforing_avyttring = None;
        let mut aterforing_utflyttning = None;
        let mut aterforing_hog_vardeoverforing = None;
        let mut aterforing_interna_forvarv = None;
        let mut datum_forvarv = None;
        let mut region = None;
        let mut verksamhetsomrade = None;
        let mut specificationsnummer = None;
        let mut inkomsttagare = None;
        let mut uppgiftslamnare = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Delagare" => {
                        delagare = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"Inkomstar" => {
                        inkomstar = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Borttag" => {
                        borttag = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"UnderlagForInvesteraravdrag" => {
                        underlag_for_investeraravdrag = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"TotUnderlagInvesteraravdrag" => {
                        tot_underlag_investeraravdrag = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"Betalningsar" => {
                        betalningsar = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"AterforingAvyttring" => {
                        aterforing_avyttring = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"AterforingUtflyttning" => {
                        aterforing_utflyttning = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"AterforingHogVardeoverforing" => {
                        aterforing_hog_vardeoverforing = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"AterforingInternaForvarv" => {
                        aterforing_interna_forvarv = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"DatumForvarv" => {
                        datum_forvarv = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Region" => {
                        region = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Verksamhetsomrade" => {
                        verksamhetsomrade = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Specifikationsnummer" => {
                        specificationsnummer = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }

                    b"InkomsttagareKU28" => {
                        inkomsttagare = Some(InkomsttagareKU28::read(reader, &element)?)
                    }
                    b"UppgiftslamnareKU28" => {
                        uppgiftslamnare = Some(UppgiftslamnareKU28::read(reader, &element)?)
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            delagare,
                            inkomstar: inkomstar.ok_or_else(|| MissingField("Inkomstar".to_string()))?,
                            borttag,
                            underlag_for_investeraravdrag,
                            tot_underlag_investeraravdrag,
                            betalningsar,
                            aterforing_avyttring,
                            aterforing_utflyttning,
                            aterforing_hog_vardeoverforing,
                            aterforing_interna_forvarv,
                            datum_forvarv,
                            region,
                            verksamhetsomrade,
                            specifikationsnummer: specificationsnummer.ok_or_else(|| MissingField("Specifikationsnummer".to_string()))?,
                            inkomsttagare: inkomsttagare.ok_or_else(|| MissingField("InkomsttagareKU28".to_string()))?,
                            uppgiftslamnare: uppgiftslamnare.ok_or_else(|| MissingField("UppgiftslamnareKU28".to_string()))?,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InkomsttagareKU28<'a> {
    pub landskod_tin: Option<Cow<'a, str>>,
    pub inkomsttagare: Option<Cow<'a, str>>,
    pub fornamn: Option<Cow<'a, str>>,
    pub efternamn: Option<Cow<'a, str>>,
    pub gatuadress: Option<Cow<'a, str>>,
    pub postnummer: Option<Cow<'a, str>>,
    pub postort: Option<Cow<'a, str>>,
    pub landskod_postort: Option<Cow<'a, str>>,
    pub fodelsetid: Option<Cow<'a, str>>,
    pub annat_id_nr: Option<Cow<'a, str>>,
    pub org_namn: Option<Cow<'a, str>>,
    pub gatuadress2: Option<Cow<'a, str>>,
    pub fri_adress: Option<Cow<'a, str>>,
    pub tin: Option<Cow<'a, str>>,
}

impl<'a> InkomsttagareKU28<'a> {
    fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("InkomsttagareKU28").write_inner_content(|w| {
            w.write_node_with_code("LandskodTIN", "076", &self.landskod_tin)?;
            w.write_node_with_code("Inkomsttagare", "215", &self.inkomsttagare)?;
            w.write_node_with_code("Fornamn", "216", &self.fornamn)?;
            w.write_node_with_code("Efternamn", "217", &self.efternamn)?;
            w.write_node_with_code("Gatuadress", "218", &self.gatuadress)?;
            w.write_node_with_code("Postnummer", "219", &self.postnummer)?;
            w.write_node_with_code("Postort", "220", &self.postort)?;
            w.write_node_with_code("LandskodPostort", "221", &self.landskod_postort)?;
            w.write_node_with_code("Fodelsetid", "222", &self.fodelsetid)?;
            w.write_node_with_code("AnnatIDNr", "224", &self.annat_id_nr)?;
            w.write_node_with_code("OrgNamn", "226", &self.org_namn)?;
            w.write_node_with_code("Gatuadress2", "228", &self.gatuadress2)?;
            w.write_node_with_code("FriAdress", "230", &self.fri_adress)?;
            w.write_node_with_code("TIN", "252", &self.tin)?;

            Ok(())
        })?;

        Ok(())
    }
}

impl<'a> InkomsttagareKU28<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {
        let mut landskod_tin = None;
        let mut inkomsttagare = None;
        let mut fornamn = None;
        let mut efternamn = None;
        let mut gatuadress = None;
        let mut postnummer = None;
        let mut postort = None;
        let mut landskod_postort = None;
        let mut fodelsetid = None;
        let mut annat_id_nr = None;
        let mut org_namn = None;
        let mut gatuadress2 = None;
        let mut fri_adress = None;
        let mut tin = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"LandskodTIN" => {
                        landskod_tin = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Inkomsttagare" => {
                        inkomsttagare = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Fornamn" => {
                        fornamn = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Efternamn" => {
                        efternamn = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Gatuadress" => {
                        gatuadress = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Postnummer" => {
                        postnummer = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Postort" => {
                        postort = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"LandskodPostort" => {
                        landskod_postort = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Fodelsetid" => {
                        fodelsetid = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"AnnatIDNr" => {
                        annat_id_nr = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"OrgNamn" => {
                        org_namn = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Gatuadress2" => {
                        gatuadress2 = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"FriAdress" => {
                        fri_adress = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"TIN" => {
                        tin = Some(reader.read_text(element.name()).unwrap());
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            landskod_tin,
                            inkomsttagare,
                            fornamn,
                            efternamn,
                            gatuadress,
                            postnummer,
                            postort,
                            landskod_postort,
                            fodelsetid,
                            annat_id_nr,
                            org_namn,
                            gatuadress2,
                            fri_adress,
                            tin,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UppgiftslamnareKU28<'a> {
    pub uppgiftslamnar_id: Cow<'a, str>,
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

impl<'a> UppgiftslamnareKU28<'a> {
    fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("UppgiftslamnareKU28").write_inner_content(|w| {
            w.write_node_with_code("UppgiftslamnarId", "201", &self.uppgiftslamnar_id)?;
            w.write_node_with_code("NamnUppgiftslamnare", "202", &self.namn_uppgiftslamnare)?;

            Ok(())
        })?;

        Ok(())
    }
}

impl<'a> UppgiftslamnareKU28<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {
        let mut uppgiftslamnar_id = None;
        let mut namn_uppgiftslamnare = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"UppgiftslamnarId" => {
                        uppgiftslamnar_id = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"NamnUppgiftslamnare" => {
                        namn_uppgiftslamnare = Some(reader.read_text(element.name()).unwrap());
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            uppgiftslamnar_id: uppgiftslamnar_id.ok_or_else(|| MissingField("UppgiftslamnarId".to_string()))?,
                            namn_uppgiftslamnare,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{Arendeinformation, Avsandare, Blankett, Blankettgemensamt, from_str, Kontaktperson, Kontrolluppgift, TekniskKontaktperson, to_string, Uppgiftslamnare};
    use crate::KontrolluppgiftType::KU28;
    use crate::ku28::{InkomsttagareKU28, KU28Type, UppgiftslamnareKU28};

    #[test]
    fn ku28_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFT INVESTERARAVDRAG (KU28)_2022.xml").unwrap();

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
