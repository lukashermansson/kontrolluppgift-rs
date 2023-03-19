use std::borrow::Cow;
use quick_xml::events::{BytesStart, Event};
use quick_xml::NsReader;
use crate::{DeError, to_bool };
use crate::DeError::{MissingField, UnexpectedElement};

/// Kontrolluppgift 21
#[derive(Debug, PartialEq)]
pub struct KU21<'a> {
    pub avdragen_skatt: Option<i32>,
    pub inkomstar: Cow<'a, str>,
    pub borttag: Option<bool>,
    pub annan_inkomst: Option<i32>,
    pub ranta_fodringsratter: Option<i32>,
    pub utbetalt_i_vissa_fall: Option<i32>,
    pub depanummer: Option<i32>,
    pub andel_av_depan: Option<f32>,
    pub erhallen_rantekompensation: Option<f32>,
    pub specifikationsnummer: i32,
    pub vp_namn: Option<Cow<'a, str>>,
    pub isin: Option<Cow<'a, str>>,
    pub avyttrad_till_isk: Option<bool>,
    pub okand_varde: Option<bool>,
    pub inkomsttagare: InkomsttagareKU21<'a>,
    pub uppgiftslamnare: UppgiftslamnareKU21<'a>,

}

#[derive(Debug, PartialEq)]
pub struct InkomsttagareKU21<'a> {
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

#[derive(Debug, PartialEq)]
pub struct UppgiftslamnareKU21<'a> {
    pub uppgiftslamnar_id: Cow<'a, str>,
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}
impl<'a> KU21<'a> {
    pub(crate) fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {
        let mut avdragen_skatt = None;
        let mut inkomstar = None;
        let mut borttag = None;
        let mut annan_inkomst = None;
        let mut ranta_fodringsratter = None;
        let mut utbetalt_i_vissa_fall = None;
        let mut depanummer = None;
        let mut andel_av_depan = None;
        let mut erhallen_rantekompensation = None;
        let mut specificationsnummer = None;
        let mut vp_namn = None;
        let mut isin = None;
        let mut avyttrad_till_isk = None;
        let mut okand_varde = None;
        let mut inkomsttagare = None;
        let mut uppgiftslamnare = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"AvdragenSkatt" => {
                        avdragen_skatt = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"Inkomstar" => {
                        inkomstar = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Borttag" => {
                        borttag = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"AnnanInkomst" => {
                        annan_inkomst = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"RantaFordringsratter" => {
                        ranta_fodringsratter = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"UtbetaltIVissaFall" => {
                        utbetalt_i_vissa_fall = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"Depanummer" => {
                        depanummer = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"AndelAvDepan" => {
                        andel_av_depan = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"ErhallenRantekompensation" => {
                        erhallen_rantekompensation = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"Specifikationsnummer" => {
                        specificationsnummer = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"VPNamn" => {
                        vp_namn = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"ISIN" => {
                         isin = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"AvyttradTillISK" => {
                        avyttrad_till_isk = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"OkandVarde" => {
                        okand_varde = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"InkomsttagareKU21" => {
                        inkomsttagare = Some(InkomsttagareKU21::read(reader, &element)?)
                    }
                    b"UppgiftslamnareKU21" => {
                        uppgiftslamnare = Some(UppgiftslamnareKU21::read(reader, &element)?)
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            avdragen_skatt,
                            inkomstar: inkomstar.ok_or_else(|| MissingField("Inkomstar".to_string()))?,
                            borttag,
                            annan_inkomst,
                            ranta_fodringsratter,
                            utbetalt_i_vissa_fall,
                            depanummer,
                            andel_av_depan,
                            erhallen_rantekompensation,
                            specifikationsnummer: specificationsnummer.ok_or_else(|| MissingField("Specifikationsnummer".to_string()))?,
                            vp_namn,
                            isin,
                            avyttrad_till_isk,
                            okand_varde,
                            inkomsttagare: inkomsttagare.ok_or_else(|| MissingField("InkomsttagareKU21".to_string()))?,
                            uppgiftslamnare: uppgiftslamnare.ok_or_else(|| MissingField("UppgiftslamnareKU21".to_string()))?,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}
impl<'a> InkomsttagareKU21<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {

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
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"LandskodTIN" => {
                        landskod_tin = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Fodelseort" => {
                        fodelseort = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"LandskodFodelseort" => {
                        landskod_fodelseort = Some(reader.read_text(element.name()).unwrap());
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
impl<'a> UppgiftslamnareKU21<'a> {
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
    fn ku21_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFTER RÄNTA, UTDELNING M.M. KU21 FÖR_2022.xml").unwrap();

        let parsed = from_str(&*xml);

        println!("{:?}", &parsed);
        assert!(parsed.is_ok())
    }
}