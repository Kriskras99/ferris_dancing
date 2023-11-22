//! # Testing
//! Contains functions like `assert!` but they return an `Error` instead of panicking.
use std::{
    convert::Infallible,
    fmt::Debug,
    ops::{ControlFlow, FromResidual, Try},
};
use thiserror::Error;

/// The result of a test
#[derive(Debug)]
#[must_use]
pub enum TestResult {
    /// Test passed
    Ok,
    /// Test did not pass
    Err(TestError),
}

impl TestResult {
    /// Ignore any test failures
    pub fn lax(self, lax: bool) -> Self {
        if lax {
            match self {
                Self::Ok => Self::Ok,
                Self::Err(error) => {
                    println!("Warning! Ignoring {error:?}");
                    Self::Ok
                }
            }
        } else {
            self
        }
    }

    /// Add context for this test
    pub fn context<C: Debug>(self, context: C) -> Self {
        match self {
            Self::Ok => Self::Ok,
            Self::Err(source) => Self::Err(TestError::Context {
                source: Box::new(source),
                context: format!("{context:?}"),
            }),
        }
    }

    /// Add context for this test
    pub fn with_context<C: Debug, F: FnOnce() -> C>(self, f: F) -> Self {
        match self {
            Self::Ok => Self::Ok,
            Self::Err(source) => Self::Err(TestError::Context {
                source: Box::new(source),
                context: format!("{:?}", f()),
            }),
        }
    }

    /// Convert this to a normal Result
    #[allow(clippy::missing_errors_doc)]
    pub fn result(self) -> Result<(), TestError> {
        self.into()
    }

    /// Is this TestResult an error
    #[must_use]
    pub const fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }
}

impl From<TestResult> for Result<(), TestError> {
    fn from(value: TestResult) -> Self {
        match value {
            TestResult::Ok => Ok(()),
            TestResult::Err(error) => Err(error),
        }
    }
}

impl FromResidual<Result<Infallible, TestError>> for TestResult {
    fn from_residual(_residual: Result<Infallible, TestError>) -> Self {
        Self::Ok
    }
}

impl Try for TestResult {
    type Output = ();

    type Residual = Result<Infallible, TestError>;

    fn from_output(_output: Self::Output) -> Self {
        Self::Ok
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok => ControlFlow::Continue(()),
            Self::Err(error) => ControlFlow::Break(Err(error)),
        }
    }
}

/// Errors returend when the test* functions fail
#[derive(Error, Debug)]
pub enum TestError {
    /// TestError with context
    #[error("{source:?}\nContext: {context}")]
    Context {
        /// The original error
        source: Box<Self>,
        /// Added context
        context: String,
    },
    /// The two values do not match
    #[error("Test failed: {left} does not match {right}")]
    NotEqual {
        /// Left value
        left: String,
        /// Right value
        right: String,
    },
    /// The two values do match
    #[error("Test failed: {left} matches {right}")]
    Equal {
        /// Left value
        left: String,
        /// Right value
        right: String,
    },
    /// The value is not any of the right values
    #[error("Test failed: {left} does not match any value in {right}")]
    NotAny {
        /// Left value
        left: String,
        /// Right value
        right: String,
    },
    /// The value is greater than it's supposed to be
    #[error("Test failed: {left} is greater than {right}")]
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
pub fn test<T: PartialEq + Debug>(left: &T, right: &T) -> TestResult {
    if left == right {
        TestResult::Ok
    } else {
        TestResult::Err(TestError::not_equal(left, right))
    }
}

/// Test if `two.contains(one)` returning a descriptive error if `two` does not contain `one`.
///
/// # Errors
/// Will return an error if `two` does not contain `one`, with a description of the values.
pub fn test_any<T: PartialEq + Debug>(left: &T, right: &[T]) -> TestResult {
    if right.contains(left) {
        TestResult::Ok
    } else {
        TestResult::Err(TestError::not_any(left, right))
    }
}

/// Test if `one` != `two` returning a descriptive error if they're the same.
///
/// # Errors
/// Will return an error if the two inputs are the same, with a description of the values.
#[allow(clippy::if_not_else)] // Much clearer this way
pub fn test_not<T: PartialEq + Debug>(left: &T, right: &T) -> TestResult {
    if left != right {
        TestResult::Ok
    } else {
        TestResult::Err(TestError::equal(left, right))
    }
}

/// Test if `one` <= `two` returning a descriptive error if `one` is bigger.
///
/// # Errors
/// Will return an error if `one` is bigger than `two`, with a description of the values.
pub fn test_le<T: PartialOrd + Debug>(left: &T, right: &T) -> TestResult {
    if left <= right {
        TestResult::Ok
    } else {
        TestResult::Err(TestError::greater_than(left, right))
    }
}
