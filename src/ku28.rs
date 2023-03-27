use std::borrow::Cow;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{NsReader, Writer};
use crate::error::Error;
use crate::{Reader, unexpected_element, Write};
use crate::error::Error::MissingElement;

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
            w.write_node_with_code("Specifikationsnummer", "570", self.specifikationsnummer)?;

            self.inkomsttagare.write(w)?;
            self.uppgiftslamnare.write(w)?;
            Ok(())
        })?;
        Ok(())
    }
}

impl<'a> KU28Type<'a> {
    pub(crate) fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, Error> {
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
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Delagare" => reader.read_node_into_with_code(element, "061", &mut delagare)?,
                    b"Inkomstar" => reader.read_node_into_with_code(element, "203", &mut inkomstar)?,
                    b"Borttag" => reader.read_node_into_with_code(element, "205", &mut borttag)?,
                    b"UnderlagForInvesteraravdrag" => reader.read_node_into_with_code(element, "528", &mut underlag_for_investeraravdrag)?,
                    b"TotUnderlagInvesteraravdrag" => reader.read_node_into_with_code(element, "529", &mut tot_underlag_investeraravdrag)?,
                    b"Betalningsar" => reader.read_node_into_with_code(element, "530", &mut betalningsar)?,
                    b"AterforingAvyttring" => reader.read_node_into_with_code(element, "531", &mut aterforing_avyttring)?,
                    b"AterforingUtflyttning" => reader.read_node_into_with_code(element, "532", &mut aterforing_utflyttning)?,
                    b"AterforingHogVardeoverforing" => reader.read_node_into_with_code(element, "533", &mut aterforing_hog_vardeoverforing)?,
                    b"AterforingInternaForvarv" => reader.read_node_into_with_code(element, "534", &mut aterforing_interna_forvarv)?,
                    b"DatumForvarv" => reader.read_node_into_with_code(element, "535", &mut datum_forvarv)?,
                    b"Region" => reader.read_node_into_with_code(element, "536", &mut region)?,
                    b"Verksamhetsomrade" => reader.read_node_into_with_code(element, "537", &mut verksamhetsomrade)?,
                    b"Specifikationsnummer" => reader.read_node_into_with_code(element, "570", &mut specificationsnummer)?,
                    b"InkomsttagareKU28" => {
                        inkomsttagare = Some(InkomsttagareKU28::read(reader, &element)?)
                    }
                    b"UppgiftslamnareKU28" => {
                        uppgiftslamnare = Some(UppgiftslamnareKU28::read(reader, &element)?)
                    }
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            delagare,
                            inkomstar: inkomstar.ok_or_else(|| MissingElement("Inkomstar".to_string()))?,
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
                            specifikationsnummer: specificationsnummer.ok_or_else(|| MissingElement("Specifikationsnummer".to_string()))?,
                            inkomsttagare: inkomsttagare.ok_or_else(|| MissingElement("Inkomsttagare".to_string()))?,
                            uppgiftslamnare: uppgiftslamnare.ok_or_else(|| MissingElement("Uppgiftslamnare".to_string()))?,
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading KU28".to_string())),
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
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, Error> {
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
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"LandskodTIN" => reader.read_node_into_with_code(element, "076", &mut landskod_tin)?,
                    b"Inkomsttagare" => reader.read_node_into_with_code(element, "215", &mut inkomsttagare)?,
                    b"Fornamn" => reader.read_node_into_with_code(element, "216", &mut fornamn)?,
                    b"Efternamn" => reader.read_node_into_with_code(element, "217", &mut efternamn)?,
                    b"Gatuadress" => reader.read_node_into_with_code(element, "218", &mut gatuadress)?,
                    b"Postnummer" => reader.read_node_into_with_code(element, "219", &mut postnummer)?,
                    b"Postort" => reader.read_node_into_with_code(element, "220", &mut postort)?,
                    b"LandskodPostort" => reader.read_node_into_with_code(element, "221", &mut landskod_postort)?,
                    b"Fodelsetid" => reader.read_node_into_with_code(element, "222", &mut fodelsetid)?,
                    b"AnnatIDNr" => reader.read_node_into_with_code(element, "224", &mut annat_id_nr)?,
                    b"OrgNamn" => reader.read_node_into_with_code(element, "226", &mut org_namn)?,
                    b"Gatuadress2" => reader.read_node_into_with_code(element, "228", &mut gatuadress2)?,
                    b"FriAdress" => reader.read_node_into_with_code(element, "230", &mut fri_adress)?,
                    b"TIN" => reader.read_node_into_with_code(element, "252", &mut tin)?,
                    &_ => unexpected_element(&element)?
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
                Event::Eof => return Err(Error::UnexpectedEof("While reading InkomsttagareKU28".to_string())),
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
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, Error> {
        let mut uppgiftslamnar_id = None;
        let mut namn_uppgiftslamnare = None;
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"UppgiftslamnarId" => reader.read_node_into_with_code(element, "201", &mut uppgiftslamnar_id)?,
                    b"NamnUppgiftslamnare" => reader.read_node_into_with_code(element, "202", &mut namn_uppgiftslamnare)?,
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            uppgiftslamnar_id: uppgiftslamnar_id.ok_or_else(|| MissingElement("UppgiftslamnarId".to_string()))?,
                            namn_uppgiftslamnare,
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading UppgiftslamnareKU28".to_string())),
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
