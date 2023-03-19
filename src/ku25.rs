use std::borrow::Cow;
use quick_xml::events::{BytesStart, Event};
use quick_xml::NsReader;
use crate::{DeError, to_bool };
use crate::DeError::{MissingField, UnexpectedElement};

#[derive(Debug, PartialEq)]
pub struct KU25<'a> {
    pub delagare: Option<bool>,
    pub inkomstar: Cow<'a, str>,
    pub borttag: Option<bool>,
    pub avdragsgill_ranta: Option<i32>,
    pub total_inbetald_ranta: Option<i32>,
    pub betald_rantekompensation: Option<i32>,
    pub gemensamt_lan: Option<bool>,
    pub specifikationsnummer: i32,
    pub inkomsttagare: InkomsttagareKU25<'a>,
    pub uppgiftslamnare: UppgiftslamnareKU25<'a>,
}

impl<'a> KU25<'a> {
    pub(crate) fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {
        let mut delagare = None;
        let mut inkomstar = None;
        let mut borttag = None;
        let mut avdragsgill_ranta = None;
        let mut total_inbetald_ranta = None;
        let mut betald_rantekompensation = None;
        let mut gemensamt_lan = None;
        let mut specificationsnummer = None;
        let mut inkomsttagare = None;
        let mut uppgiftslamnare = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                   
                    b"Delagare" => {
                        delagare = Some(to_bool(reader.read_text(element.name()).unwrap())).unwrap();
                    }
                    b"Inkomstar" => {
                        inkomstar = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Borttag" => {
                        borttag = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"AvdragsgillRanta" => {
                        avdragsgill_ranta = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"TotalInbetaldRanta" => {
                        total_inbetald_ranta = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"BetaldRantekompensation" => {
                        betald_rantekompensation = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"GemensamtLan" => {
                        gemensamt_lan = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"Specifikationsnummer" => {
                        specificationsnummer = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"InkomsttagareKU25" => {
                        inkomsttagare = Some(InkomsttagareKU25::read(reader, &element)?)
                    }
                    b"UppgiftslamnareKU25" => {
                        uppgiftslamnare = Some(UppgiftslamnareKU25::read(reader, &element)?)
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            delagare,
                            inkomstar: inkomstar.ok_or_else(|| MissingField("Inkomstar".to_string()))?,
                            borttag,

                            avdragsgill_ranta,
                            total_inbetald_ranta,
                            betald_rantekompensation,
                            gemensamt_lan,
                            specifikationsnummer: specificationsnummer.ok_or_else(|| MissingField("Specifikationsnummer".to_string()))?,
                            inkomsttagare: inkomsttagare.ok_or_else(|| MissingField("InkomsttagareKU25".to_string()))?,
                            uppgiftslamnare: uppgiftslamnare.ok_or_else(|| MissingField("UppgiftslamnareKU25".to_string()))?,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct InkomsttagareKU25<'a> {
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

#[derive(Debug, PartialEq)]
pub struct UppgiftslamnareKU25<'a> {
    pub uppgiftslamnar_id: Cow<'a, str>,
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}

impl<'a> InkomsttagareKU25<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {
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
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
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
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
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
                _ => {}
            }
        }
    }
}
impl<'a> UppgiftslamnareKU25<'a> {
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
    use crate::{from_str};

    #[test]
    fn ku25_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFTER RÄNTA, UTDELNING M.M. KU25 FÖR_2022.xml").unwrap();

        let parsed = from_str(&*xml);
        println!("{:?}", &parsed);
        assert!(parsed.is_ok());
    }
}