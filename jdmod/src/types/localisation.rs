//! # Localisation
//! Implements functionality for dealing with translations and locale ids
use std::{borrow::Cow, collections::HashMap, fs::File};

use anyhow::{anyhow, Error};

use bitvec::BitArr;
use ubiart_toolkit::loc8::Language;
pub use ubiart_toolkit::utils::LocaleId;

use super::DirectoryTree;

/// All language files that could be found in a mod directory
pub const LANGUAGE_FILES: &[(Language, &str)] = &[
    (Language::English, "english.json"),
    (Language::French, "french.json"),
    (Language::Japanese, "japanese.json"),
    (Language::German, "german.json"),
    (Language::Spanish, "spanish.json"),
    (Language::Italian, "italian.json"),
    (Language::Korean, "korean.json"),
    (Language::TradChinese, "trad_chinese.json"),
    (Language::Portuguese, "portuguese.json"),
    (Language::SimplChinese, "simpl_chinese.json"),
    (Language::Russian, "russian.json"),
    (Language::Dutch, "dutch.json"),
    (Language::Danish, "danish.json"),
    (Language::Norwegian, "norwegian.json"),
    (Language::Swedish, "swedish.json"),
    (Language::Finnish, "finnish.json"),
    (Language::GavChinese, "gav_chinese.json"),
    (Language::DevReference, "dev_reference.json"),
];

/// Maps locale ids from the game currently being parsed to the locale ids of the mod
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct LocaleIdMap {
    /// Mapping game LocaleId to mod LocaleId
    id_map: HashMap<LocaleId, LocaleId>,
}

impl Default for LocaleIdMap {
    fn default() -> Self {
        Self {
            // Games have between 5500 and 9100 ids
            id_map: HashMap::with_capacity(10_000),
        }
    }
}

impl LocaleIdMap {
    /// Map a new game id to a mod id
    pub fn insert(&mut self, id_game: LocaleId, id_mod: LocaleId) {
        assert!(
            self.id_map.insert(id_game, id_mod).is_none(),
            "Game locale id already exists!"
        );
    }

    /// Get the right mod id for a game id
    pub fn get(&self, id_game: LocaleId) -> Result<LocaleId, Error> {
        self.id_map
            .get(&id_game)
            .copied()
            .ok_or_else(|| anyhow!("LocaleId {id_game:?} unknown!"))
    }
}

/// Maps all locale ids to their translations
#[derive(Debug, Clone)]
pub struct Localisation<'a> {
    // TODO: Convert to BiMap that supports M-N mappings (where M >= N)
    /// Maps the mod's locale ids to translations
    translations: HashMap<LocaleId, Translation<'a>>,
    /// Maps translations to the mod's locale ids
    reverse: HashMap<Translation<'a>, LocaleId>,
    /// The next available locale id
    free_id: LocaleId,
}

impl Localisation<'_> {
    /// Load all existing translations in the mod
    pub fn load(dir_tree: &DirectoryTree) -> Result<Self, Error> {
        let mut translations: HashMap<LocaleId, Translation<'_>> = HashMap::new();
        for (lang, file) in LANGUAGE_FILES {
            let path = dir_tree.translations().join(file);
            if let Ok(file) = File::open(path) {
                let new_translations: HashMap<LocaleId, Cow<'_, str>> =
                    serde_json::from_reader(file)?;
                for (id, translation) in new_translations {
                    translations
                        .entry(id)
                        .or_default()
                        .add_translation(*lang, translation);
                }
            }
        }
        let reverse: HashMap<Translation<'_>, LocaleId> =
            translations.iter().map(|(k, v)| (v.clone(), *k)).collect();

        let free_id = translations
            .keys()
            .copied()
            .max()
            .unwrap_or(LocaleId::MIN)
            .increment();

        Ok(Self {
            translations,
            reverse,
            free_id,
        })
    }

    /// Check if there are any translations
    pub fn is_empty(&self) -> bool {
        self.translations.is_empty()
    }

    /// Get the next available locale id
    fn next_id(&mut self) -> LocaleId {
        let next_id = self.free_id;
        self.free_id = self.free_id.increment();
        next_id
    }

    /// Save all the translations
    pub fn save(&self, dir_tree: &DirectoryTree) -> Result<(), Error> {
        for (lang, file) in LANGUAGE_FILES {
            // Load all translations for this language in a new map
            let mut submap = HashMap::with_capacity(self.translations.capacity());
            for (id, translation) in &self.translations {
                submap.insert(*id, translation.get(*lang));
            }

            // Save the map
            let path = dir_tree.translations().join(file);
            let file = File::create(path)?;
            serde_json::to_writer_pretty(file, &submap)?;
        }
        Ok(())
    }

    /// Get an iterator over all the ids
    pub fn ids(&self) -> impl Iterator<Item = LocaleId> + '_ {
        self.translations.keys().copied()
    }

    /// Get an iterator over all ids and their translations
    pub fn entries(&self) -> impl Iterator<Item = (&LocaleId, &Translation<'_>)> {
        self.translations.iter()
    }
}

