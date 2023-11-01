//! # Testing
//! Contains functions like `assert!` but they return an `Error` instead of panicking.

use anyhow::{anyhow, Error};
use std::fmt::Debug;

/// Test if `one` == `two` returning a descriptive error if they're not the same.
///
/// # Errors
/// Will return an error if the two inputs are not the same, with a description of the values.
///
/// # Panics
/// When compiled in debug mode, it will panic instead of returning an error.
pub fn test<T: PartialEq + Debug>(one: &T, two: &T) -> Result<(), Error> {
    // debug_assert!(one == two, "{one:?} does not match {two:?}!");
    if one == two {
        Ok(())
    } else {
        Err(anyhow!("{one:?} does not match {two:?}!"))
    }
}

/// Test if `two.contains(one)` returning a descriptive error if `two` does not contain `one`.
///
/// # Errors
/// Will return an error if `two` does not contain `one`, with a description of the values.
///
/// # Panics
/// When compiled in debug mode, it will panic instead of returning an error.
pub fn test_any<T: PartialEq + Debug>(one: &T, two: &[T]) -> Result<(), Error> {
    // debug_assert!(two.contains(one), "{one:?} does not match any value in {two:?}!");
    if two.contains(one) {
        Ok(())
    } else {
        Err(anyhow!("{one:?} does not match any value in {two:?}!"))
    }
}

/// Test if `one` != `two` returning a descriptive error if they're the same.
///
/// # Errors
/// Will return an error if the two inputs are the same, with a description of the values.
///
/// # Panics
/// When compiled in debug mode, it will panic instead of returning an error.
#[allow(clippy::if_not_else)] // Much clearer this way
pub fn test_not<T: PartialEq + Debug>(one: &T, two: &T) -> Result<(), Error> {
    debug_assert!(one != two, "{one:?} is the same as {two:?}!");
    if one != two {
        Ok(())
    } else {
        Err(anyhow!("{one:?} is the same as {two:?}!"))
    }
}

/// Test if `one` <= `two` returning a descriptive error if `one` is bigger.
///
/// # Errors
/// Will return an error if `one` is bigger than `two`, with a description of the values.
///
/// # Panics
/// When compiled in debug mode, it will panic instead of returning an error.
pub fn test_le<T: PartialOrd + Debug>(one: &T, two: &T) -> Result<(), Error> {
    debug_assert!(one <= two, "{one:?} is larger than {two:?}!");
    if one <= two {
        Ok(())
    } else {
        Err(anyhow!("{one:?} is larger than {two:?}!"))
    }
}
