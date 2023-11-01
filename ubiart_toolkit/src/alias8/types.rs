//! Contains the types that describe the usefull information in this filetype

use stable_deref_trait::StableDeref;
use yoke::{Yoke, Yokeable};

/// Describes a single alias
#[derive(Debug, Clone)]
pub struct Alias<'a> {
    /// The first alias name
    pub first_alias: &'a str,
    /// The second alias name
    pub second_alias: &'a str,
    /// The filename for the alias
    pub filename: &'a str,
    /// The (uncooked) path for the alias
    pub path: &'a str,
    /// Unknown value
    pub unk3: u16,
}

pub struct Alias8Owned<C: StableDeref> {
    /// Internal yoke, always hide from the user
    yoke: Yoke<Alias8<'static>, C>,
}

impl<C: StableDeref> From<Yoke<Alias8<'static>, C>> for Alias8Owned<C> {
    fn from(yoke: Yoke<Alias8<'static>, C>) -> Self {
        Self { yoke }
    }
}

impl<'a, C: StableDeref> Alias8Owned<C> {
    pub fn aliases(&'a self) -> &[Alias<'a>] {
        &self.yoke.get().aliases
    }

    pub fn alias8(&'a self) -> &'a Alias8<'a> {
        self.yoke.get()
    }
}

/// Describes the entire file
#[derive(Debug, Clone, Yokeable)]
pub struct Alias8<'a> {
    /// The aliases in this file
    pub aliases: Vec<Alias<'a>>,
}

impl Alias8<'_> {
    /// Find the path for a given alias
    #[must_use]
    pub fn get_path_for_alias(&self, alias: &str) -> Option<String> {
        for a in &self.aliases {
            if a.first_alias == alias || a.second_alias == alias {
                return Some(format!("{}{}", a.path, a.filename));
            }
        }
        None
    }
}
