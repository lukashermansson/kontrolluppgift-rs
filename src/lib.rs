pub mod ku20;
pub mod ku10;
pub mod ku25;
pub mod ku28;
pub mod ku21;

use std::borrow::Cow;
use std::io::Cursor;
use quick_xml::{Error, NsReader, Writer};
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::name::QName;
use crate::DeError::{MissingAttribute, MissingField, Read, ReadError, UnexpectedElement};
use crate::KontrolluppgiftType::{KU10, KU20, KU21, KU25, KU28};
use crate::ku10::{KU10Type};
use crate::ku20::{KU20Type};
use crate::ku25::{KU25Type};
use crate::ku21::{KU21Type};
use crate::ku28::{KU28Type};

#[derive(Debug, PartialEq)]
pub struct Kontrolluppgift<'a> {
    pub avsandare: Avsandare<'a>,
    pub blankettgemensamt: Blankettgemensamt<'a>,
    pub blanketter: Vec<Blankett<'a>>,
}

impl<'a> Kontrolluppgift<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), Error> {
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
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), Error> {
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
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), Error> {
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
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), Error> {
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
    KU28(KU28Type<'a>),
}

impl<'a> KontrolluppgiftType<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), Error> {
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
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), Error> {
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
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), Error> {
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
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), Error> {
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
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), Error> {
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
/// Does not validate data contained according to specification
/// It also does not currently validate any namespace information or "faltkod" ( this may be subject to change before 1.0)
pub fn from_str(str: &str) -> Result<Kontrolluppgift, DeError> {
    let mut reader = NsReader::from_str(str);

    let mut g_avsandare = None;
    let mut blankettgemensamt = None;
    let mut blanketter = vec![];

    loop {
        let event = reader.read_event().map_err(|_| ReadError)?;
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
                _ => {}
            },

            Event::Eof => break, // exits the loop when reaching end of file
            _ => (),             // There are `Event` types not considered here
        }
    }
    return Ok(Kontrolluppgift {
        avsandare: g_avsandare.ok_or_else(|| MissingField("Avsandare".to_string()))?,
        blankettgemensamt: blankettgemensamt.ok_or_else(|| MissingField("Blankettgemensamt".to_string()))?,
        blanketter,
    });
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
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {
        let mut nummer = None;
        let mut arendeinformation = None;
        let mut blankettinnehall = None;
        for attr_result in tag.attributes() {
            let a = attr_result.unwrap();
            match a.key.as_ref() {
                b"nummer" => nummer = Some(a.decode_and_unescape_value(reader).map_err(|_| ReadError)?
                    .parse::<i64>().map_err(|_| Read("nummer is not a number".to_string()))?),
                _ => (),
            }
        }
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Arendeinformation" => {
                        arendeinformation = Some(Arendeinformation::read(reader, &element)?)
                    }
                    b"Blankettinnehall" => {
                        loop {
                            match reader.read_event().unwrap() {
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
                                    b"KU28" => {
                                        blankettinnehall = Some(KU28(KU28Type::read(reader, &element)?));
                                        break;
                                    }
                                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                                },
                                Event::End(_) => break,
                                _ => {}
                            }
                        }
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            nummer: nummer.ok_or_else(|| MissingAttribute(("Blankett".to_string(), "nummer".to_string())))?,
                            arendeinformation: arendeinformation.ok_or_else(|| MissingField("Arendeinformation".to_string()))?,
                            blankettinnehall: blankettinnehall.ok_or_else(|| MissingField("Blankettinnehall".to_string()))?,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

