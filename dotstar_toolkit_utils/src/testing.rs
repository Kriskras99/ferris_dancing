//! # Testing
//! Contains functions like `assert!` but they return an `Error` instead of panicking.
use std::{
    convert::Infallible,
    fmt::{Debug, Display, Formatter},
    ops::{ControlFlow, FromResidual, Try},
};

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
    /// Combine two tests, only fail if both tests fail
    pub fn or(self, right: Self) -> Self {
        match (self, right) {
            (Self::Err(left), Self::Err(right)) => Self::Err(TestError::And {
                left: Box::new(left),
                right: Box::new(right),
            }),
            _ => Self::Ok,
        }
    }

    /// Combine two tests, fail if any of the two fail
    pub fn and(self, right: Self) -> Self {
        match (self, right) {
            (Self::Err(left), Self::Err(right)) => Self::Err(TestError::And {
                left: Box::new(left),
                right: Box::new(right),
            }),
            (Self::Err(left), _) => Self::Err(TestError::Or(Box::new(left))),
            (_, Self::Err(right)) => Self::Err(TestError::Or(Box::new(right))),
            _ => Self::Ok,
        }
    }

    /// Ignore any test failures
    pub fn lax(self, lax: bool) -> Self {
        if lax {
            match self {
                Self::Ok => Self::Ok,
                Self::Err(error) => {
                    println!("Warning! Ignoring: {error:?}");
                    Self::Ok
                }
            }
        } else {
            self
        }
    }

    /// Convert this to a normal Result
    #[allow(clippy::missing_errors_doc, reason = "It does not actually error")]
    pub fn result(self) -> Result<(), TestError> {
        self.into()
    }

    /// Is this TestResult an error
    #[must_use]
    pub const fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    /// Is this TestResult not an error
    #[must_use]
    pub const fn is_ok(&self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Panic if this result is an error
    #[allow(clippy::missing_panics_doc, reason = "This is supposed to panic")]
    pub fn unwrap(self) {
        match self {
            Self::Ok => (),
            Self::Err(err) => panic!("{err:?}"),
        }
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

pub enum TestError {
    // #[error("{0}")]
    Normal(String),
    // #[error("Both tests failed:\n1: {left}\n2: {right}")]
    And { left: Box<Self>, right: Box<Self> },
    // #[error("One of the tests failed:\n{0}")]
    Or(Box<Self>),
}

impl std::error::Error for TestError {}

impl Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TestError::Normal(error) => f.write_str(&error)?,
            TestError::And { left, right } => {
                f.write_str("Both tests failed:\n1: ")?;
                f.write_str(&left.to_string())?;
                f.write_str("\n2: ")?;
                f.write_str(&right.to_string())?;
            }
            TestError::Or(error) => {
                f.write_str("One of the tests failed:\n")?;
                f.write_str(&error.to_string())?;
            }
        }
        Ok(())
    }
}

impl Debug for TestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl TestError {
    #[inline(never)]
    #[cold]
    pub fn test_failed_two_idents<T, U>(
        message: &'static str,
        left_ident: &'static str,
        left_val: &T,
        right_ident: &'static str,
        right_val: &U,
        args: Option<std::fmt::Arguments<'_>>,
    ) -> Self
    where
        T: std::fmt::Debug + ?Sized,
        U: std::fmt::Debug + ?Sized,
    {
        Self::test_failed_inner_two_idents(
            message,
            left_ident,
            &left_val,
            right_ident,
            &right_val,
            args,
        )
    }

    fn test_failed_inner_two_idents(
        message: &'static str,
        left_ident: &'static str,
        left_val: &dyn std::fmt::Debug,
        right_ident: &'static str,
        right_val: &dyn std::fmt::Debug,
        args: Option<std::fmt::Arguments<'_>>,
    ) -> Self {
        let msg = match args {
            Some(args) => format!(
                r#"{message}: {args}
{left_ident}: {left_val:?}
{right_ident}: {right_val:?}
"#
            ),
            None => format!(
                r#"{message}
{left_ident}: {left_val:?}
{right_ident}: {right_val:?}
"#
            ),
        };

        Self::Normal(msg)
    }

