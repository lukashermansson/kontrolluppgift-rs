pub mod error;
pub mod ku10;
pub mod ku13;
pub mod ku14;
pub mod ku16;
pub mod ku17;
pub mod ku18;
pub mod ku19;
pub mod ku20;
pub mod ku21;
pub mod ku25;
pub mod ku26;
pub mod ku28;
pub mod ku30;

use crate::error::Error;
use crate::error::Error::{MissingElement, NonDecodable};
use crate::ku10::KU10Type;
use crate::ku13::KU13Type;
use crate::ku14::KU14Type;
use crate::ku16::KU16Type;
use crate::ku17::KU17Type;
use crate::ku18::KU18Type;
use crate::ku19::KU19Type;
use crate::ku20::KU20Type;
use crate::ku21::KU21Type;
use crate::ku25::KU25Type;
use crate::ku26::KU26Type;
use crate::ku28::KU28Type;
use crate::ku30::KU30Type;
use crate::KontrolluppgiftType::*;
use kontrolluppgift_macros::{
    KUStringEnum, KUVariantsEnum, KontrolluppgiftRead, KontrolluppgiftWrite,
};
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::{NsReader, Writer};
use regex::Regex;
use std::borrow::Cow;
use std::io::Cursor;

#[derive(Debug, PartialEq)]
pub struct Kontrolluppgift<'a> {
    pub avsandare: Avsandare<'a>,
    pub blankettgemensamt: Blankettgemensamt<'a>,
    pub blanketter: Vec<Blankett<'a>>,
}

impl<'a> Kontrolluppgift<'a> {
    fn write<W: std::io::Write>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> {
        w.write_event(Event::PI(BytesText::from_escaped(
            "xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"",
        )))?;
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

#[derive(Debug, Default, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("Blankettgemensamt"))]
pub struct Blankettgemensamt<'a> {
    #[ku(name(b"Uppgiftslamnare"), inner_ty(true), required(true))]
    pub uppgiftslamnare: Uppgiftslamnare<'a>,
}

#[derive(Debug, Default, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("Uppgiftslamnare"))]
pub struct Uppgiftslamnare<'a> {
    #[ku(name(b"UppgiftslamnarePersOrgnr"), required(true))]
    pub uppgiftslamnare_pers_orgnr: Cow<'a, str>,
    #[ku(name(b"Kontaktperson"), required(true), inner_ty(true))]
    pub kontaktperson: Kontaktperson<'a>,
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
                w.create_element("Blankettinnehall")
                    .write_inner_content(|w| {
                        self.blankettinnehall.write(w)?;
                        Ok(())
                    })?;
                Ok(())
            })?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, KUVariantsEnum)]
