//! # Localisation
//! Imports the localisation files and matches them against existing translations
use std::collections::HashMap;

use anyhow::Error;
use dotstar_toolkit_utils::{bytes::read::BinaryDeserializeExt as _, vfs::VirtualFileSystem};
use ubiart_toolkit::loc8::Loc8;

use crate::types::{
    localisation::{LocaleId, LocaleIdMap, Localisation, Translation},
    DirectoryTree,
};

/// Imports the localisation files and matches them against existing translations
pub fn import(vfs: &dyn VirtualFileSystem, dirs: &DirectoryTree) -> Result<LocaleIdMap, Error> {
    // First: load existing translations
    let mut mod_locale = Localisation::load(dirs)?;

    let mut game_locale: HashMap<LocaleId, Translation<'_>> = HashMap::new();

    let mut loc8_files = Vec::new();

    // Second: load new translations
    for file in vfs.walk_filesystem("enginedata/localisation".as_ref())? {
        let loc8_file = vfs.open(file.as_ref())?;
        loc8_files.push(loc8_file);
    }
    for loc8_file in &loc8_files {
        if let Ok(loc) = Loc8::deserialize(loc8_file) {
            let lang = loc.language;

            for (id, translation) in loc.strings {
                game_locale
                    .entry(id)
                    .or_default()
                    .add_translation(lang, translation);
            }
        } else {
            println!("Warning! Parsing of a language file failed!");
        }
    }

    // Third: Merge translations if the mod already has translations, otherwise just set the translations
    let locale_id_map = if mod_locale.is_empty() {
        mod_locale = Localisation::from_game_locale(game_locale);
        let mut locale_id_map = LocaleIdMap::default();
        for key in mod_locale.ids() {
            locale_id_map.insert(key, key);
        }

        locale_id_map
    } else {
        let mut locale_id_map = LocaleIdMap::default();
        for (id_game, translation) in game_locale {
            let id_mod = mod_locale.add_translation(translation);
            locale_id_map.insert(id_game, id_mod);
        }
        locale_id_map
    };

    // Fourth: Save translations
    mod_locale.save(dirs)?;

    Ok(locale_id_map)
}
