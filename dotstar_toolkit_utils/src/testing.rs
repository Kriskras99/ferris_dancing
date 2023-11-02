//! # Testing
//! Contains functions like `assert!` but they return an `Error` instead of panicking.

use std::fmt::Debug;
use thiserror::Error;

/// Errors returend when the test* functions fail
#[derive(Error, Debug)]
pub enum TestError {
    /// The two values do not match
    #[error("{left} does not match {right}")]
    NotEqual {
        /// Left value
        left: String,
        /// Right value
        right: String,
    },
    /// The two values do match
    #[error("{left} matches {right}")]
    Equal {
        /// Left value
        left: String,
        /// Right value
        right: String,
    },
    /// The value is not any of the right values
    #[error("{left} does not match any value in {right}")]
    NotAny {
        /// Left value
        left: String,
        /// Right value
        right: String,
    },
    /// The value is greater than it's supposed to be
    #[error("{left} is greater than {right}")]
    GreaterThan {
        /// Left value
        left: String,
        /// Right value
        right: String,
    },
}

impl TestError {
    /// Create the [`TestError::NotEqual`] error
    fn not_equal<T: Debug>(left: &T, right: &T) -> Self {
        Self::NotEqual {
            left: format!("{left:?}"),
            right: format!("{right:?}"),
        }
    }

    /// Create the [`TestError::Equal`] error
    fn equal<T: Debug>(left: &T, right: &T) -> Self {
        Self::Equal {
            left: format!("{left:?}"),
            right: format!("{right:?}"),
        }
    }

    /// Create the [`TestError::NotAny`] error
    fn not_any<T: Debug>(left: &T, right: &[T]) -> Self {
        Self::NotAny {
            left: format!("{left:?}"),
            right: format!("{right:?}"),
        }
    }

    /// Create the [`TestError::GreaterThan`] error
    fn greater_than<T: Debug>(left: &T, right: &T) -> Self {
        Self::GreaterThan {
            left: format!("{left:?}"),
            right: format!("{right:?}"),
        }
    }
}

/// Test if `one` == `two` returning a descriptive error if they're not the same.
///
/// # Errors
/// Will return an error if the two inputs are not the same, with a description of the values.
pub fn test<T: PartialEq + Debug>(left: &T, right: &T) -> Result<(), TestError> {
    if left == right {
        Ok(())
    } else {
        Err(TestError::not_equal(left, right))
    }
}

/// Test if `two.contains(one)` returning a descriptive error if `two` does not contain `one`.
///
/// # Errors
/// Will return an error if `two` does not contain `one`, with a description of the values.
pub fn test_any<T: PartialEq + Debug>(left: &T, right: &[T]) -> Result<(), TestError> {
    if right.contains(left) {
        Ok(())
    } else {
        Err(TestError::not_any(left, right))
    }
}

/// Test if `one` != `two` returning a descriptive error if they're the same.
///
/// # Errors
/// Will return an error if the two inputs are the same, with a description of the values.
#[allow(clippy::if_not_else)] // Much clearer this way
pub fn test_not<T: PartialEq + Debug>(left: &T, right: &T) -> Result<(), TestError> {
    if left != right {
        Ok(())
    } else {
        Err(TestError::equal(left, right))
    }
}

/// Test if `one` <= `two` returning a descriptive error if `one` is bigger.
///
/// # Errors
/// Will return an error if `one` is bigger than `two`, with a description of the values.
pub fn test_le<T: PartialOrd + Debug>(left: &T, right: &T) -> Result<(), TestError> {
    if left <= right {
        Ok(())
    } else {
        Err(TestError::greater_than(left, right))
    }
}