impl<'a> Localisation<'a> {
    /// Add a translation to the localisations
    ///
    /// If a matching translation already exists, the existing locale id is returned.
    /// Otherwise a new locale id will be created
    pub fn add_translation<'b: 'a>(&mut self, translation: Translation<'b>) -> LocaleId {
        if self.reverse.contains_key(&translation) {
            // Translation already exists, return existing locale id
            self.reverse
                .get(&translation)
                .copied()
                .unwrap_or_else(|| unreachable!())
        } else if let Some(found) = self
            .reverse
            .keys()
            .find(|k| k.empty_or_equals(&translation))
            .cloned()
        {
            // There's a similar translation, that's just missing translations for specific languages
            // Merge and then return the existing id.
            let id = self.reverse.remove(&found).unwrap();
            let merged = found.merge(translation).unwrap_or_else(|_| unreachable!());
            self.reverse.insert(merged.clone(), id);
            self.translations.insert(id, merged);
            id
        } else {
            let new_id = self.next_id();
            self.reverse.insert(translation.clone(), new_id);
            self.translations.insert(new_id, translation);
            new_id
        }
    }

    /// Initialize Localisation from the game locale.
    ///
    /// This is used when creating a new mod which has no translations saved yet.
    pub fn from_game_locale(translations: HashMap<LocaleId, Translation<'a>>) -> Self {
        let reverse: HashMap<Translation<'_>, LocaleId> =
            translations.iter().map(|(k, v)| (v.clone(), *k)).collect();

        let free_id = translations
            .keys()
            .copied()
            .max()
            .unwrap_or_else(|| unreachable!())
            .increment();

        Self {
            translations,
            reverse,
            free_id,
        }
    }
}

/// Contains all translations for a locale id
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Translation<'a> {
    /// The translations, indexed by [`Language`] as usize
    inner: [Cow<'a, str>; 0x18],
    /// Keep track of which strings are not empty (performance optimisation)
    not_empty: BitArr!(for 0x18, in u8),
}

impl Translation<'_> {
    /// Check if all localisations of this translation are empty
    pub fn is_empty(&self) -> bool {
        self.not_empty.not_any()
    }

    /// Check if the translations overlap, excluding empty translations
    pub fn empty_or_equals(&self, other: &Self) -> bool {
        if self.is_empty() && other.is_empty() {
            // Both are empty
            true
        } else if (self.not_empty & other.not_empty).not_any() {
            // They don't overlap, so equality cannot be checked
            false
        } else {
            // They have some overlap
            for (bit, (left, right)) in (self.not_empty & other.not_empty)
                .iter()
                .zip(self.inner.iter().zip(other.inner.iter()))
            {
                if *bit && left != right {
                    return false;
                }
            }
            true
        }
    }

    /// Get the translation for a `language`
    pub fn get(&self, language: Language) -> &'_ str {
        let index =
            usize::try_from(u32::from(language)).expect("Don't run this on a 16-bit machine!");
        self.inner[index].as_ref()
    }
}

impl<'a: 'c, 'b: 'c, 'c> Translation<'a> {
    /// Merge two translations, rejecting if translations do not match
    pub fn merge(self, other: Translation<'b>) -> Result<Translation<'c>, Error> {
        if self.is_empty() && other.is_empty() {
            // Both are empty
            Ok(self)
        } else if (self.not_empty & other.not_empty).not_any() {
            // They don't overlap, so equality cannot be checked
            Err(anyhow!("No overlap between translations, can't merge!"))
        } else {
            let mut translation = Translation::default();
            // They have some overlap
            for (new, (one, two)) in translation
                .inner
                .iter_mut()
                .zip(self.inner.into_iter().zip(other.inner.into_iter()))
            {
                *new = merge_string(one, two)?;
            }
            translation.not_empty = self.not_empty | other.not_empty;
            Ok(translation)
        }
    }

    /// Add a translation for `language`
    pub fn add_translation<'d: 'a>(&mut self, language: Language, string: Cow<'d, str>) {
        // Ignore a translation if it's empty
        if !string.is_empty() {
            let index =
                usize::try_from(u32::from(language)).expect("Don't run this on a 16-bit machine!");
            if self.not_empty[index] {
                assert!(
                    self.inner[index] == string,
                    "Translation does not match! {} {string}",
                    self.inner[index]
                );
            } else {
                debug_assert!(
                    self.inner[index].is_empty(),
                    "not_empty says the string should be empty, but it's not"
                );
                self.inner[index] = string;
                *self
                    .not_empty
                    .get_mut(index)
                    .unwrap_or_else(|| unreachable!()) = true;
            }
        }
    }
}

/// Merges an empty and non-empty string.
/// If both strings are empty or equal the original string will be returned
///
/// # Error
/// Will error if both strings are non-empty and are not the same
#[inline]
fn merge_string<'a: 'c, 'b: 'c, 'c>(
    original: Cow<'a, str>,
    new: Cow<'b, str>,
) -> Result<Cow<'c, str>, Error> {
    if original == new {
        Ok(original)
    } else if original.is_empty() {
        Ok(new)
    } else if new.is_empty() {
        Ok(original)
    } else {
        Err(anyhow!("{original} does not match {new}"))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::Translation;

    #[test]
    fn test_is_empty() {
        let mut translation = Translation::default();
        assert!(translation.is_empty());
        translation.add_translation(
            ubiart_toolkit::loc8::Language::English,
            Cow::Borrowed("Hello World!"),
        );
        assert!(!translation.is_empty());
    }
}