pub enum KontrolluppgiftType<'a> {
    KU10(KU10Type<'a>),
    KU13(KU13Type<'a>),
    KU14(KU14Type<'a>),
    KU16(KU16Type<'a>),
    KU17(KU17Type<'a>),
    KU18(KU18Type<'a>),
    KU19(KU19Type<'a>),
    KU20(KU20Type<'a>),
    KU21(KU21Type<'a>),
    KU25(KU25Type<'a>),
    KU26(KU26Type<'a>),
    KU28(KU28Type<'a>),
    KU30(KU30Type<'a>),
}

#[derive(Debug, Default, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("Arendeinformation"))]
pub struct Arendeinformation<'a> {
    #[ku(name(b"Arendeagare"), required(true))]
    pub arendeagare: Cow<'a, str>,
    #[ku(name(b"Period"), required(true))]
    pub period: Cow<'a, str>,
    #[ku(name(b"Arendenummer"))]
    pub arendenummer: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("Kontaktperson"))]
pub struct Kontaktperson<'a> {
    #[ku(name(b"Namn"), required(true))]
    pub namn: Cow<'a, str>,
    #[ku(name(b"Telefon"), required(true))]
    pub telefon: Cow<'a, str>,
    #[ku(name(b"Epostadress"), required(true))]
    pub epostadress: Cow<'a, str>,
    #[ku(name(b"Sakomrade"))]
    pub sakomrade: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("Avsandare"))]
pub struct Avsandare<'a> {
    #[ku(name(b"Programnamn"), required(true))]
    pub programnamn: Cow<'a, str>,
    #[ku(name(b"Organisationsnummer"), required(true))]
    pub organisationsnummer: Cow<'a, str>,
    #[ku(name(b"TekniskKontaktperson"), required(true), inner_ty(true))]
    pub teknisk_kontaktperson: TekniskKontaktperson<'a>,
    #[ku(name(b"Skapad"), required(true))]
    pub skapad: Cow<'a, str>,
}

#[derive(Debug, Default, PartialEq, KontrolluppgiftRead, KontrolluppgiftWrite)]
#[ku(name("TekniskKontaktperson"))]
pub struct TekniskKontaktperson<'a> {
    #[ku(name(b"Namn"), required(true))]
    pub namn: Cow<'a, str>,
    #[ku(name(b"Telefon"), required(true))]
    pub telefon: Cow<'a, str>,
    #[ku(name(b"Epostadress"), required(true))]
    pub epostadress: Cow<'a, str>,
    #[ku(name(b"Utdelningsadress1"))]
    pub utdelningsadress1: Option<Cow<'a, str>>,
    #[ku(name(b"Utdelningsadress2"))]
    pub utdelningsadress2: Option<Cow<'a, str>>,
    #[ku(name(b"Postnummer"))]
    pub postnummer: Option<Cow<'a, str>>,
    #[ku(name(b"Postort"))]
    pub postort: Option<Cow<'a, str>>,
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
                b"Skatteverket" => {
                    //Noop just becuase of the way its structured
                }
                b"Avsandare" => g_avsandare = Some(Avsandare::read(&mut reader, &element)?),
                b"Blankettgemensamt" => {
                    blankettgemensamt = Some(Blankettgemensamt::read(&mut reader, &element)?)
                }
                b"Blankett" => blanketter.push(Blankett::read(&mut reader, &element)?),
                _ => unexpected_element(&element)?,
            },
            Event::End(element) => {
                if element.local_name().as_ref() == b"Skatteverket" {
                    return Ok(Kontrolluppgift {
                        avsandare: g_avsandare.ok_or_else(|| MissingElement {
                            missing: "Avsandare".into(),
                            reading: "Skatteverket".into(),
                        })?,
                        blankettgemensamt: blankettgemensamt.ok_or_else(|| MissingElement {
                            missing: "Blankettgemensamt".into(),
                            reading: "Skatteverket".into(),
                        })?,
                        blanketter,
                    });
                }
            }
            Event::Eof => return Err(Error::UnexpectedEof("While reading Skatteverket".into())),
            _ => (),
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
                nummer = Some(
                    a.decode_and_unescape_value(reader)?
                        .parse::<i64>()
                        .map_err(|_| MissingElement {
                            missing: "nummer".into(),
                            reading: "Blankett".into(),
                        })?,
                )
            }
        }
        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"Arendeinformation" => {
                        arendeinformation = Some(Arendeinformation::read(reader, &element)?)
                    }
                    b"Blankettinnehall" => {
                        blankettinnehall = KontrolluppgiftType::read(reader)?;
                    }
                    &_ => unexpected_element(&element)?,
                },
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            nummer: nummer.ok_or_else(|| MissingElement {
                                missing: "nummer".into(),
                                reading: "Blankett".into(),
                            })?,
                            arendeinformation: arendeinformation.ok_or_else(|| MissingElement {
                                missing: "Arendeinformation".into(),
                                reading: "Blankett".into(),
                            })?,
                            blankettinnehall: blankettinnehall.ok_or_else(|| MissingElement {
                                missing: "Blankettinnehall".into(),
                                reading: "Blankett".into(),
                            })?,
                        });
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEof("While reading Blankett".into())),
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

trait KontrolluppgiftRead<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, Error>
    where
        Self: Sized;
}

trait KontrolluppgiftWrite {
    fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error>
    where
        W: std::io::Write;
}

fn unexpected_element<E>(element: &BytesStart) -> Result<E, Error> {
    Err(Error::UnexpectedToken(
        std::str::from_utf8(element.name().as_ref())
            .map_err(|e| NonDecodable(Some(e)))?
            .into(),
    ))
}

trait Write<'a, T>
where
    T: Writable,
{
    fn write_node(&mut self, name: &str, data: T) -> Result<(), quick_xml::Error>;
    fn write_node_with_code(
        &mut self,
        name: &str,
        code: &str,
        data: T,
    ) -> Result<(), quick_xml::Error>;
}

trait Reader<'a, 'b, T>
where
    T: Readable<'a, 'b>,
{
    fn read_node_into(&mut self, element: BytesStart, x: &mut Option<T>) -> Result<(), Error>;
    fn read_node_into_with_code(
        &mut self,
        element: BytesStart,
        code: &str,
        x: &mut Option<T>,
    ) -> Result<(), Error>;
}

