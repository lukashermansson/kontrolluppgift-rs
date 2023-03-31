use std::borrow::Cow;
use quick_xml::{NsReader, Writer};
use quick_xml::events::{BytesStart, Event};
use crate::{Reader, unexpected_element, Write};
use crate::error::Error;
use crate::error::Error::MissingElement;

#[derive(Debug, PartialEq)]
pub struct KU26Type<'a> {
    pub inkomstar: Cow<'a, str>,
    pub borttag: Option<bool>,
    pub betald_tomttsavgald: Option<i32>,
    pub fastighetsbeteckning: Option<Cow<'a, str>>,
    pub specifikationsnummer: i32,
    pub inkomsttagare: InkomsttagareKU26<'a>,
    pub uppgiftslamnare: UppgiftslamnareKU26<'a>,
}

impl<'a> KU26Type<'a> {
    pub(crate) fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("KU26").write_inner_content(|w| {
            w.write_node_with_code("Inkomstar", "203", &self.inkomstar)?;
            w.write_node_with_code("Borttag", "205", &self.borttag)?;
            w.write_node_with_code("BetaldTomtrattsavgald", "560", &self.betald_tomttsavgald)?;
            w.write_node_with_code("Fastighetsbeteckning", "561", &self.fastighetsbeteckning)?;
            w.write_node_with_code("Specifikationsnummer", "570", self.specifikationsnummer)?;

            self.inkomsttagare.write(w)?;
            self.uppgiftslamnare.write(w)?;
            Ok(())
        })?;
        Ok(())
    }
}

impl<'a> KU26Type<'a> {
    pub(crate) fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, Error> {
        let mut inkomstar = None;
        let mut borttag = None;
        let mut betald_tomttsavgald = None;
        let mut fastighetsbeteckning = None;
        let mut specificationsnummer = None;
        let mut inkomsttagare = None;
        let mut uppgiftslamnare = None;
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Inkomstar" => reader.read_node_into_with_code(element, "203", &mut inkomstar)?,
                    b"Borttag" => reader.read_node_into_with_code(element, "205", &mut borttag)?,
                    b"BetaldTomtrattsavgald" => reader.read_node_into_with_code(element, "560", &mut betald_tomttsavgald)?,
                    b"Fastighetsbeteckning" => reader.read_node_into_with_code(element, "561", &mut fastighetsbeteckning)?,
                    b"Specifikationsnummer" => reader.read_node_into_with_code(element, "570", &mut specificationsnummer)?,
                    b"InkomsttagareKU26" => {
                        inkomsttagare = Some(InkomsttagareKU26::read(reader, &element)?)
                    }
                    b"UppgiftslamnareKU26" => {
                        uppgiftslamnare = Some(UppgiftslamnareKU26::read(reader, &element)?)
                    }
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            inkomstar: inkomstar.ok_or_else(|| MissingElement("Inkomstar".to_string()))?,
                            borttag,
                            betald_tomttsavgald,
                            fastighetsbeteckning,
                            specifikationsnummer: specificationsnummer.ok_or_else(|| MissingElement("Specificationsnummer".to_string()))?,
                            inkomsttagare: inkomsttagare.ok_or_else(|| MissingElement("Inkomsttagare".to_string()))?,
                            uppgiftslamnare: uppgiftslamnare.ok_or_else(|| MissingElement("Uppgiftslamnare".to_string()))?,
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading KU25".to_string())),
                _ => {}
            }
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct InkomsttagareKU26<'a> {
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
}

impl<'a> InkomsttagareKU26<'a> {
    fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("InkomsttagareKU26").write_inner_content(|w| {
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

            Ok(())
        })?;

        Ok(())
    }
}

impl<'a> InkomsttagareKU26<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, Error> {
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
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
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
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
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
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading InkomsttagareKU26".to_string())),
                _ => {}
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UppgiftslamnareKU26<'a> {
    pub uppgiftslamnar_id: Cow<'a, str>,
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

impl<'a> UppgiftslamnareKU26<'a> {
    fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("UppgiftslamnareKU26").write_inner_content(|w| {
            w.write_node_with_code("UppgiftslamnarId", "201", &self.uppgiftslamnar_id)?;
            w.write_node_with_code("NamnUppgiftslamnare", "202", &self.namn_uppgiftslamnare)?;

            Ok(())
        })?;

        Ok(())
    }
}

impl<'a> UppgiftslamnareKU26<'a> {
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
                Event::Eof => return Err(Error::UnexpectedEof("While reading UppgiftslamnareKU26".to_string())),
                _ => {}
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{Arendeinformation, Avsandare, Blankett, Blankettgemensamt, from_str, Kontaktperson, Kontrolluppgift, TekniskKontaktperson, to_string, Uppgiftslamnare};
    use crate::KontrolluppgiftType::{ KU26};
    use crate::ku26::{InkomsttagareKU26, KU26Type, UppgiftslamnareKU26};

    #[test]
    fn ku26_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFT TOMTRÄTTSAVGÄLD (KU26) FÖR KOMMUNER M.FL._2022.xml").unwrap();

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
