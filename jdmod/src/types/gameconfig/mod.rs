//! # Gameconfig
//!
//! Contains types for dealing with anything specified in enginedata/gameconfig folder
use std::sync::atomic::{AtomicU16, Ordering};

pub mod aliases;
pub mod avatars;
pub mod gachacontent;
pub mod objectives;
pub mod playlists;
pub mod portraitborders;
pub mod scheduled_quests;
pub mod search_labels;

/// Contains the last id used for items that could go into a gacha machine
static mut GACHA_ID: AtomicU16 = AtomicU16::new(0);

/// Generate a new id for content that could go into a gacha machine
pub fn generate_gacha_id() -> u16 {
    // SAFETY: The atomic u16 will make sure every call gets a different value
    let id = unsafe { GACHA_ID.fetch_add(1, Ordering::SeqCst) };
    assert!(
        id != u16::MAX,
        "Ran out of IDs for aliases, avatars, and portrait borders!"
    );
    id
}
