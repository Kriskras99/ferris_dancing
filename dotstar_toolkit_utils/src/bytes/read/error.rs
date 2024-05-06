//! Errors for the read implementations

use std::{
    backtrace::Backtrace, io::ErrorKind, num::TryFromIntError, str::Utf8Error,
    string::FromUtf8Error,
};

use thiserror::Error;

use crate::testing::TestError;

/// Errors returend when the test* functions fail
#[derive(Error, Debug)]
pub enum ReadError {
    /// ReadError with context
    #[error("{source:?}\n    Context: {context}")]
    Context {
        /// The original error
        source: Box<Self>,
        /// Added context
        context: String,
    },
    /// Encountered invalid UTF-8 when trying to read a string from source
    #[error("invalid UTF-8 encountered: {error}")]
    InvalidUTF8 {
        /// Original UTF-8 error
        #[from]
        error: Utf8Error,
        /// Backtrace
        backtrace: Backtrace,
    },
    #[error("no null-byte for null terminated string, while reading a string at {position}")]
    /// Encountered no null byte when trying to read a null-terminated string
    NoNullByte {
        /// Position in the source
        position: u64,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// Encountered an I/O error while trying to read from the source
    #[error("io error occured while trying to read from the source: {error}")]
    IoError {
        /// The error
        #[from]
        error: std::io::Error,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// A read value did not match the expected value
    #[error("test failed: {test:?}")]
    Test {
        /// The original test error
        #[from]
        test: TestError,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// An integer could not be converted to another integer size
    #[error("an integer could not be converted to another integer size: {tfie:?}")]
    IntConversion {
        /// The original test error
        #[from]
        tfie: TryFromIntError,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// Create a custom [`ReadError`]
    #[error("{string}")]
    Custom {
        /// The error description
        string: String,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// Integer over/underflow
    #[error("an integer over/underflow occured")]
    IntUnderOverflow {
        /// Backtrace
        backtrace: Backtrace,
    },
}

impl From<FromUtf8Error> for ReadError {
    fn from(value: FromUtf8Error) -> Self {
        value.utf8_error().into()
    }
}

impl ReadError {
    #[must_use]
    /// The byte source ended while reading
    pub fn unexpected_eof() -> Self {
        Self::IoError {
            error: ErrorKind::UnexpectedEof.into(),
            backtrace: Backtrace::capture(),
        }
    }

    #[must_use]
    /// An integer under- or overflowed
    pub fn int_under_overflow() -> Self {
        Self::IntUnderOverflow {
            backtrace: Backtrace::capture(),
        }
    }

    /// Create the [`ReadError::NoNullByte`] error
    #[must_use]
    pub fn no_null_byte(position: u64) -> Self {
        Self::NoNullByte {
            position,
            backtrace: Backtrace::capture(),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn context<C: std::fmt::Debug>(self, context: C) -> Self {
        Self::Context {
            source: Box::new(self),
            context: format!("{context:?}"),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn with_context<C: std::fmt::Debug, F: FnOnce() -> C>(self, f: F) -> Self {
        Self::Context {
            source: Box::new(self),
            context: format!("{:?}", f()),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn custom(string: String) -> Self {
        Self::Custom {
            string,
            backtrace: Backtrace::capture(),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn with_custom<F: FnOnce() -> String>(f: F) -> Self {
        Self::Custom {
            string: f(),
            backtrace: Backtrace::capture(),
        }
    }
}
