pub mod ku20;
pub mod ku10;
pub mod ku25;
pub mod ku28;
pub mod ku21;
pub mod error;
mod ku26;

use std::borrow::Cow;
use std::io::Cursor;
use quick_xml::{NsReader, Writer};
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::name::QName;
use crate::error::Error;
use crate::error::Error::MissingElement;
use crate::KontrolluppgiftType::{KU10, KU20, KU21, KU25, KU26, KU28};
use crate::ku10::{KU10Type};
use crate::ku20::{KU20Type};
use crate::ku25::{KU25Type};
use crate::ku21::{KU21Type};
use crate::ku26::KU26Type;
use crate::ku28::{KU28Type};

#[derive(Debug, PartialEq)]
pub struct Kontrolluppgift<'a> {
    pub avsandare: Avsandare<'a>,
    pub blankettgemensamt: Blankettgemensamt<'a>,
    pub blanketter: Vec<Blankett<'a>>,
}

impl<'a> Kontrolluppgift<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> {
        w.write_event(Event::PI(BytesText::from_escaped("xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"")))?;
        w.create_element("i:Skatteverket")
            .with_attribute(("xmlns:i", "http://xmls.skatteverket.se/se/skatteverket/ai/instans/infoForBeskattning/8.0"))
            .with_attribute(("xmlns", "http://xmls.skatteverket.se/se/skatteverket/ai/komponent/infoForBeskattning/8.0"))
            .with_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"))
            .with_attribute(("omrade", "Kontrolluppgifter"))
            .with_attribute(("xsi:schemaLocation", "http://xmls.skatteverket.se/se/skatteverket/ai/instans/infoForBeskattning/8.0 http://xmls.skatteverket.se/se/skatteverket/ai/kontrolluppgift/instans/Kontrolluppgifter_8.0.xsd"))
            .write_inner_content(|w| {
                self.avsandare.write(w)?;
                self.blankettgemensamt.write(w)?;

                for b in &self.blanketter {
                    b.write(w)?;
                }
                Ok(())
            })?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Blankettgemensamt<'a> {
    pub uppgiftslamnare: Uppgiftslamnare<'a>,
}

impl<'a> Blankettgemensamt<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> {
        w.create_element("Blankettgemensamt").write_inner_content(|w| {
            self.uppgiftslamnare.write(w)?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Uppgiftslamnare<'a> {
    pub uppgiftslamnare_pers_orgnr: Cow<'a, str>,
    pub kontaktperson: Kontaktperson<'a>,
}

impl<'a> Uppgiftslamnare<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> {
        w.create_element("Uppgiftslamnare").write_inner_content(|w| {
            w.write_node("UppgiftslamnarePersOrgnr", &self.uppgiftslamnare_pers_orgnr)?;
            self.kontaktperson.write(w)?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Blankett<'a> {
    pub nummer: i64,
    pub arendeinformation: Arendeinformation<'a>,
    pub blankettinnehall: KontrolluppgiftType<'a>,
}

impl<'a> Blankett<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> {
        w.create_element("Blankett")
            .with_attribute(("nummer", self.nummer.to_string().as_ref()))
            .write_inner_content(|w| {
                self.arendeinformation.write(w)?;
                w.create_element("Blankettinnehall").write_inner_content(|w| {
                    self.blankettinnehall.write(w)?;
                    Ok(())
                })?;
                Ok(())
            })?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum KontrolluppgiftType<'a> {
    KU10(KU10Type<'a>),
    KU20(KU20Type<'a>),
    KU21(KU21Type<'a>),
    KU25(KU25Type<'a>),
    KU26(KU26Type<'a>),
    KU28(KU28Type<'a>),
}

impl<'a> KontrolluppgiftType<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> {
        match self {
            KU10(v) => {
                v.write(w)?;
            }
            KU20(v) => {
                v.write(w)?;
            }
            KU21(v) => {
                v.write(w)?;
            }
            KU25(v) => {
                v.write(w)?;
            }
            KU26(v) => {
                v.write(w)?;
            }
            KU28(v) => {
                v.write(w)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Arendeinformation<'a> {
    pub arendeagare: Cow<'a, str>,
    pub period: Cow<'a, str>,
    pub arendenummer: Option<Cow<'a, str>>,
}

impl<'a> Arendeinformation<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> {
        w.create_element("Arendeinformation").write_inner_content(|w| {
            w.write_node("Arendeagare", &self.arendeagare)?;
            w.write_node("Period", &self.period)?;
            w.write_node("Arendenummer", &self.arendenummer)?;

            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Kontaktperson<'a> {
    pub namn: Cow<'a, str>,
    pub telefon: Cow<'a, str>,
    pub epostadress: Cow<'a, str>,
    pub sakomrade: Option<Cow<'a, str>>,
}

impl<'a> Kontaktperson<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> {
        w.create_element("Kontaktperson").write_inner_content(|w| {
            w.write_node("Namn", &self.namn)?;
            w.write_node("Telefon", &self.telefon)?;
            w.write_node("Epostadress", &self.epostadress)?;
            w.write_node("Sakomrade", &self.sakomrade)?;

            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Avsandare<'a> {
    pub programnamn: Cow<'a, str>,
    pub organisationsnummer: Cow<'a, str>,
    pub teknisk_kontaktperson: TekniskKontaktperson<'a>,
    pub skapad: Cow<'a, str>,
}

impl<'a> Avsandare<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> {
        w.create_element("Avsandare").write_inner_content(|w| {
            w.write_node("Programnamn", &self.programnamn)?;
            w.write_node("Organisationsnummer", &self.organisationsnummer)?;
            self.teknisk_kontaktperson.write(w)?;
            w.write_node("Skapad", &self.skapad)?;
            Ok(())
        })?;


        Ok(())
    }
}


#[derive(Debug, PartialEq, Default)]
pub struct TekniskKontaktperson<'a> {
    pub namn: Cow<'a, str>,
    pub telefon: Cow<'a, str>,
    pub epostadress: Cow<'a, str>,
    pub utdelningsadress1: Option<Cow<'a, str>>,
    pub utdelningsadress2: Option<Cow<'a, str>>,
    pub postnummer: Option<Cow<'a, str>>,
    pub postort: Option<Cow<'a, str>>,
}

impl<'a> TekniskKontaktperson<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> {
        w.create_element("TekniskKontaktperson").write_inner_content(|w| {
            w.write_node("Namn", &self.namn)?;
            w.write_node("Telefon", &self.telefon)?;
            w.write_node("Epostadress", &self.epostadress)?;
            w.write_node("Utdelningsadress1", &self.utdelningsadress1)?;
            w.write_node("Utdelningsadress2", &self.utdelningsadress2)?;
            w.write_node("Postnummer", &self.postnummer)?;
            w.write_node("Postort", &self.postort)?;
            Ok(())
        })?;

        Ok(())
    }
}

/// Deserialize xml into rust types
/// It also does not currently validate any namespace information.
pub fn from_str(str: &str) -> Result<Kontrolluppgift, Error> {
    let mut reader = NsReader::from_str(str);

    let mut g_avsandare = None;
    let mut blankettgemensamt = None;
    let mut blanketter = vec![];

    reader.expand_empty_elements(true);
    loop {
        let event = reader.read_event()?;
        match event {
            Event::Start(element) => match element.local_name().as_ref() {
                b"Skatteverket" => {}
                b"Avsandare" => {
                    g_avsandare = Some(Avsandare::read(&mut reader, element.name())?)
                }
                b"Blankettgemensamt" => {
                    blankettgemensamt = Some(Blankettgemensamt::read(&mut reader, element.name())?)
                }
                b"Blankett" => {
                    blanketter.push(Blankett::read(&mut reader, &element)?)
                }
                _ => unexpected_element(&element)?
            },
            Event::End(element) => {
                if element.local_name().as_ref() == b"Skatteverket" {
                    return Ok(Kontrolluppgift {
                        avsandare: g_avsandare.ok_or_else(|| MissingElement("Avsandare".to_string()))?,
                        blankettgemensamt: blankettgemensamt.ok_or_else(|| MissingElement("Blankettgemensamt".to_string()))?,
                        blanketter,
                    });
                }
            }
            Event::Eof => return Err(Error::UnexpectedEof("While reading Skatteverket".to_string())),
            _ => (),             // There are `Event` types not considered here
        }
    }
}


/// Turns a Kontrolluppgift into an owned string. Provides "faltkod" with the const values in the specification
pub fn to_string(kontrolluppgift: &Kontrolluppgift) -> Result<String, Error> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    kontrolluppgift.write(&mut writer)?;

    let res = String::from_utf8(writer.into_inner().into_inner())
        .expect("We just created this, so it should only be valid utf8");
    Ok(res)
}

impl<'a> Blankett<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, Error> {
        let mut nummer = None;
        let mut arendeinformation = None;
        let mut blankettinnehall = None;
        for attr_result in tag.attributes() {
            let a = attr_result?;

            if let b"nummer" = a.key.as_ref() {
                nummer = Some(a.decode_and_unescape_value(reader)?
                    .parse::<i64>().map_err(|_| MissingElement("nummer".to_string()))?)
            }
        }
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Arendeinformation" => {
                        arendeinformation = Some(Arendeinformation::read(reader, &element)?)
                    }
                    b"Blankettinnehall" => {
                        loop {
                            match reader.read_event()? {
                                Event::Start(element) => match element.local_name().as_ref() {
                                    b"KU10" => {
                                        blankettinnehall = Some(KU10(KU10Type::read(reader, &element)?));
                                        break;
                                    }
                                    b"KU21" => {
                                        blankettinnehall = Some(KU21(KU21Type::read(reader, &element)?));
                                        break;
                                    }
                                    b"KU20" => {
                                        blankettinnehall = Some(KU20(KU20Type::read(reader, &element)?));
                                        break;
                                    }
                                    b"KU25" => {
                                        blankettinnehall = Some(KU25(KU25Type::read(reader, &element)?));
                                        break;
                                    }
                                    b"KU26" => {
                                        blankettinnehall = Some(KU26(KU26Type::read(reader, &element)?));
                                        break;
                                    }
                                    b"KU28" => {
                                        blankettinnehall = Some(KU28(KU28Type::read(reader, &element)?));
                                        break;
                                    }
                                    &_ => unexpected_element(&element)?
                                },
                                Event::End(_) => break,
                                _ => {}
                            }
                        }
                    }
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            nummer: nummer.ok_or_else(|| MissingElement("nummer".to_string()))?,
                            arendeinformation: arendeinformation.ok_or_else(|| MissingElement("Arendeinformation".to_string()))?,
                            blankettinnehall: blankettinnehall.ok_or_else(|| MissingElement("Blankettinnehall".to_string()))?,
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading Blankett".to_string())),
                _ => {}
            }
        }
    }
}

impl<'a> Arendeinformation<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, Error> {
        let mut arendeagare = None;
        let mut period = None;
        let mut arendenummer = None;
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Arendeagare" => reader.read_node_into(element, &mut arendeagare)?,
                    b"Period" => reader.read_node_into(element, &mut period)?,
                    b"Arendenummer" => reader.read_node_into(element, &mut arendenummer)?,
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            arendeagare: arendeagare.ok_or_else(|| MissingElement("Arendeagare".to_string()))?,
                            period: period.ok_or_else(|| MissingElement("Period".to_string()))?,
                            arendenummer,
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading Arendeinformation".to_string())),
                _ => {}
            }
        }
    }
}

