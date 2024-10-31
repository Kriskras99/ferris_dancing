//! # Gameconfig
//!
//! Contains types for dealing with anything specified in enginedata/gameconfig folder
use std::sync::atomic::{AtomicU32, Ordering};

pub mod aliases;
pub mod avatars;
pub mod gachacontent;
pub mod objectives;
pub mod playlists;
pub mod portraitborders;
pub mod scheduled_quests;
pub mod search_labels;

/// Contains the last id used for items that could go into a gacha machine
static GACHA_ID: AtomicU32 = AtomicU32::new(0);

/// Generate a new id for content that could go into a gacha machine
///
/// # Panics
/// Will panic if incrementing the id would overflow
pub fn generate_gacha_id() -> u32 {
    // SAFETY: The atomic u16 will make sure every call gets a different value
    let id = GACHA_ID.fetch_add(1, Ordering::SeqCst);
    assert_ne!(
        id,
        u32::from(u16::MAX),
        "Ran out of IDs for aliases, avatars, and portrait borders!"
    );
    id
}
