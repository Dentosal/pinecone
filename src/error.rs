#![allow(unused_variables)]

use core::fmt::{Display, Formatter};

use crate::prelude::*;

/// This is the error type used by Pinecone
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// This is a feature that Pinecone will never implement
    WontImplement,
    /// The serialize buffer is full
    SerializeBufferFull,
    /// The length of a sequence or map must be known
    SerializeLengthUnknown,
    /// Hit the end of buffer, expected more data
    DeserializeUnexpectedEnd,
    /// Found a varint that didn't terminate. Is the usize too big for this platform?
    DeserializeBadVarint,
    /// Found a bool that wasn't 0 or 1
    DeserializeBadBool,
    /// Found an invalid unicode char
    DeserializeBadChar,
    /// Tried to parse invalid utf-8
    DeserializeBadUtf8,
    /// Found an Option discriminant that wasn't 0 or 1
    DeserializeBadOption,
    /// Found an enum discriminant that was > u32::max_value()
    DeserializeBadEnum,
    /// The original data was not well encoded
    DeserializeBadEncoding,
    /// Serde Serialization Error
    SerdeSerCustom(String),
    /// Serde Deserialization Error
    SerdeDeCustom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// This is the Result type used by Pinecone.
#[must_use]
pub type Result<T> = ::core::result::Result<T, Error>;

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::SerdeSerCustom(format!("{}", msg))
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::SerdeDeCustom(format!("{}", msg))
    }
}

impl serde::ser::StdError for Error {}
