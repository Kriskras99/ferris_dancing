//! # Localisation building
//! Build the localisations
use std::collections::HashMap;

use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualPathBuf;
use hipstr::HipStr;
use ubiart_toolkit::loc8::{self, Language, Loc8};

use super::{BuildFiles, BuildState};
use crate::types::localisation::Localisation;

/// The .loc8 files in the game
const LOC8_FILES: &[(Language, &str)] = &[
    (
        Language::Danish,
        "enginedata/localisation/localisation.itf_language_danish.loc8",
    ),
    (
        Language::DevReference,
        "enginedata/localisation/localisation.itf_language_dev_reference.loc8",
    ),
    (
        Language::Dutch,
        "enginedata/localisation/localisation.itf_language_dutch.loc8",
    ),
    (
        Language::English,
        "enginedata/localisation/localisation.itf_language_english.loc8",
    ),
    (
        Language::Finnish,
        "enginedata/localisation/localisation.itf_language_finnish.loc8",
    ),
    (
        Language::French,
        "enginedata/localisation/localisation.itf_language_french.loc8",
    ),
    (
        Language::GavChinese,
        "enginedata/localisation/localisation.itf_language_gavchinese.loc8",
    ),
    (
        Language::German,
        "enginedata/localisation/localisation.itf_language_german.loc8",
    ),
    (
        Language::Italian,
        "enginedata/localisation/localisation.itf_language_italian.loc8",
    ),
    (
        Language::Japanese,
        "enginedata/localisation/localisation.itf_language_japanese.loc8",
    ),
    (
        Language::Korean,
        "enginedata/localisation/localisation.itf_language_korean.loc8",
    ),
    (
        Language::Norwegian,
        "enginedata/localisation/localisation.itf_language_norwegian.loc8",
    ),
    (
        Language::Portuguese,
        "enginedata/localisation/localisation.itf_language_portuguese_br.loc8",
    ),
    (
        Language::Russian,
        "enginedata/localisation/localisation.itf_language_russian.loc8",
    ),
    (
        Language::SimplChinese,
        "enginedata/localisation/localisation.itf_language_simplifiedchinese.loc8",
    ),
    (
        Language::Spanish,
        "enginedata/localisation/localisation.itf_language_spanish.loc8",
    ),
    (
        Language::Swedish,
        "enginedata/localisation/localisation.itf_language_swedish.loc8",
    ),
    (
        Language::TradChinese,
        "enginedata/localisation/localisation.itf_language_traditionalchinese.loc8",
    ),
];

/// Build the localisations
pub fn build(bs: &BuildState, bf: &mut BuildFiles) -> Result<(), Error> {
    println!("Building localisations...");
    // Load localisations
    let localisations = Localisation::load_vfs(bs.native_vfs, &bs.rel_tree)?;
    let mut map: HashMap<Language, Loc8> = HashMap::with_capacity(LOC8_FILES.len());
    let unique_ids = localisations.len();

    for (locale_id, translation) in localisations.entries() {
        for lang in Language::all().iter().copied() {
            map.entry(lang)
                .or_insert_with_key(|language| Loc8 {
                    language: *language,
                    strings: HashMap::with_capacity(unique_ids),
                })
                .strings
                .insert(*locale_id, HipStr::borrowed(translation.get(lang)));
        }
    }

    for (lang, loc8) in map {
        let vec = loc8::create_vec(loc8)?;
        // TODO: replace with phf map when it supports enums as keys
        let path = LOC8_FILES
            .iter()
            .find(|(one, _)| *one == lang)
            .unwrap_or_else(|| unreachable!())
            .1;
        bf.generated_files
            .add_file(VirtualPathBuf::from(path), vec)?;
    }

    Ok(())
}
