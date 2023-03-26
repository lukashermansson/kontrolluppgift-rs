use std::borrow::Cow;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{NsReader, Writer};
use crate::error::Error;
use crate::{Reader, unexpected_element, Write};
use crate::error::Error::MissingElement;

/// Kontrolluppgift 20
#[derive(Debug, PartialEq)]
pub struct KU20Type<'a> {
    pub avdragen_skatt: Option<i32>,
    pub delagare: Option<bool>,
    pub inkomstar: Cow<'a, str>,
    pub borttag: Option<bool>,
    pub ranteinkomst: Option<i32>,
    pub forfogarkonto: Option<bool>,
    pub ranteinkomst_ej_konto: Option<i32>,
    pub annan_inkomst: Option<i32>,
    pub specifikationsnummer: i32,
    pub inkomsttagare: InkomsttagareKU20<'a>,
    pub uppgiftslamnare: UppgiftslamnareKU20<'a>,

}

impl<'a> KU20Type<'a> {
    pub(crate) fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("KU20").write_inner_content(|w| {
            w.write_node_with_code("AvdragenSkatt", "001", &self.avdragen_skatt)?;
            w.write_node_with_code("Delagare", "061", &self.delagare)?;
            w.write_node_with_code("Inkomstar", "203", &self.inkomstar)?;
            w.write_node_with_code("Borttag", "205", &self.borttag)?;
            w.write_node_with_code("Ranteinkomst", "500", &self.ranteinkomst)?;
            w.write_node_with_code("Forfogarkonto", "502", &self.forfogarkonto)?;
            w.write_node_with_code("RanteinkomstEjKonto", "503", &self.ranteinkomst_ej_konto)?;
            w.write_node_with_code("AnnanInkomst", "504", &self.annan_inkomst)?;
            w.write_node_with_code("Specifikationsnummer", "570", self.specifikationsnummer)?;

            self.inkomsttagare.write(w)?;
            self.uppgiftslamnare.write(w)?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct InkomsttagareKU20<'a> {
    pub landskod_tin: Option<Cow<'a, str>>,
    pub fodelseort: Option<Cow<'a, str>>,
    pub landskod_fodelseort: Option<Cow<'a, str>>,
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

impl<'a> InkomsttagareKU20<'a> {
    fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("InkomsttagareKU20").write_inner_content(|w| {
            w.write_node_with_code("LandskodTIN", "076", &self.landskod_tin)?;
            w.write_node_with_code("Fodelseort", "077", &self.fodelseort)?;
            w.write_node_with_code("LandskodFodelseort", "078", &self.landskod_fodelseort)?;
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

#[derive(Debug, PartialEq)]
pub struct UppgiftslamnareKU20<'a> {
    pub uppgiftslamnar_id: Cow<'a, str>,
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

impl<'a> UppgiftslamnareKU20<'a> {
    fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("UppgiftslamnareKU20").write_inner_content(|w| {
            w.write_node_with_code("UppgiftslamnarId", "201", &self.uppgiftslamnar_id)?;
            w.write_node_with_code("NamnUppgiftslamnare", "202", &self.namn_uppgiftslamnare)?;

            Ok(())
        })?;

        Ok(())
    }
}

impl<'a> KU20Type<'a> {
    pub(crate) fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, Error> {
        let mut avdragen_skatt = None;
        let mut delagare = None;
        let mut inkomstar = None;
        let mut borttag = None;
        let mut ranteinkomst = None;
        let mut forfogarkonto = None;
        let mut ranteinkomst_ej_konto = None;
        let mut annan_inkomst = None;
        let mut specificationsnummer = None;
        let mut inkomsttagare = None;
        let mut uppgiftslamnare = None;
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"AvdragenSkatt" => reader.read_node_into(element, &mut avdragen_skatt)?,
                    b"Delagare" => reader.read_node_into(element, &mut delagare)?,
                    b"Inkomstar" => reader.read_node_into(element, &mut inkomstar)?,
                    b"Borttag" => reader.read_node_into(element, &mut borttag)?,
                    b"Ranteinkomst" => reader.read_node_into(element, &mut ranteinkomst)?,
                    b"Forfogarkonto" => reader.read_node_into(element, &mut forfogarkonto)?,
                    b"RanteinkomstEjKonto" => reader.read_node_into(element, &mut ranteinkomst_ej_konto)?,
                    b"AnnanInkomst" =>reader.read_node_into(element, &mut annan_inkomst)?,
                    b"Specifikationsnummer" =>reader.read_node_into(element, &mut specificationsnummer)?,
                    b"InkomsttagareKU20" => {
                        inkomsttagare = Some(InkomsttagareKU20::read(reader, &element)?)
                    }
                    b"UppgiftslamnareKU20" => {
                        uppgiftslamnare = Some(UppgiftslamnareKU20::read(reader, &element)?)
                    }
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            avdragen_skatt,
                            delagare,
                            inkomstar: inkomstar.ok_or_else(|| MissingElement("Inkomstar".to_string()))?,
                            borttag,
                            ranteinkomst,
                            forfogarkonto,
                            ranteinkomst_ej_konto,
                            annan_inkomst,
                            specifikationsnummer: specificationsnummer.ok_or_else(|| MissingElement("Specifikationsnummer".to_string()))?,
                            inkomsttagare: inkomsttagare.ok_or_else(|| MissingElement("Inkomsttagare".to_string()))?,
                            uppgiftslamnare: uppgiftslamnare.ok_or_else(|| MissingElement("Uppgiftslamnare".to_string()))?,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

impl<'a> InkomsttagareKU20<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, Error> {
        let mut landskod_tin = None;
        let mut fodelseort = None;
        let mut landskod_fodelseort = None;
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
                    b"LandskodTIN" => reader.read_node_into(element, &mut landskod_tin)?,
                    b"Fodelseort" =>reader.read_node_into(element, &mut fodelseort)?,
                    b"LandskodFodelseort" =>reader.read_node_into(element, &mut landskod_fodelseort)?,
                    b"Inkomsttagare" =>reader.read_node_into(element, &mut inkomsttagare)?,
                    b"Fornamn" =>reader.read_node_into(element, &mut fornamn)?,
                    b"Efternamn" =>reader.read_node_into(element, &mut efternamn)?,
                    b"Gatuadress" =>reader.read_node_into(element, &mut gatuadress)?,
                    b"Postnummer" =>reader.read_node_into(element, &mut postnummer)?,
                    b"Postort" =>reader.read_node_into(element, &mut postort)?,
                    b"LandskodPostort" =>reader.read_node_into(element, &mut landskod_postort)?,
                    b"Fodelsetid" =>reader.read_node_into(element, &mut fodelsetid)?,
                    b"AnnatIDNr" =>reader.read_node_into(element, &mut annat_id_nr)?,
                    b"OrgNamn" =>reader.read_node_into(element, &mut org_namn)?,
                    b"Gatuadress2" =>reader.read_node_into(element, &mut gatuadress2)?,
                    b"FriAdress" =>reader.read_node_into(element, &mut fri_adress)?,
                    b"TIN" =>reader.read_node_into(element, &mut tin)?,
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            landskod_tin,
                            fodelseort,
                            landskod_fodelseort,
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

impl<'a> UppgiftslamnareKU20<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, Error> {
        let mut uppgiftslamnar_id = None;
        let mut namn_uppgiftslamnare = None;
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"UppgiftslamnarId" => reader.read_node_into(element, &mut uppgiftslamnar_id)?,
                    b"NamnUppgiftslamnare" => reader.read_node_into(element, &mut namn_uppgiftslamnare)?,
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
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{Arendeinformation, Avsandare, Blankett, Blankettgemensamt, from_str, Kontaktperson, Kontrolluppgift, TekniskKontaktperson, to_string, Uppgiftslamnare};
    use crate::KontrolluppgiftType::KU20;
    use crate::ku20::{InkomsttagareKU20, KU20Type, UppgiftslamnareKU20};

    #[test]
    fn ku20_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFTER RÄNTA, UTDELNING M.M. KU20 FÖR_2022.xml").unwrap();

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
