use std::fmt::{self, Write};

use crate::fields::Fields;
use crate::formatter::Formatter;

use crate::r#type::Type;

/// Defines an enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    name: String,
    fields: Fields,
    /// Annotations for field e.g., `#[serde(rename = "variant")]`.
    annotations: Vec<String>,
}

impl Variant {
    /// Return a new enum variant with the given name.
    pub fn new(name: impl ToString) -> Self {
        Variant {
            name: name.to_string(),
            fields: Fields::Empty,
            annotations: Vec::new(),
        }
    }

    /// Add a named field to the variant.
    pub fn named<T>(&mut self, name: impl ToString, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the variant.
    pub fn tuple(&mut self, ty: impl ToString) -> &mut Self {
        self.fields.tuple(ty);
        self
    }

    /// Add an anotation to the variant.
    pub fn annotation(&mut self, annotation: impl Into<String>) -> &mut Self {
        self.annotations.push(annotation.into());
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for a in &self.annotations {
            write!(fmt, "{}", a)?;
            write!(fmt, "\n")?;
        }
        write!(fmt, "{}", self.name)?;
        self.fields.fmt(fmt)?;
        write!(fmt, ",\n")?;

        Ok(())
    }
}