impl<'a> Arendeinformation<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {
        let mut arendeagare = None;
        let mut period = None;
        let mut arendenummer = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Arendeagare" => {
                        arendeagare = Some(reader.read_text(element.name()).map_err(|_| Read(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))?);
                    }
                    b"Period" => {
                        period = Some(reader.read_text(element.name()).map_err(|_| Read(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))?);
                    }
                    b"Arendenummer" => {
                        arendenummer = Some(reader.read_text(element.name()).map_err(|_| Read(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))?);
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            arendeagare: arendeagare.ok_or_else(|| MissingField("Arendeagare".to_string()))?,
                            period: period.ok_or_else(|| MissingField("Period".to_string()))?,
                            arendenummer,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

impl<'a> Avsandare<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: QName) -> Result<Self, DeError> {
        let mut programnamn = None;
        let mut organisationsnummer = None;
        let mut skapad = None;
        let mut tekninsk_kontaktperson = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Programnamn" => {
                        programnamn = Some(reader.read_text(element.name()).map_err(|_| Read(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))?);
                    }
                    b"Organisationsnummer" => {
                        organisationsnummer = Some(reader.read_text(element.name()).map_err(|_| Read(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))?);
                    }
                    b"Skapad" => {
                        skapad = Some(reader.read_text(element.name()).map_err(|_| Read(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))?);
                    }
                    b"TekniskKontaktperson" => {
                        tekninsk_kontaktperson = Some(TekniskKontaktperson::read(reader, element.name())?)
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag {
                        return Ok(Avsandare {
                            programnamn: programnamn.ok_or_else(|| MissingField("Programnamn".to_string()))?,
                            organisationsnummer: organisationsnummer.ok_or_else(|| MissingField("Organisationsnummer".to_string()))?,
                            teknisk_kontaktperson: tekninsk_kontaktperson.ok_or_else(|| MissingField("TekniskKontaktperson".to_string()))?,
                            skapad: skapad.ok_or_else(|| MissingField("Skapad".to_string()))?,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

impl<'a> Blankettgemensamt<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: QName) -> Result<Self, DeError> {
        let mut uppgiftslamnare = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Uppgiftslamnare" => {
                        uppgiftslamnare = Some(Uppgiftslamnare::read(reader, element.name())?)
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag {
                        return Ok(Self {
                            uppgiftslamnare: uppgiftslamnare.ok_or_else(|| MissingField("Uppgiftslamnare".to_string()))?
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

impl<'a> Uppgiftslamnare<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: QName) -> Result<Self, DeError> {
        let mut uppgiftslamnare_pers_orgnr = None;
        let mut kontaktperson = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"UppgiftslamnarePersOrgnr" => {
                        uppgiftslamnare_pers_orgnr = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Kontaktperson" => {
                        kontaktperson = Some(Kontaktperson::read(reader, element.name())?)
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag {
                        return Ok(Self {
                            uppgiftslamnare_pers_orgnr: uppgiftslamnare_pers_orgnr.ok_or_else(|| MissingField("UppgiftslamnarePersOrgnr".to_string()))?,
                            kontaktperson: kontaktperson.ok_or_else(|| MissingField("Kontaktperson".to_string()))?,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

impl<'a> Kontaktperson<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: QName) -> Result<Self, DeError> {
        let mut namn = None;
        let mut telefon = None;
        let mut epostadress = None;
        let mut sakomrade = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Namn" => {
                        namn = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Telefon" => {
                        telefon = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Epostadress" => {
                        epostadress = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Sakomrade" => {
                        sakomrade = Some(reader.read_text(element.name()).unwrap());
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag {
                        return Ok(Self {
                            namn: namn.ok_or_else(|| MissingField("Namn".to_string()))?,
                            telefon: telefon.ok_or_else(|| MissingField("Telefon".to_string()))?,
                            epostadress: epostadress.ok_or_else(|| MissingField("Epostadress".to_string()))?,
                            sakomrade,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

impl<'a> TekniskKontaktperson<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: QName) -> Result<Self, DeError> {
        let mut namn = None;
        let mut telefon = None;
        let mut epostadress = None;
        let mut utdelningsadress1 = None;
        let mut utdelningsadress2 = None;
        let mut postnummer = None;
        let mut postort = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Namn" => {
                        namn = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Telefon" => {
                        telefon = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Epostadress" => {
                        epostadress = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Utdelningsadress1" => {
                        utdelningsadress1 = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Utdelningsadress2" => {
                        utdelningsadress2 = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Postnummer" => {
                        postnummer = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Postort" => {
                        postort = Some(reader.read_text(element.name()).unwrap());
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag {
                        return Ok(Self {
                            namn: namn.unwrap(),
                            telefon: telefon.unwrap(),
                            epostadress: epostadress.unwrap(),
                            utdelningsadress1,
                            utdelningsadress2,
                            postnummer,
                            postort,
                        });
                    }
                }
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


pub(crate) fn to_bool(input: Cow<str>) -> Option<bool> {
    return match input.as_ref() {
        "1" => Some(true),
        "0" => Some(false),
        _ => None
    };
}

trait Write<'a, T> where T: Writable {
    fn write_node(&mut self, name: &str, data: T) -> Result<(), Error>;
    fn write_node_with_code(&mut self, name: &str, code: &str, data: T) -> Result<(), Error>;
}

impl<'a, T: Writable, W: std::io::Write> Write<'a, T> for Writer<W> {
    fn write_node(&mut self, name: &str, data: T) -> Result<(), Error> {
        let str = data.get_str();
        match str {
            None => {
                Ok(())
            }
            Some(data) => {
                self.create_element(name).write_text_content(BytesText::new(&*data))?;
                Ok(())
            }
        }
    }
    fn write_node_with_code(&mut self, name: &str, code: &str, data: T) -> Result<(), Error> {
        let str = data.get_str();
        match str {
            None => {
                Ok(())
            }
            Some(data) => {
                self.create_element(name)
                    .with_attribute(("faltkod", code))
                    .write_text_content(BytesText::new(&*data))?;
                Ok(())
            }
        }
    }
}


pub(crate) trait Writable {
    fn get_str(&self) -> Option<String>;
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

impl<'a> Writable for i32 {
    fn get_str(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl<'a> Writable for f32 {
    fn get_str(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl<'a> Writable for &i32 {
    fn get_str(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl<'a> Writable for bool {
    fn get_str(&self) -> Option<String> {
        Some(match self {
            &true => "1".to_string(),
            &false => "0".to_string()
        })
    }
}

impl<'a> Writable for &Cow<'a, str> {
    fn get_str(&self) -> Option<String> {
        Some(self.to_string())
    }
}
