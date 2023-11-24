use std::fmt;

use crate::formatter::Formatter;
use crate::type_def::TypeDef;
use crate::variant::Variant;

use crate::r#type::Type;

/// Defines an enumeration.
#[derive(Debug, Clone)]
pub struct Enum {
    type_def: TypeDef,
    variants: Vec<Variant>,
}

impl Enum {
    /// Return a enum definition with the provided name.
    pub fn new(name: impl ToString) -> Self {
        Enum {
            type_def: TypeDef::new(name),
            variants: vec![],
        }
    }

    /// Returns a reference to the type.
    pub fn ty(&self) -> &Type {
        &self.type_def.ty
    }

    /// Set the enum visibility.
    pub fn vis(&mut self, vis: impl ToString) -> &mut Self {
        self.type_def.vis(vis);
        self
    }

    /// Add a generic to the enum.
    pub fn generic(&mut self, name: impl ToString) -> &mut Self {
        self.type_def.ty.generic(name);
        self
    }

    /// Add a `where` bound to the enum.
    pub fn bound<T>(&mut self, name: impl ToString, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.type_def.bound(name, ty);
        self
    }

    /// Set the enum documentation.
    pub fn doc(&mut self, docs: impl ToString) -> &mut Self {
        self.type_def.doc(docs);
        self
    }

    /// Add a new type that the struct should derive.
    pub fn derive(&mut self, name: impl ToString) -> &mut Self {
        self.type_def.derive(name);
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

    /// Push a variant to the enum, returning a mutable reference to it.
    pub fn new_variant(&mut self, name: impl ToString) -> &mut Variant {
        self.push_variant(Variant::new(name.to_string()));
        self.variants.last_mut().unwrap()
    }

    /// Push a variant to the enum.
    pub fn push_variant(&mut self, item: Variant) -> &mut Self {
        self.variants.push(item);
        self
    }

    /// Formats the enum using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("enum", &[], fmt)?;

        fmt.block(|fmt| {
            for variant in &self.variants {
                variant.fmt(fmt)?;
            }

            Ok(())
        })
    }
}