    #[inline(never)]
    #[cold]
    pub fn test_failed_one_ident<T>(
        message: &'static str,
        ident: &'static str,
        val: &T,
        args: Option<std::fmt::Arguments<'_>>,
    ) -> Self
    where
        T: std::fmt::Debug + ?Sized,
    {
        Self::test_failed_inner_one_ident(message, ident, &val, args)
    }

    fn test_failed_inner_one_ident(
        message: &'static str,
        ident: &'static str,
        val: &dyn std::fmt::Debug,
        args: Option<std::fmt::Arguments<'_>>,
    ) -> Self {
        let msg = match args {
            Some(args) => format!(
                r#"{message}: {args}
{ident}: {val:?}
"#
            ),
            None => format!(
                r#"{message}
{ident}: {val:?}
"#
            ),
        };

        Self::Normal(msg)
    }
}

// /// Errors returend when the test* functions fail
// #[derive(Error, Debug)]
// pub enum TestError {
//     /// TestError with context
//     #[error("{source:?}\n    Context: {context}")]
//     Context {
//         /// The original error
//         #[backtrace]
//         source: Box<Self>,
//         /// Added context
//         context: String,
//     },
//     /// The two values do not match
//     #[error("Test failed: {left} does not match {right}")]
//     NotEqual {
//         /// Left value
//         left: String,
//         /// Right value
//         right: String,
//         /// Backtrace
//         backtrace: Backtrace,
//     },
//     /// The two values do match
//     #[error("Test failed: {left} matches {right}")]
//     Equal {
//         /// Left value
//         left: String,
//         /// Right value
//         right: String,
//         /// Backtrace
//         backtrace: Backtrace,
//     },
//     /// The value is not any of the right values
//     #[error("Test failed: {left} does not match any value in {right}")]
//     NotAny {
//         /// Left value
//         left: String,
//         /// Right value
//         right: String,
//         /// Backtrace
//         backtrace: Backtrace,
//     },
//     /// The value is greater than it's supposed to be
//     #[error("Test failed: {left} is greater than {right}")]
//     GreaterThan {
//         /// Left value
//         left: String,
//         /// Right value
//         right: String,
//         /// Backtrace
//         backtrace: Backtrace,
//     },
//     /// The value is smaller than it's supposed to be
//     #[error("Test failed: {left} is smaller than {right}")]
//     SmallerThan {
//         /// Left value
//         left: String,
//         /// Right value
//         right: String,
//         /// Backtrace
//         backtrace: Backtrace,
//     },
//     /// Both tests failed
//     #[error("Both tests failed:\n    {source:?}\n    {other:?}")]
//     And {
//         /// The original left result
//         #[backtrace]
//         source: Box<Self>,
//         /// The original right result
//         other: Box<Self>,
//     },
//     /// One of the tests failed
//     #[error("One of two tests failed:\n    {source:?}")]
//     Or {
//         /// The original failed result
//         #[backtrace]
//         source: Box<Self>,
//     },
// }

// impl TestError {
//     /// Create the [`TestError::NotEqual`] error
//     fn not_equal<T: Debug>(left: T, right: T) -> Self {
//         Self::NotEqual {
//             left: format!("{left:?}"),
//             right: format!("{right:?}"),
//             backtrace: Backtrace::capture(),
//         }
//     }

//     /// Create the [`TestError::Equal`] error
//     fn equal<T: Debug>(left: T, right: T) -> Self {
//         Self::Equal {
//             left: format!("{left:?}"),
//             right: format!("{right:?}"),
//             backtrace: Backtrace::capture(),
//         }
//     }

