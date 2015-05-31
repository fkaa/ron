use std::error;
use std::fmt;
use std::io;

use serde::de;

/// The errors that can arise while parsing a RON stream.
#[derive(Clone, PartialEq)]
pub enum ErrorCode {
    KeyMustBeAValue,

    InvalidEscape,
    InvalidNumber,
    InvalidUnicodeCodePoint,

    NotFourDigit,
    NotUtf8,

    UnknownVariant,

    UnknownField(String),
    MissingField(&'static str),

    ExpectedColon,
    ExpectedConversion,
    ExpectedEnumEnd,
    ExpectedEnumEndToken,
    ExpectedEnumMapStart,
    ExpectedEnumToken,
    ExpectedEnumVariantString,
    ExpectedListCommaOrEnd,
    ExpectedName,
    ExpectedObjectCommaOrEnd,
    ExpectedSomeIdent,
    ExpectedSomeValue,

    LoneLeadingSurrogateInHexEscape,
    UnexpectedEndOfHexEscape,
    UnrecognizedHex,

    TrailingCharacters,

    EOFWhileParsingObject,
    EOFWhileParsingString,
    EOFWhileParsingArray,
    EOFWhileParsingMap,
    EOFWhileParsingValue,
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Debug;

        match *self {
            ErrorCode::EOFWhileParsingArray => "EOF While parsing array".fmt(f),
            ErrorCode::EOFWhileParsingObject => "EOF While parsing object".fmt(f),
            ErrorCode::EOFWhileParsingString => "EOF While parsing string".fmt(f),
            ErrorCode::EOFWhileParsingValue => "EOF While parsing value".fmt(f),
            ErrorCode::EOFWhileParsingMap => "EOF While parsing map".fmt(f),
            ErrorCode::ExpectedColon => "expected `:`".fmt(f),
            ErrorCode::ExpectedConversion => "expected conversion".fmt(f),
            ErrorCode::ExpectedEnumEnd => "expected enum end".fmt(f),
            ErrorCode::ExpectedEnumEndToken => "expected enum map end".fmt(f),
            ErrorCode::ExpectedEnumMapStart => "expected enum map start".fmt(f),
            ErrorCode::ExpectedEnumToken => "expected enum token".fmt(f),
            ErrorCode::ExpectedEnumVariantString => "expected variant".fmt(f),
            ErrorCode::ExpectedListCommaOrEnd => "expected `,` or `]`".fmt(f),
            ErrorCode::ExpectedName => "expected name".fmt(f),
            ErrorCode::ExpectedObjectCommaOrEnd => "expected `,` or `}`".fmt(f),
            ErrorCode::ExpectedSomeIdent => "expected ident".fmt(f),
            ErrorCode::ExpectedSomeValue => "expected value".fmt(f),
            ErrorCode::InvalidEscape => "invalid escape".fmt(f),
            ErrorCode::InvalidNumber => "invalid number".fmt(f),
            ErrorCode::InvalidUnicodeCodePoint => "invalid unicode code point".fmt(f),
            ErrorCode::KeyMustBeAValue => "key must be a string".fmt(f),
            ErrorCode::LoneLeadingSurrogateInHexEscape => "lone leading surrogate in hex escape".fmt(f),
            ErrorCode::UnknownField(ref field) => write!(f, "unknown field \"{}\"", field),
            ErrorCode::MissingField(ref field) => write!(f, "missing field \"{}\"", field),
            ErrorCode::NotFourDigit => "invalid \\u escape (not four digits)".fmt(f),
            ErrorCode::NotUtf8 => "contents not utf-8".fmt(f),
            ErrorCode::TrailingCharacters => "trailing characters".fmt(f),
            ErrorCode::UnexpectedEndOfHexEscape => "unexpected end of hex escape".fmt(f),
            ErrorCode::UnknownVariant => "unknown variant".fmt(f),
            ErrorCode::UnrecognizedHex => "invalid \\u escape (unrecognized hex)".fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Syntax(ErrorCode, usize, usize),
    Io(io::Error),
    MissingField(&'static str)
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Syntax(..) => "syntax error",
            Error::Io(ref error) => error::Error::description(error),
            Error::MissingField(_) => "missing field",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref error) => Some(error),
            _ => None,
        }
    }

}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(ref code, line, col) => {
                write!(fmt, "{:?} at line {} column {}", code, line, col)
            }
            Error::Io(ref error) => fmt::Display::fmt(error, fmt),
            Error::MissingField(ref field) => {
                write!(fmt, "missing field {}", field)
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<de::value::Error> for Error {
    fn from(error: de::value::Error) -> Error {
        match error {
            de::value::Error::SyntaxError => {
                de::Error::syntax_error()
            }
            de::value::Error::EndOfStreamError => {
                de::Error::end_of_stream_error()
            }
            de::value::Error::UnknownFieldError(field) => {
                Error::Syntax(ErrorCode::UnknownField(field), 0, 0)
            }
            de::value::Error::MissingFieldError(field) => {
                de::Error::missing_field_error(field)
            }
        }
    }
}

impl de::Error for Error {
    fn syntax_error() -> Error {
        Error::Syntax(ErrorCode::ExpectedSomeValue, 0, 0)
    }

    fn end_of_stream_error() -> Error {
        Error::Syntax(ErrorCode::EOFWhileParsingValue, 0, 0)
    }

    fn unknown_field_error(field: &str) -> Error {
        Error::Syntax(ErrorCode::UnknownField(field.to_string()), 0, 0)
    }

    fn missing_field_error(field: &'static str) -> Error {
        Error::MissingField(field)
    }
}
