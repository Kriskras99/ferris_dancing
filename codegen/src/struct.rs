use std::fmt::{self, Write};

use crate::field::Field;
use crate::fields::Fields;
use crate::formatter::Formatter;
use crate::type_def::TypeDef;

use crate::r#type::Type;

/// Defines a struct.
#[derive(Debug, Clone)]
pub struct Struct {
    type_def: TypeDef,

    /// Struct fields
    fields: Fields,
}

impl Struct {
    /// Return a structure definition with the provided name
    pub fn new(name: impl ToString) -> Self {
        Struct {
            type_def: TypeDef::new(name),
            fields: Fields::Empty,
        }
    }

    /// Returns a reference to the type
    pub fn ty(&self) -> &Type {
        &self.type_def.ty
    }

    /// Set the structure visibility.
    pub fn vis(&mut self, vis: impl ToString) -> &mut Self {
        self.type_def.vis(vis);
        self
    }

    /// Add a generic to the struct.
    pub fn generic(&mut self, name: impl ToString) -> &mut Self {
        self.type_def.ty.generic(name);
        self
    }

    /// Add a `where` bound to the struct.
    pub fn bound<T>(&mut self, name: impl ToString, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.type_def.bound(name, ty);
        self
    }

    /// Set the structure documentation.
    pub fn doc(&mut self, docs: impl ToString) -> &mut Self {
        self.type_def.doc(docs);
        self
    }

    /// Add a new type that the struct should derive.
    pub fn derive(&mut self, name: impl ToString) -> &mut Self {
        self.type_def.derive(name);
        self
    }

    /// Add a new attribute to the struct
    pub fn attr(&mut self, attr: impl ToString) -> &mut Self {
        self.type_def.attr(attr);
        self
    }

    /// Specify lint attribute to supress a warning or error.
    pub fn allow(&mut self, allow: impl ToString) -> &mut Self {
        self.type_def.allow(allow);
        self
    }

    /// Specify representation.
    pub fn repr(&mut self, repr: impl ToString) -> &mut Self {
        self.type_def.repr(repr);
        self
    }

    /// Add an arbitrary macro.
    pub fn r#macro(&mut self, r#macro: impl ToString) -> &mut Self {
        self.type_def.r#macro(r#macro);
        self
    }

    /// Push a named field to the struct.
    ///
    /// A struct can either set named fields with this function or tuple fields
    /// with `push_tuple_field`, but not both.
    pub fn push_field(&mut self, field: Field) -> &mut Self {
        self.fields.push_named(field);
        self
    }

    /// Add a named field to the struct.
    ///
    /// A struct can either set named fields with this function or tuple fields
    /// with `tuple_field`, but not both.
    pub fn field<T>(&mut self, name: impl ToString, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.named(name, ty);
        self
    }

    /// Create a named field for the struct.
    ///
    /// A struct can either set named fields with this function or tuple fields
    /// with `tuple_field`, but not both.
    pub fn new_field<T>(&mut self, name: impl ToString, ty: T) -> &mut Field
    where
        T: Into<Type>,
    {
        self.fields.new_named(name, ty)
    }

    /// Add a tuple field to the struct.
    ///
    /// A struct can either set tuple fields with this function or named fields
    /// with `field`, but not both.
    pub fn tuple_field<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.tuple(ty);
        self
    }

    /// Formats the struct using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("struct", &[], fmt)?;
        self.fields.fmt(fmt)?;

        match self.fields {
            Fields::Empty => {
                write!(fmt, ";\n")?;
            }
            Fields::Tuple(..) => {
                write!(fmt, ";\n")?;
            }
            _ => {}
        }

        Ok(())
    }
}