impl<'a> Avsandare<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: QName) -> Result<Self, Error> {
        let mut programnamn = None;
        let mut organisationsnummer = None;
        let mut skapad = None;
        let mut tekninsk_kontaktperson = None;
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Programnamn" => reader.read_node_into(element, &mut programnamn)?,
                    b"Organisationsnummer" => reader.read_node_into(element, &mut organisationsnummer)?,
                    b"Skapad" => reader.read_node_into(element, &mut skapad)?,
                    b"TekniskKontaktperson" => tekninsk_kontaktperson = Some(TekniskKontaktperson::read(reader, element.name())?),
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag {
                        return Ok(Avsandare {
                            programnamn: programnamn.ok_or_else(|| MissingElement("Programnamn".to_string()))?,
                            organisationsnummer: organisationsnummer.ok_or_else(|| MissingElement("Organisationsnummer".to_string()))?,
                            teknisk_kontaktperson: tekninsk_kontaktperson.ok_or_else(|| MissingElement("TekninskKontaktperson".to_string()))?,
                            skapad: skapad.ok_or_else(|| MissingElement("Skapad".to_string()))?,
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading Avsandare".to_string())),
                _ => {}
            }
        }
    }
}

impl<'a> Blankettgemensamt<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: QName) -> Result<Self, Error> {
        let mut uppgiftslamnare = None;
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Uppgiftslamnare" => {
                        uppgiftslamnare = Some(Uppgiftslamnare::read(reader, element.name())?)
                    }
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag {
                        return Ok(Self {
                            uppgiftslamnare: uppgiftslamnare.ok_or_else(|| MissingElement("Uppgiftslamnare".to_string()))?,
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading Blankettgemensamt".to_string())),
                _ => {}
            }
        }
    }
}

