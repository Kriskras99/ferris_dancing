//! Contains the types that describe the usefull information in this filetype

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

/// Describes the entire file
#[derive(Debug, Clone)]
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
