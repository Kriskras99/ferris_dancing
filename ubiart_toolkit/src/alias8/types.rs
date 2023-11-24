//! Contains the types that describe the usefull information in this filetype

use crate::utils::SplitPath;

/// Describes a single alias
#[derive(Debug, Clone)]
pub struct Alias<'a> {
    /// The first alias name
    pub first_alias: &'a str,
    /// The second alias name
    pub second_alias: &'a str,
    /// The (uncooked) path for the alias
    pub path: SplitPath<'a>,
    /// Unknown value
    pub unk3: u16,
}

impl Alias<'_> {
    pub const UNK3: &'static [u16] = &[
        0x8001, 0x8002, 0x8008, 0x8100, 0x83d6, 0x8400, 0x8800, 0x9000, 0xa000, 0xc000, 0xe001,
        0xe002, 0xe008, 0xe100, 0xe400, 0xe800, 0xefdf, 0xf000, 0xf001, 0xf002, 0xf008, 0xf100,
        0xf400, 0xf800, 0xfc08, 0xfd19, 0xffdf, 0xffff,
    ];
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
                return Some(a.path.to_string());
            }
        }
        None
    }
}