impl<'a> Uppgiftslamnare<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: QName) -> Result<Self, Error> {
        let mut uppgiftslamnare_pers_orgnr = None;
        let mut kontaktperson = None;
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"UppgiftslamnarePersOrgnr" => reader.read_node_into(element, &mut uppgiftslamnare_pers_orgnr)?,
                    b"Kontaktperson" => {
                        kontaktperson = Some(Kontaktperson::read(reader, element.name())?)
                    }
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag {
                        return Ok(Self {
                            uppgiftslamnare_pers_orgnr: uppgiftslamnare_pers_orgnr.ok_or_else(|| MissingElement("UppgiftslamnarePersOrgnr".to_string()))?,
                            kontaktperson: kontaktperson.ok_or_else(|| MissingElement("Kontaktperson".to_string()))?,
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading Uppgiftslamnare".to_string())),
                _ => {}
            }
        }
    }
}

impl<'a> Kontaktperson<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: QName) -> Result<Self, Error> {
        let mut namn = None;
        let mut telefon = None;
        let mut epostadress = None;
        let mut sakomrade = None;
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Namn" => reader.read_node_into(element, &mut namn)?,
                    b"Telefon" => reader.read_node_into(element, &mut telefon)?,
                    b"Epostadress" => reader.read_node_into(element, &mut epostadress)?,
                    b"Sakomrade" => reader.read_node_into(element, &mut sakomrade)?,
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag {
                        return Ok(Self {
                            namn: namn.ok_or_else(|| MissingElement("Namn".to_string()))?,
                            telefon: telefon.ok_or_else(|| MissingElement("Telefon".to_string()))?,
                            epostadress: epostadress.ok_or_else(|| MissingElement("Epostadress".to_string()))?,
                            sakomrade,
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading Kontaktperson".to_string())),
                _ => {}
            }
        }
    }
}

