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
            Self::Normal(error) => f.write_str(error)?,
            Self::And { left, right } => {
                f.write_str("Both tests failed:\n1: ")?;
                f.write_str(&left.to_string())?;
                f.write_str("\n2: ")?;
                f.write_str(&right.to_string())?;
            }
            Self::Or(error) => {
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
                "{message}: {args}\n{left_ident}: {left_val:?}\n{right_ident}: {right_val:?}"
            ),
            None => format!("{message}\n{left_ident}: {left_val:?}\n{right_ident}: {right_val:?}"),
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
            Some(args) => format!("{message}: {args}\n{ident}: {val:?}"),
            None => format!("{message}\n{ident}: {val:?}"),
        };

        Self::Normal(msg)
    }
}
