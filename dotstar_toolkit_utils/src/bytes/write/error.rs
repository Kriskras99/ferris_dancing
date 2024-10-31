use std::{backtrace::Backtrace, num::TryFromIntError};

use test_eq::TestFailure;
use thiserror::Error;

/// Errors returend when the test* functions fail
#[derive(Error, Debug)]
pub enum WriteError {
    /// WriteError with context
    #[error("{source:?}\n    Context: {context}")]
    Context {
        /// The original error
        source: Box<Self>,
        /// Added context
        context: String,
    },
    /// Encountered an I/O error while trying to write to the destination
    #[error("IoError occured while trying to write to the destination: {error}")]
    IoError {
        /// The error
        #[from]
        error: std::io::Error,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// A custom error
    #[error("{string}")]
    Custom {
        /// The error description
        string: String,
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
    /// Something went wrong
    #[error("something went wrong: {test:?}")]
    Test {
        /// The original test error
        #[from]
        test: TestFailure,
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

impl WriteError {
    /// Create the [`WriteError::IntUnderOverflow`] error
    #[must_use]
    pub fn int_under_overflow() -> Self {
        Self::IntUnderOverflow {
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

    /// Create a custom [`WriteError`]
    #[must_use]
    pub fn custom(string: String) -> Self {
        Self::Custom {
            string,
            backtrace: Backtrace::capture(),
        }
    }

    /// Create a custom [`WriteError`]
    #[must_use]
    pub fn with_custom<F: FnOnce() -> String>(f: F) -> Self {
        Self::Custom {
            string: f(),
            backtrace: Backtrace::capture(),
        }
    }
}