impl<'a> TekniskKontaktperson<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: QName) -> Result<Self, Error> {
        let mut namn = None;
        let mut telefon = None;
        let mut epostadress = None;
        let mut utdelningsadress1 = None;
        let mut utdelningsadress2 = None;
        let mut postnummer = None;
        let mut postort = None;
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Namn" => reader.read_node_into(element, &mut namn)?,
                    b"Telefon" => reader.read_node_into(element, &mut telefon)?,
                    b"Epostadress" => reader.read_node_into(element, &mut epostadress)?,
                    b"Utdelningsadress1" => reader.read_node_into(element, &mut utdelningsadress1)?,
                    b"Utdelningsadress2" => reader.read_node_into(element, &mut utdelningsadress2)?,
                    b"Postnummer" => reader.read_node_into(element, &mut postnummer)?,
                    b"Postort" => reader.read_node_into(element, &mut postort)?,
                    &_ => unexpected_element(&element)?
                }
                Event::End(element) => {
                    if element.name() == tag {
                        return Ok(Self {
                            namn: namn.ok_or_else(|| MissingElement("Namn".to_string()))?,
                            telefon: telefon.ok_or_else(|| MissingElement("Telefon".to_string()))?,
                            epostadress: epostadress.ok_or_else(|| MissingElement("Epostadress".to_string()))?,
                            utdelningsadress1,
                            utdelningsadress2,
                            postnummer,
                            postort,
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading TekniskKontaktperson".to_string())),
                _ => {}
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum DeError {
    ReadError,
    Read(String),
    Custom(String),
    UnexpectedElement(String),
    MissingField(String),
    MissingAttribute((String, String)),
    UnexpectedToken(String),
    UnexpectedXml(String),
}

fn unexpected_element<E>(element: &BytesStart) -> Result<E, Error> {
    Err(Error::UnexpectedToken(std::str::from_utf8(element.name().as_ref()).map_err(|e| Error::NonDecodable(Some(e)))?.to_string()))
}

trait Write<'a, T> where T: Writable {
    fn write_node(&mut self, name: &str, data: T) -> Result<(), quick_xml::Error>;
    fn write_node_with_code(&mut self, name: &str, code: &str, data: T) -> Result<(), quick_xml::Error>;
}

trait Reader<'a, 'b, T> where T: Readable<'a, 'b> {
    fn read_node_into(&mut self, element: BytesStart, x: &mut Option<T>) -> Result<(), Error>;
    fn read_node_into_with_code(&mut self, element: BytesStart, code: &str, x: &mut Option<T>) -> Result<(), Error>;
}

impl<'a, 'b : 'a, T: Readable<'a, 'b> + 'b> Reader<'a, 'b, T> for NsReader<&'b [u8]> {
    fn read_node_into(&mut self, element: BytesStart, x: &mut Option<T>) -> Result<(), Error> {
        *x = Some(T::get_str(self.read_text(element.name())?)?);
        Ok(())
    }

    fn read_node_into_with_code(&mut self, element: BytesStart, code: &str, x: &mut Option<T>) -> Result<(), Error> {
        let kod = element.try_get_attribute("faltkod")?.ok_or_else(|| MissingElement("faltkod".to_string()))?;
        let kod = kod.decode_and_unescape_value(self)?;
        if code != kod {
            return Err(Error::UnexpectedToken(format!("Unexpected faltkod on {}, expected: {}, got: {}", std::str::from_utf8(element.name().as_ref()).unwrap(), code, kod)));
        }
        *x = Some(T::get_str(self.read_text(element.name())?)?);
        Ok(())
    }
}

impl<'a, T: Writable, W: std::io::Write> Write<'a, T> for Writer<W> {
    fn write_node(&mut self, name: &str, data: T) -> Result<(), quick_xml::Error> {
        let str = data.get_str();
        match str {
            None => {
                Ok(())
            }
            Some(data) => {
                self.create_element(name).write_text_content(BytesText::new(&data))?;
                Ok(())
            }
        }
    }
    fn write_node_with_code(&mut self, name: &str, code: &str, data: T) -> Result<(), quick_xml::Error> {
        let str = data.get_str();
        match str {
            None => {
                Ok(())
            }
            Some(data) => {
                self.create_element(name)
                    .with_attribute(("faltkod", code))
                    .write_text_content(BytesText::new(&data))?;
                Ok(())
            }
        }
    }
}


pub(crate) trait Writable {
    fn get_str(&self) -> Option<String>;
}


pub(crate) trait Readable<'a, 'b> {
    fn get_str(data: Cow<'a, str>) -> Result<Self, Error> where Self: Sized + 'b;
}

impl<'a, 'b> Readable<'a, 'b> for bool {
    fn get_str(data: Cow<str>) -> Result<Self, Error> {
        match data.as_ref() {
            "0" => Ok(false),
            "1" => Ok(true),
            &_ => Err(Error::UnexpectedToken("expected KryssTyp, found: ".to_string() + &data))
        }
    }
}

impl<'a, 'b> Readable<'a, 'b> for i32 {
    fn get_str(data: Cow<str>) -> Result<Self, Error> {
        data.as_ref().parse().map_err(|_| Error::UnexpectedToken("expected number got: ".to_string() + data.as_ref()))
    }
}

impl<'a, 'b> Readable<'a, 'b> for f32 {
    fn get_str(data: Cow<str>) -> Result<Self, Error> {
        data.as_ref().parse().map_err(|_| Error::UnexpectedToken("expected fraction got: ".to_string() + data.as_ref()))
    }
}

impl<'a, 'b : 'a> Readable<'a, 'b> for Cow<'a, str> {
    fn get_str(data: Cow<'b, str>) -> Result<Self, Error> {
        Ok(data)
    }
}

impl<T> Writable for &Option<T> where T: Writable {
    fn get_str(&self) -> Option<String> {
        match self {
            None => None,
            Some(v) => v.get_str()
        }
    }
}

impl<'a> Writable for Cow<'a, str> {
    fn get_str(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Writable for i32 {
    fn get_str(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Writable for f32 {
    fn get_str(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Writable for &i32 {
    fn get_str(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Writable for bool {
    fn get_str(&self) -> Option<String> {
        Some(match *self {
            true => "1".to_string(),
            false => "0".to_string()
        })
    }
}

impl<'a> Writable for &Cow<'a, str> {
    fn get_str(&self) -> Option<String> {
        Some(self.to_string())
    }
}
