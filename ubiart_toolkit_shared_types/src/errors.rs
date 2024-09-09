use std::{fmt::Debug, num::TryFromIntError, str::Utf8Error};

use dotstar_toolkit_utils::{bytes::read::ReadError, testing::TestError};
use thiserror::Error;

/// Errors returend when parsers fail
#[derive(Error, Debug)]
pub enum ParserError {
    /// ParserError with context
    #[error("{source:?}\n    Context: {context}")]
    Context {
        /// The original error
        source: Box<Self>,
        /// Added context
        context: String,
    },
    /// I/O failure
    #[error("I/O failure: {io:?}")]
    Io {
        /// The original I/O error
        #[from]
        io: std::io::Error,
    },
    /// Read failure
    #[error("Read error: {read:?}")]
    Read {
        /// The original read error
        #[from]
        read: ReadError,
    },
    /// Test failure
    #[error("Value test failed: {test:?}")]
    Test {
        /// The original test error
        #[from]
        test: TestError,
    },
    /// Integer conversion failed
    #[error("Integer conversion failed: {try_from_int:?}")]
    TryFromInt {
        /// The original integer conversion error
        #[from]
        try_from_int: TryFromIntError,
    },
    /// String conversion failed
    #[error("Converting bytes to string failed: {utf8_error:?}")]
    Utf8Error {
        /// The orginal string conversion error
        #[from]
        utf8_error: Utf8Error,
    },
    /// String conversion failed
    #[error("Converting bytes to string failed: {utf8_error:?}")]
    SimdUtf8Error {
        /// The orginal string conversion error
        #[from]
        utf8_error: simdutf8::basic::Utf8Error,
    },
    /// XML deserialization failed
    #[error("XML deserialization failed: {xml_error:?}")]
    XmlError {
        /// The orginal XML deserialization error
        #[from]
        xml_error: quick_xml::DeError,
    },
    /// JSON deserialization failed
    #[error("JSON deserialization failed: {json_error:?}")]
    JSONError {
        /// The orginal JSON deserialization error
        #[from]
        json_error: serde_json::Error,
    },
    /// Custom error
    #[error("{error}")]
    Custom { error: String },
}

impl ParserError {
    /// Create a custom parser error
    ///
    /// `custom` is expected to be: "{Short description of error}: {more details}"
    #[must_use]
    pub fn custom<C: Debug>(custom: C) -> Self {
        Self::Custom {
            error: format!("{custom:?}"),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn context<C: Debug>(self, context: C) -> Self {
        Self::Context {
            source: Box::new(self),
            context: format!("{context:?}"),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn with_context<C: Debug, F: FnOnce() -> C>(self, f: F) -> Self {
        Self::Context {
            source: Box::new(self),
            context: format!("{:?}", f()),
        }
    }
}

/// Errors returend when parsers fail
#[derive(Error, Debug)]
pub enum WriterError {
    /// WriterError with context
    #[error("{source:?}\n    Context: {context}")]
    Context {
        /// The original error
        source: Box<Self>,
        /// Added context
        context: String,
    },
    /// I/O failure
    #[error("I/O failure: {io:?}")]
    Io {
        /// The original I/O error
        #[from]
        io: std::io::Error,
    },
    /// Test failure
    #[error("Value test failed: {test:?}")]
    Test {
        /// The original test error
        #[from]
        test: TestError,
    },
    /// Integer conversion failed
    #[error("Integer conversion failed: {try_from_int:?}")]
    TryFromInt {
        /// The original integer conversion error
        #[from]
        try_from_int: TryFromIntError,
    },
    /// XML serialization failed
    #[error("XML serialization failed: {xml_error:?}")]
    XmlError {
        /// The orginal XML serialization error
        #[from]
        xml_error: quick_xml::DeError,
    },
    /// JSON serialization failed
    #[error("JSON serialization failed: {json_error:?}")]
    JSONError {
        /// The orginal JSON serialization error
        #[from]
        json_error: serde_json::Error,
    },
    /// Parsing failed
    #[error("Parsing failed: {parse_error:?}")]
    ParseError {
        /// The orginal parser error
        #[from]
        parse_error: ParserError,
    },
    /// Custom error
    #[error("{error}")]
    Custom { error: String },
}

impl WriterError {
    /// Create a custom writer error
    ///
    /// `custom` is expected to be: "{Short description of error}: {more details}"
    #[must_use]
    pub fn custom<C: Debug>(custom: C) -> Self {
        Self::Custom {
            error: format!("{custom:?}"),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn context<C: Debug>(self, context: C) -> Self {
        Self::Context {
            source: Box::new(self),
            context: format!("{context:?}"),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn with_context<C: Debug, F: FnOnce() -> C>(self, f: F) -> Self {
        Self::Context {
            source: Box::new(self),
            context: format!("{:?}", f()),
        }
    }
}