//     /// Create the [`TestError::NotAny`] error
//     fn not_any<T: Debug>(left: &T, right: &[T]) -> Self {
//         Self::NotAny {
//             left: format!("{left:?}"),
//             right: format!("{right:?}"),
//             backtrace: Backtrace::capture(),
//         }
//     }

//     /// Create the [`TestError::GreaterThan`] error
//     fn greater_than<T: Debug>(left: T, right: T) -> Self {
//         Self::GreaterThan {
//             left: format!("{left:?}"),
//             right: format!("{right:?}"),
//             backtrace: Backtrace::capture(),
//         }
//     }

//     /// Create the [`TestError::SmallerThan`] error
//     fn smaller_than<T: Debug>(left: T, right: T) -> Self {
//         Self::GreaterThan {
//             left: format!("{left:?}"),
//             right: format!("{right:?}"),
//             backtrace: Backtrace::capture(),
//         }
//     }
// }

// /// Test if `value` is true.
// ///
// /// # Errors
// /// Will return an error if the input is not true.
// pub fn test(value: bool) -> TestResult {
//     if value {
//         TestResult::Ok
//     } else {
//         TestResult::Err(TestError::not_equal(&false, &true))
//     }
// }

// /// Test if `value` is false.
// ///
// /// # Errors
// /// Will return an error if the input is not false.
// #[allow(clippy::if_not_else, reason = "Much clearer this way")]
// #[inline]
// pub fn test_not(value: bool) -> TestResult {
//     if !value {
//         TestResult::Ok
//     } else {
//         TestResult::Err(TestError::not_equal(&false, &true))
//     }
// }

// /// Test if `one` == `two` returning a descriptive error if they're not the same.
// ///
// /// # Errors
// /// Will return an error if the two inputs are not the same, with a description of the values.
// #[inline]
// pub fn test_eq<T: PartialEq + Debug>(left: T, right: T) -> TestResult {
//     if left == right {
//         TestResult::Ok
//     } else {
//         TestResult::Err(TestError::not_equal(left, right))
//     }
// }

// /// Test if `two.contains(one)` returning a descriptive error if `two` does not contain `one`.
// ///
// /// # Errors
// /// Will return an error if `two` does not contain `one`, with a description of the values.
// #[inline]
// pub fn test_any<T: PartialEq + Debug>(left: &T, right: &[T]) -> TestResult {
//     if right.contains(left) {
//         TestResult::Ok
//     } else {
//         TestResult::Err(TestError::not_any(left, right))
//     }
// }

// /// Test if `one` != `two` returning a descriptive error if they're the same.
// ///
// /// # Errors
// /// Will return an error if the two inputs are the same, with a description of the values.
// #[allow(clippy::if_not_else, reason = "Much clearer this way")]
// #[inline]
// pub fn test_ne<T: PartialEq + Debug>(left: T, right: T) -> TestResult {
//     if left != right {
//         TestResult::Ok
//     } else {
//         TestResult::Err(TestError::equal(left, right))
//     }
// }

// /// Test if `one` <= `two` returning a descriptive error if `one` is bigger.
// ///
// /// # Errors
// /// Will return an error if `one` is bigger than `two`, with a description of the values.
// #[inline]
// pub fn test_le<T: PartialOrd + Debug>(left: T, right: T) -> TestResult {
//     if left <= right {
//         TestResult::Ok
//     } else {
//         TestResult::Err(TestError::greater_than(left, right))
//     }
// }

// /// Test if `one` >= `two` returning a descriptive error if `one` is smaller.
// ///
// /// # Errors
// /// Will return an error if `one` is smaller than `two`, with a description of the values.
// #[inline]
// pub fn test_ge<T: PartialOrd + Debug>(left: T, right: T) -> TestResult {
//     if left >= right {
//         TestResult::Ok
//     } else {
//         TestResult::Err(TestError::smaller_than(left, right))
//     }
// }
