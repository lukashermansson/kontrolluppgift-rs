use quick_xml::events::attributes::AttrError;
use quick_xml::utils::write_byte_string;
use quick_xml::Error as QXMLError;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::io::Error as IoError;
use std::str::Utf8Error;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Error {
    Io(Arc<IoError>),
    NonDecodable(Option<Utf8Error>),
    UnexpectedEof(String),
    EndEventMismatch { expected: String, found: String },
    UnexpectedToken(String),
    UnexpectedBang(u8),
    TextNotFound,
    XmlDeclWithoutVersion(Option<String>),
    InvalidAttr(AttrError),
    EscapeError,
    UnknownPrefix(Vec<u8>),
    MissingElement { missing: String, reading: String },
    EmptyDocType,
}

impl From<QXMLError> for Error {
    fn from(value: QXMLError) -> Self {
        match value {
            QXMLError::Io(x) => Self::Io(x),
            QXMLError::NonDecodable(x) => Self::NonDecodable(x),
            QXMLError::UnexpectedEof(x) => Self::UnexpectedEof(x),
            QXMLError::EndEventMismatch { expected, found } => {
                Self::EndEventMismatch { expected, found }
            }
            QXMLError::UnexpectedToken(x) => Self::UnexpectedToken(x),
            QXMLError::UnexpectedBang(x) => Self::UnexpectedBang(x),
            QXMLError::TextNotFound => Self::TextNotFound,
            QXMLError::XmlDeclWithoutVersion(x) => Self::XmlDeclWithoutVersion(x),
            QXMLError::InvalidAttr(x) => Self::InvalidAttr(x),
            QXMLError::EscapeError(_) => Self::EscapeError,
            QXMLError::UnknownPrefix(x) => Self::UnknownPrefix(x),
            QXMLError::EmptyDocType => Self::EmptyDocType,
        }
    }
}

impl From<AttrError> for Error {
    fn from(value: AttrError) -> Self {
        Self::InvalidAttr(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::NonDecodable(None) => write!(f, "Malformed input, decoding impossible"),
            Error::NonDecodable(Some(e)) => write!(f, "Malformed UTF-8 input: {}", e),
            Error::UnexpectedEof(e) => write!(f, "Unexpected EOF during reading {}", e),
            Error::EndEventMismatch { expected, found } => {
                write!(f, "Expecting </{}> found </{}>", expected, found)
            }
            Error::UnexpectedToken(e) => write!(f, "Unexpected token '{}'", e),
            Error::UnexpectedBang(b) => write!(
                f,
                "Only Comment (`--`), CDATA (`[CDATA[`) and DOCTYPE (`DOCTYPE`) nodes can start with a '!', but symbol `{}` found",
                *b as char
            ),
            Error::TextNotFound => write!(f, "Cannot read text, expecting Event::Text"),
            Error::XmlDeclWithoutVersion(e) => write!(
                f,
                "XmlDecl must start with 'version' attribute, found {:?}",
                e
            ),
            Error::InvalidAttr(e) => write!(f, "error while parsing attribute: {}", e),
            Error::EscapeError => write!(f, "escape error"),
            Error::UnknownPrefix(prefix) => {
                f.write_str("Unknown namespace prefix '")?;
                write_byte_string(f, prefix)?;
                f.write_str("'")
            },
            Error::MissingElement { missing, reading }  => write!(f, "Missing element {}, while reading {}", missing, reading),
            Error::EmptyDocType => write!(f, "DOCTYPE declaration must not be empty"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::NonDecodable(Some(e)) => Some(e),
            Error::InvalidAttr(e) => Some(e),
            _ => None,
        }
    }
}
