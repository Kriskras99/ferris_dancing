use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// Defines a type.
#[derive(Debug, Clone)]
pub struct Type {
    name: String,
    generics: Vec<Type>,
}

impl Type {
    /// Return a new type with the given name.
    pub fn new(name: impl ToString) -> Self {
        Type {
            name: name.to_string(),
            generics: Vec::new(),
        }
    }

    /// Add a generic to the type.
    pub fn generic<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        // Make sure that the name doesn't already include generics
        assert!(
            !self.name.contains("<"),
            "type name already includes generics"
        );

        self.generics.push(ty.into());
        self
    }

    /// Rewrite the `Type` with the provided path
    ///
    /// TODO: Is this needed?
    pub fn path(&self, path: impl ToString) -> Type {
        // TODO: This isn't really correct
        assert!(!self.name.contains("::"));

        let mut name = path.to_string();
        name.push_str("::");
        name.push_str(&self.name);

        Type {
            name,
            generics: self.generics.clone(),
        }
    }

    /// Formats the struct using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.name)?;
        Type::fmt_slice(&self.generics, fmt)
    }

    fn fmt_slice(generics: &[Type], fmt: &mut Formatter<'_>) -> fmt::Result {
        if !generics.is_empty() {
            write!(fmt, "<")?;

            for (i, ty) in generics.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?
                }
                ty.fmt(fmt)?;
            }

            write!(fmt, ">")?;
        }

        Ok(())
    }
}

impl<S: ToString> From<S> for Type {
    fn from(src: S) -> Self {
        Type {
            name: src.to_string(),
            generics: vec![],
        }
    }
}

impl<'a> From<&'a Type> for Type {
    fn from(src: &'a Type) -> Self {
        src.clone()
    }
}
