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
        0x8001, 0x8002, 0x8008, 0x8100, 0x83D6, 0x8400, 0x8800, 0x9000, 0xA000, 0xC000, 0xE001,
        0xE002, 0xE008, 0xE100, 0xE400, 0xE800, 0xEFDF, 0xF000, 0xF001, 0xF002, 0xF008, 0xF100,
        0xF400, 0xF800, 0xFC08, 0xFD19, 0xFFDF, 0xFFFF,
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