impl<'a, 'b: 'a, T: Readable<'a, 'b> + 'b> Reader<'a, 'b, T> for NsReader<&'b [u8]> {
    fn read_node_into(&mut self, element: BytesStart, x: &mut Option<T>) -> Result<(), Error> {
        *x = Some(T::get_str(self.read_text(element.name())?)?);
        Ok(())
    }

    fn read_node_into_with_code(
        &mut self,
        element: BytesStart,
        code: &str,
        x: &mut Option<T>,
    ) -> Result<(), Error> {
        let element_name = element.name();
        let element_name =
            std::str::from_utf8(element_name.as_ref()).map_err(|e| NonDecodable(Some(e)))?;
        let kod = element
            .try_get_attribute("faltkod")?
            .ok_or_else(|| MissingElement {
                missing: "faltkod".into(),
                reading: element_name.into(),
            })?;

        let kod = kod.decode_and_unescape_value(self)?;
        if code != kod {
            return Err(Error::UnexpectedToken(format!(
                "Unexpected faltkod on {}, expected: {}, got: {}",
                element_name, code, kod
            )));
        }
        *x = Some(T::get_str(self.read_text(element.name())?)?);
        Ok(())
    }
}

impl<'a, T: Writable, W: std::io::Write> Write<'a, T> for Writer<W> {
    fn write_node(&mut self, name: &str, data: T) -> Result<(), quick_xml::Error> {
        let str = data.get_str();
        match str {
            None => Ok(()),
            Some(data) => {
                self.create_element(name)
                    .write_text_content(BytesText::new(&data))?;
                Ok(())
            }
        }
    }
    fn write_node_with_code(
        &mut self,
        name: &str,
        code: &str,
        data: T,
    ) -> Result<(), quick_xml::Error> {
        let str = data.get_str();
        match str {
            None => Ok(()),
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
    fn get_str(data: Cow<'a, str>) -> Result<Self, Error>
    where
        Self: Sized + 'b;
}

impl<'a, 'b> Readable<'a, 'b> for bool {
    fn get_str(data: Cow<str>) -> Result<Self, Error> {
        match data.as_ref() {
            "0" => Ok(false),
            "1" => Ok(true),
            &_ => Err(Error::UnexpectedToken(format!(
                "expected KryssTyp, found: {}",
                &data
            ))),
        }
    }
}

impl<'a, 'b> Readable<'a, 'b> for i32 {
    fn get_str(data: Cow<str>) -> Result<Self, Error> {
        data.as_ref()
            .parse()
            .map_err(|_| Error::UnexpectedToken(format!("expected number got: {}", &data)))
    }
}

impl<'a, 'b> Readable<'a, 'b> for f32 {
    fn get_str(data: Cow<str>) -> Result<Self, Error> {
        data.as_ref()
            .parse()
            .map_err(|_| Error::UnexpectedToken(format!("expected fraction got: {}", &data)))
    }
}

impl<'a, 'b: 'a> Readable<'a, 'b> for Cow<'a, str> {
    fn get_str(data: Cow<'b, str>) -> Result<Self, Error> {
        Ok(data)
    }
}

impl<T> Writable for &Option<T>
where
    T: Writable,
{
    fn get_str(&self) -> Option<String> {
        match self {
            None => None,
            Some(v) => v.get_str(),
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
            true => "1".into(),
            false => "0".into(),
        })
    }
}

impl<'a> Writable for &Cow<'a, str> {
    fn get_str(&self) -> Option<String> {
        Some(self.to_string())
    }
}

#[derive(Debug, PartialEq, KUStringEnum)]
pub enum NarfartFjarrfart {
    N,
    F,
}

#[derive(Debug, PartialEq, KUStringEnum)]
pub enum Landskod {
    AD,
    AE,
    AF,
    AG,
    AI,
    AL,
    AN,
    AM,
    AO,
    AQ,
    AR,
    AS,
    AT,
    AU,
    AW,
    AX,
    AZ,
    BA,
    BB,
    BD,
    BE,
    BF,
    BG,
    BH,
    BI,
    BJ,
    BL,
    BM,
    BN,
    BO,
    BQ,
    BR,
    BS,
    BT,
    BV,
    BW,
    BY,
    BZ,
    CA,
    CC,
    CD,
    CF,
    CG,
    CH,
    CI,
    CK,
    CL,
    CM,
    CN,
    CO,
    CR,
    CU,
    CV,
    CW,
    CX,
    CY,
    CZ,
    DE,
    DJ,
    DK,
    DM,
    DO,
    DZ,
    EC,
    EE,
    EG,
    EH,
    ER,
    ES,
    ET,
    FI,
    FJ,
    FK,
    FM,
    FO,
    FR,
    GA,
    GB,
    GD,
    GE,
    GF,
    GG,
    GH,
    GI,
    GL,
    GM,
    GN,
    GP,
    GQ,
    GR,
    GS,
    GT,
    GU,
    GW,
    GY,
    HK,
    HM,
    HN,
    HR,
    HT,
    HU,
    ID,
    IE,
    IL,
    IM,
    IN,
    IO,
    IQ,
    IR,
    IS,
    IT,
    JE,
    JM,
    JO,
    JP,
    KE,
    KG,
    KH,
    KI,
    KM,
    KN,
    KP,
    KR,
    KW,
    KY,
    KZ,
    LA,
    LB,
    LC,
    LI,
    LK,
    LR,
    LS,
    LT,
    LU,
    LV,
    LY,
    MA,
    MC,
    MD,
    ME,
    MF,
    MG,
    MH,
    MK,
    ML,
    MM,
    MN,
    MO,
    MP,
    MQ,
    MR,
    MS,
    MT,
    MU,
    MV,
    MW,
    MX,
    MY,
    MZ,
    NA,
    NC,
    NE,
    NF,
    NG,
    NI,
    NL,
    NO,
    NP,
    NR,
    NU,
    NZ,
    OM,
    PA,
    PE,
    PF,
    PG,
    PH,
    PK,
    PL,
    PM,
    PN,
    PR,
    PS,
    PT,
    PW,
    PY,
    QA,
    RE,
    RO,
    RS,
    RU,
    RW,
    SA,
    SB,
    SC,
    SD,
    SE,
    SG,
    SH,
    SI,
    SJ,
    SK,
    SL,
    SM,
    SN,
    SO,
    SR,
    SS,
    ST,
    SV,
    SX,
    SY,
    SZ,
    TC,
    TD,
    TF,
    TG,
    TH,
    TJ,
    TK,
    TL,
    TM,
    TN,
    TO,
    TR,
    TT,
    TV,
    TW,
    TZ,
    UA,
    UG,
    UM,
    US,
    UY,
    UZ,
    VA,
    VC,
    VE,
    VG,
    VI,
    VN,
    VU,
    WF,
    WS,
    XK,
    YE,
    YT,
    ZA,
    ZM,
    ZW,
}

#[derive(Debug, PartialEq)]
pub struct IdentitetsbeteckningForPerson<'a>(Cow<'a, str>);

impl<'a, 'b: 'a> Readable<'a, 'b> for IdentitetsbeteckningForPerson<'a> {
    fn get_str(data: Cow<'b, str>) -> Result<Self, Error> {
        data.as_ref()
            .try_into()
            .map_err(|e: &str| Error::UnexpectedToken(e.to_string()))
    }
}

impl TryFrom<&str> for IdentitetsbeteckningForPerson<'_> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let person_nr_regex = Regex::new(r"((((18|19|20)[0-9][0-9])(((01|03|05|07|08|10|12)(0[1-9]|1[0-9]|2[0-9]|3[0-1]))|((04|06|09|11)(0[1-9]|1[0-9]|2[0-9]|30))|((02)(0[1-9]|1[0-9]|2[0-8]))))|(((18|19|20)(04|08|12|16|20|24|28|32|36|40|44|48|52|56|60|64|68|72|76|80|84|88|92|96)(0229))|(20000229)))(00[1-9]|0[1-9][0-9]|[1-9][0-9][0-9])[0-9]").expect("These are constructed and should be valid");
        let samordnings_nr_regex = Regex::new(r"((((18|19|20)[0-9][0-9])(((01|03|05|07|08|10|12)(6[1-9]|7[0-9]|8[0-9]|9[0-1]))|((04|06|09|11)(6[1-9]|7[0-9]|8[0-9]|90))|((02)(6[1-9]|7[0-9]|8[0-8]))))|(((18|19|20)(04|08|12|16|20|24|28|32|36|40|44|48|52|56|60|64|68|72|76|80|84|88|92|96)(0289))|(20000289)))(00[1-9]|0[1-9][0-9]|[1-9][0-9][0-9])[0-9]").expect("These are constructed and should be valid");
        let org_nr_regex =
            Regex::new(r"16\d{2}[2-9]\d{7}").expect("These are constructed and should be valid");

        if person_nr_regex.is_match(value)
            || samordnings_nr_regex.is_match(value)
            || org_nr_regex.is_match(value)
        {
            return Ok(IdentitetsbeteckningForPerson(Cow::Owned(value.to_string())));
        }
        return Err("Not valid Identitetsbeteckning");
    }
}

impl From<&IdentitetsbeteckningForPerson<'_>> for String {
    fn from(value: &IdentitetsbeteckningForPerson) -> Self {
        value.0.to_string()
    }
}

impl Writable for IdentitetsbeteckningForPerson<'_> {
    fn get_str(&self) -> Option<String> {
        Some(self.0.to_string())
    }
}
