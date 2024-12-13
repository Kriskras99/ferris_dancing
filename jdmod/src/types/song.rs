//! # Song types
//! Types used to describe songs

use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    hash::Hash,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::vfs::{VirtualPath, VirtualPathBuf};
use hash32::{Hasher, Murmur3Hasher};
use hipstr::HipStr;
use ownable::IntoOwned;
use path_clean::PathClean;
use serde::{Deserialize, Serialize};
use ubiart_toolkit::{
    cooked,
    cooked::tape,
    shared_json_types,
    utils::{LocaleId, Platform},
};

use crate::{regex, utils::hipstr_regex_single_capture};

/// Directory structure of a song
pub struct SongDirectoryTree {
    /// Root song dir
    dir_root: PathBuf,
    /// Contains the msm files
    dir_moves: PathBuf,
    /// Contains the pictos
    dir_pictos: PathBuf,
    /// Contains the menuart
    dir_menuart: PathBuf,
    /// Contains the audio clips
    dir_audio: PathBuf,
    /// File container most metadata
    file_song: PathBuf,
    /// Song name (capitalized)
    song_name: String,
}

impl SongDirectoryTree {
    /// Create a new directory tree from root.
    ///
    /// This does not create directories or check if they exists!
    #[must_use]
    pub fn new(dir_song: &Path, song_name: &str) -> Self {
        let dir_root = dir_song.join(song_name).clean();
        let dir_moves = dir_root.join("moves");
        let dir_pictos = dir_root.join("pictos");
        let dir_menuart = dir_root.join("menuart");
        let dir_audio = dir_root.join("audio");
        Self {
            dir_moves,
            dir_pictos,
            dir_menuart,
            dir_audio,
            file_song: dir_root.join("song.json"),
            song_name: song_name.to_owned(),
            dir_root,
        }
    }

    /// Create the directory tree.
    ///
    /// # Errors
    /// Will error if it fails to create any directory
    pub fn create_dir_all(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.dir_root)
            .and_then(|()| std::fs::create_dir_all(&self.dir_moves))
            .and_then(|()| std::fs::create_dir_all(&self.dir_pictos))
            .and_then(|()| std::fs::create_dir_all(&self.dir_menuart))
            .and_then(|()| std::fs::create_dir_all(&self.dir_audio))
    }

    /// Delete the directory tree and any files it contains.
    ///
    /// # Errors
    /// Will error if it fails to delete all directories.
    pub fn remove_dir_all(&self) -> std::io::Result<()> {
        if self.dir_root.exists() {
            std::fs::remove_dir_all(&self.dir_root)?;
        }
        Ok(())
    }

    /// Check if the directory tree exists.
    #[must_use]
    pub fn exists(&self) -> bool {
        self.dir_root.exists()
            && self.dir_moves.exists()
            && self.dir_pictos.exists()
            && self.dir_menuart.exists()
            && self.dir_audio.exists()
    }

    /// The root of the song directory.
    #[must_use]
    pub fn song(&self) -> &Path {
        &self.dir_root
    }

    /// The main metadata file
    #[must_use]
    pub fn song_file(&self) -> &Path {
        &self.file_song
    }

    /// Used to store the MovementSpace files.
    #[must_use]
    pub fn moves(&self) -> &Path {
        &self.dir_moves
    }

    /// Used to store the pictos.
    #[must_use]
    pub fn pictos(&self) -> &Path {
        &self.dir_pictos
    }

    /// Used to store the pictos.
    #[must_use]
    pub fn menuart(&self) -> &Path {
        &self.dir_menuart
    }

    /// Used to store the pictos.
    #[must_use]
    pub fn audio(&self) -> &Path {
        &self.dir_audio
    }
}

/// Directory structure of a song
pub struct RelativeSongDirectoryTree {
    /// Root song dir
    dir_song: VirtualPathBuf,
    /// Contains the msm files
    dir_song_moves: VirtualPathBuf,
    /// Contains the pictos
    dir_song_pictos: VirtualPathBuf,
    /// Contains the menuart
    dir_song_menuart: VirtualPathBuf,
    /// Contains the audio clips
    dir_song_audio: VirtualPathBuf,
}

impl RelativeSongDirectoryTree {
    /// Create a new directory tree from root.
    ///
    /// This does not create directories or check if they exists!
    #[must_use]
    pub fn new(dir_song: &VirtualPath) -> Self {
        let dir_song = dir_song.to_owned();
        let dir_song_moves = dir_song.join("moves");
        let dir_song_pictos = dir_song.join("pictos");
        let dir_song_menuart = dir_song.join("menuart");
        let dir_song_audio = dir_song.join("audio");
        Self {
            dir_song,
            dir_song_moves,
            dir_song_pictos,
            dir_song_menuart,
            dir_song_audio,
        }
    }

    /// The root of the song directory.
    #[must_use]
    pub fn song(&self) -> &VirtualPath {
        &self.dir_song
    }

    /// Used to store the MovementSpace files.
    #[must_use]
    pub fn moves(&self) -> &VirtualPath {
        &self.dir_song_moves
    }

    /// Used to store the pictos.
    #[must_use]
    pub fn pictos(&self) -> &VirtualPath {
        &self.dir_song_pictos
    }

    /// Used to store the pictos.
    #[must_use]
    pub fn menuart(&self) -> &VirtualPath {
        &self.dir_song_menuart
    }

    /// Used to store the pictos.
    #[must_use]
    pub fn audio(&self) -> &VirtualPath {
        &self.dir_song_audio
    }
}

/// Number of coaches for this song
#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, IntoOwned)]
pub enum NumberOfCoaches {
    /// One coach
    Solo = 1,
    /// Two coaches
    Duo = 2,
    /// Three coaches
    Trio = 3,
    /// Four coaches
    Quarto = 4,
    /// Six coaches (unused)
    Sextet = 6,
}

impl TryFrom<u32> for NumberOfCoaches {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Solo),
            2 => Ok(Self::Duo),
            3 => Ok(Self::Trio),
            4 => Ok(Self::Quarto),
            6 => Ok(Self::Sextet),
            _ => Err(anyhow!("Unknown NumberOfCoaches! {}", value)),
        }
    }
}

impl From<NumberOfCoaches> for u32 {
    #[allow(clippy::as_conversions, reason = "Is repr(Self)")]
    fn from(value: NumberOfCoaches) -> Self {
        value as Self
    }
}

/// Main metadata about the song
#[derive(Serialize, Deserialize, Clone, IntoOwned)]
pub struct Song<'a> {
    /// Codename for the song (Capitalised)
    #[serde(borrow)]
    pub map_name: HipStr<'a>,
    /// Original Just Dance version
    pub original_jd_version: u32,
    /// Artist
    #[serde(borrow)]
    pub artist: HipStr<'a>,
    /// Coach name
    #[serde(borrow)]
    pub dancer_name: HipStr<'a>,
    /// Song name
    #[serde(borrow)]
    pub title: HipStr<'a>,
    /// Writing credits
    #[serde(borrow)]
    pub credits: HipStr<'a>,
    /// Number of coaches
    pub number_of_coaches: NumberOfCoaches,
    /// Which of the coaches is the main coach? None if only one coach
    pub main_coach: Option<u32>,
    /// Difficulty of the song
    pub difficulty: Difficulty,
    /// Intensity of the song
    pub sweat_difficulty: SweatDifficulty,
    /// Related songs (other difficulties, covers)
    #[serde(borrow)]
    pub related_songs: Vec<HipStr<'a>>,
    /// How is the song unlocked
    pub status: MapStatus,
    /// Tags related to this song
    pub tags: Vec<HipStr<'a>>,
    /// Subtitle locale id (like: Extreme Version)
    pub subtitle: LocaleId,
    /// Theme colors of the song
    pub default_colors: SongColors,
    /// The audio file for the song
    #[serde(borrow)]
    pub audiofile: HipStr<'a>,
    /// The videofile for the song
    #[serde(borrow)]
    pub videofile: HipStr<'a>,
}

/// Image used in the menus
#[derive(Serialize, Deserialize, Clone)]
pub enum MenuArt<'a> {
    /// Image for the game itself
    #[serde(borrow)]
    Texture(MenuArtTexture<'a>),
    /// Image for a phone controller
    #[serde(borrow)]
    Phone(PhoneImage<'a>),
}

/// Texture used by the game
#[derive(Serialize, Deserialize, Clone)]
pub struct MenuArtTexture<'a> {
    /// Userfriendly name
    #[serde(borrow)]
    pub name: HipStr<'a>,
    /// Filename
    #[serde(borrow)]
    pub filename: HipStr<'a>,
    /// Scale as used in [`cooked::isc::MaterialGraphicComponent`]
    pub scale: (f32, f32),
    /// 2d position as used in [`cooked::isc::MaterialGraphicComponent`]
    pub pos2d: (f32, f32),
    /// Disable shadow as used in [`cooked::isc::MaterialGraphicComponent`]
    pub disable_shadow: u32,
    /// Unknown? Used in [`cooked::isc::MaterialGraphicComponent`]
    pub anchor: i32,
}

/// Image used for the phone application
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhoneImage<'a> {
    /// Filename
    #[serde(borrow)]
    pub filename: HipStr<'a>,
    /// Userfriendly name
    #[serde(borrow)]
    pub name: HipStr<'a>,
}

/// Unknown
#[derive(Serialize, Deserialize, Clone)]
pub struct Record {
    /// Start of the record?
    pub start: f32,
    /// Duration of the record?
    pub duration: f32,
}

impl From<&Record> for cooked::tpl::types::Record<'static> {
    fn from(value: &Record) -> Self {
        Self {
            class: Some(Self::CLASS),
            start: value.start,
            duration: value.duration,
        }
    }
}

impl From<&cooked::tpl::types::Record<'_>> for Record {
    fn from(value: &cooked::tpl::types::Record) -> Self {
        Self {
            start: value.start,
            duration: value.duration,
        }
    }
}

/// Unknown
#[derive(Serialize, Deserialize, Clone)]
pub struct PlaybackEvent {
    /// Clip to play?
    pub clip_number: u32,
    /// Start time in the clip?
    pub start_clip: f32,
    /// Start time of the event?
    pub start_time: f32,
    /// Duration of the event?
    pub duration: f32,
    /// Playback speed?
    pub speed: f32,
}

impl From<&PlaybackEvent> for shared_json_types::PlaybackEvent<'static> {
    fn from(value: &PlaybackEvent) -> Self {
        Self {
            class: Some(Self::CLASS),
            clip_number: value.clip_number,
            start_clip: value.start_clip,
            start_time: value.start_time,
            duration: value.duration,
            speed: value.speed,
        }
    }
}

impl From<&shared_json_types::PlaybackEvent<'_>> for PlaybackEvent {
    fn from(value: &shared_json_types::PlaybackEvent) -> Self {
        Self {
            clip_number: value.clip_number,
            start_clip: value.start_clip,
            start_time: value.start_time,
            duration: value.duration,
            speed: value.speed,
        }
    }
}

/// A RGBA8 color
#[derive(
    Hash, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, IntoOwned,
)]
pub struct Color {
    /// Transparency, 0 is fully transparent, 255 is fully opaque
    pub transparency: u8,
    /// Red
    pub red: u8,
    /// Green
    pub green: u8,
    /// Blue
    pub blue: u8,
}

impl From<&Color> for ubiart_toolkit::utils::Color {
    fn from(color: &Color) -> Self {
        Self {
            color: (
                f32::from(color.transparency) / 255.0,
                f32::from(color.red) / 255.0,
                f32::from(color.green) / 255.0,
                f32::from(color.blue) / 255.0,
            ),
        }
    }
}

impl From<&ubiart_toolkit::utils::Color> for Color {
    fn from(value: &ubiart_toolkit::utils::Color) -> Self {
        Self {
            transparency: map_range_to_u8(value.color.0, 0.0, 1.0),
            red: map_range_to_u8(value.color.1, 0.0, 1.0),
            green: map_range_to_u8(value.color.2, 0.0, 1.0),
            blue: map_range_to_u8(value.color.3, 0.0, 1.0),
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        Self {
            transparency: u8::from_str_radix(&value[2..4], 16).unwrap_or(255),
            red: u8::from_str_radix(&value[4..6], 16).unwrap_or(255),
            green: u8::from_str_radix(&value[6..8], 16).unwrap_or(255),
            blue: u8::from_str_radix(&value[8..10], 16).unwrap_or(255),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            transparency: 255,
            red: 255,
            green: 255,
            blue: 255,
        }
    }
}

/// Map a `value` in range `min` to `max` to a u8
///
/// # Panics
/// Will panic if `max` >= `min`
#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::as_conversions,
    reason = "Safe because of the maths"
)]
fn map_range_to_u8(mut value: f32, min: f32, max: f32) -> u8 {
    assert!(max >= min, "Range is not sane! {min} {max}");
    // Check if the range is zero
    let range = max - min;
    if range == 0.0 {
        0
    } else {
        // Make sure the value falls into the range
        value = value.clamp(min, max);
        // move value down to zero..range, then map it to 255
        let new_value = (((value - min) * 255.0) / range).round();
        new_value as u8
    }
}

/// Theme colors of the song
#[allow(clippy::module_name_repetitions, reason = "It's a good name")]
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, IntoOwned)]
pub struct SongColors {
    /// Main theme
    pub theme: Color,
    /// Lyrics color
    pub lyrics: Color,
    /// Accent 1A
    pub one_a: Color,
    /// Accent 1B
    pub one_b: Color,
    /// Accent 2A
    pub two_a: Color,
    /// Accent 2B
    pub two_b: Color,
}

impl From<&SongColors> for cooked::tpl::types::DefaultColors {
    fn from(colors: &SongColors) -> Self {
        Self {
            theme: (&colors.theme).into(),
            lyrics: (&colors.lyrics).into(),
            songcolor_1a: Some((&colors.one_a).into()),
            songcolor_1b: Some((&colors.one_b).into()),
            songcolor_2a: Some((&colors.two_a).into()),
            songcolor_2b: Some((&colors.two_b).into()),
        }
    }
}

impl From<&cooked::tpl::types::DefaultColors> for SongColors {
    fn from(value: &cooked::tpl::types::DefaultColors) -> Self {
        Self {
            theme: (&value.theme).into(),
            lyrics: (&value.lyrics).into(),
            one_a: value.songcolor_1a.as_ref().unwrap_or(&value.theme).into(),
            one_b: value.songcolor_1b.as_ref().unwrap_or(&value.theme).into(),
            two_a: value.songcolor_2a.as_ref().unwrap_or(&value.theme).into(),
            two_b: value.songcolor_2b.as_ref().unwrap_or(&value.theme).into(),
        }
    }
}

impl From<&HashMap<HipStr<'_>, HipStr<'_>>> for SongColors {
    fn from(value: &HashMap<HipStr<'_>, HipStr<'_>>) -> Self {
        let theme = value
            .get("theme")
            .map(|s| Color::from(s.as_str()))
            .unwrap_or_default();
        let lyrics = value
            .get("lyrics")
            .map(|s| Color::from(s.as_str()))
            .unwrap_or_default();
        let one_a = value
            .get("songColor_1A")
            .map(|s| Color::from(s.as_str()))
            .unwrap_or_default();
        let one_b = value
            .get("songColor_1B")
            .map(|s| Color::from(s.as_str()))
            .unwrap_or_default();
        let two_a = value
            .get("songColor_2A")
            .map(|s| Color::from(s.as_str()))
            .unwrap_or_default();
        let two_b = value
            .get("songColor_2B")
            .map(|s| Color::from(s.as_str()))
            .unwrap_or_default();

        Self {
            theme,
            lyrics,
            one_a,
            one_b,
            two_a,
            two_b,
        }
    }
}

/// How is this map unlocked
#[repr(u8)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, IntoOwned)]
pub enum MapStatus {
    /// Probably DLC?
    Unknown0 = 0,
    /// Unknown
    Unknown1 = 1,
    /// Buy it with mojo (JD2018 and earler)
    BuyWithMojo = 2,
    /// Unlocked by default
    Unlocked = 3,
    /// Unlocked with a code (JD2020)
    CodeUnlockable = 4,
    /// Unknown
    Unknown5 = 5,
    /// Unknown
    Unknown6 = 6,
    /// Won from the gift (gacha) machine
    GiftMachine = 9,
    /// Score superstar on the related map (JD2018 and earlier)
    ScoreSuperstar = 10,
    /// Complete a quest
    Quest = 12,
    /// Complete the anthology (JD2022)
    Anthology = 13,
}

impl From<MapStatus> for u32 {
    #[allow(clippy::as_conversions, reason = "Is repr(Self)")]
    fn from(value: MapStatus) -> Self {
        value as Self
    }
}

impl MapStatus {
    /// Normalize the map status to those used by JD2022
    #[must_use]
    pub const fn normalize(self) -> Self {
        match self {
            Self::Quest => Self::Quest,
            _ => Self::Unlocked,
        }
    }
}

impl TryFrom<u32> for MapStatus {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown0),
            1 => Ok(Self::Unknown1),
            2 => Ok(Self::BuyWithMojo),
            3 => Ok(Self::Unlocked),
            4 => Ok(Self::CodeUnlockable),
            5 => Ok(Self::Unknown5),
            6 => Ok(Self::Unknown6),
            9 => Ok(Self::GiftMachine),
            10 => Ok(Self::ScoreSuperstar),
            12 => Ok(Self::Quest),
            13 => Ok(Self::Anthology),
            _ => Err(anyhow!("Unknown MapType! {}", value)),
        }
    }
}

/// Difficulty of the song
#[repr(u8)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, IntoOwned)]
pub enum Difficulty {
    /// Easy
    Easy = 1,
    /// Medium
    Medium = 2,
    /// Hard
    Hard = 3,
    /// Extreme
    Extreme = 4,
}

impl From<Difficulty> for u32 {
    #[allow(clippy::as_conversions, reason = "Is repr(Self)")]
    fn from(value: Difficulty) -> Self {
        value as Self
    }
}

impl TryFrom<u32> for Difficulty {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Easy),
            2 => Ok(Self::Medium),
            3 => Ok(Self::Hard),
            4 => Ok(Self::Extreme),
            _ => Err(anyhow!("Unknown Difficulty! {}", value)),
        }
    }
}

/// Intensity of the song
#[repr(u8)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, IntoOwned)]
pub enum SweatDifficulty {
    /// Low intensity
    Low = 1,
    /// Medium intensity
    Moderate = 2,
    /// High intensity
    Intense = 3,
}

impl From<SweatDifficulty> for u32 {
    #[allow(clippy::as_conversions, reason = "Is repr(Self)")]
    fn from(value: SweatDifficulty) -> Self {
        value as Self
    }
}

impl TryFrom<u32> for SweatDifficulty {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Low),
            2 => Ok(Self::Moderate),
            3 => Ok(Self::Intense),
            _ => Err(anyhow!("Unknown SweatDifficulty! {}", value)),
        }
    }
}

/// Describes the music
#[derive(Serialize, Deserialize, Clone)]
pub struct MusicTrack {
    /// Start of the audio track
    pub start_beat: i32,
    /// End of the audio track
    pub end_beat: u32,
    /// Start of the video
    pub video_start_time: f32,
    /// Unknown
    pub preview_entry: u32,
    /// Preview audio track start
    pub preview_loop_start: u32,
    /// Preview audio track end
    pub preview_loop_end: u32,
    /// Unknown
    pub signatures: Vec<Signature>,
    /// Unknown
    pub sections: Vec<Section>,
    /// Unknown
    pub markers: Vec<u32>,
}

/// Unknown
#[derive(Serialize, Deserialize, Clone)]
pub struct Signature {
    /// Unknown
    pub marker: i32,
    /// Unknown
    pub beats: u32,
}

impl From<Signature> for cooked::tpl::types::MusicSignature<'static> {
    fn from(value: Signature) -> Self {
        Self {
            class: Some(Self::CLASS),
            marker: value.marker,
            beats: value.beats,
            comment: None,
        }
    }
}

impl From<cooked::tpl::types::MusicSignature<'_>> for Signature {
    fn from(value: cooked::tpl::types::MusicSignature) -> Self {
        Self {
            marker: value.marker,
            beats: value.beats,
        }
    }
}

/// Unknown
#[derive(Serialize, Deserialize, Clone)]
pub struct Section {
    /// Unknown
    pub marker: i32,
    /// Unknown
    pub section_type: u32,
}

impl From<Section> for cooked::tpl::types::MusicSection<'static> {
    fn from(value: Section) -> Self {
        Self {
            class: Some(Self::CLASS),
            marker: value.marker,
            section_type: value.section_type,
            comment: HipStr::borrowed(""),
        }
    }
}

impl From<cooked::tpl::types::MusicSection<'_>> for Section {
    fn from(value: cooked::tpl::types::MusicSection) -> Self {
        Self {
            marker: value.marker,
            section_type: value.section_type,
        }
    }
}

/// Everything that happens during a song
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Timeline<'a> {
    /// The events in chronological order
    #[serde(borrow)]
    pub timeline: BTreeSet<Clip<'a>>,
}

/// A event that happens during a song
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Clip<'a> {
    /// Unknown
    Alpha(AlphaClip),
    /// Unknown
    Color(ColorClip),
    /// Unknown
    #[serde(borrow)]
    GameplayEvent(GameplayEventClip<'a>),
    /// Gold move effect
    GoldEffect(GoldEffectClip),
    /// Hide user interface
    HideUserInterface(HideUserInterfaceClip),
    /// Show lyric
    #[serde(borrow)]
    Karaoke(KaraokeClip<'a>),
    /// Unknown
    MaterialGraphicEnableLayer(MaterialGraphicEnableLayerClip),
    /// Grade dance move
    #[serde(borrow)]
    Motion(MotionClip<'a>),
    /// Show picto
    #[serde(borrow)]
    Pictogram(PictogramClip<'a>),
    /// Unknown
    Proportion(ProportionClip),
    /// Unknown
    Rotation(RotationClip),
    /// Play audio sample
    #[serde(borrow)]
    SoundSet(SoundSetClip<'a>),
    /// Unknown
    Translation(TranslationClip),
    /// Vibrate the controller
    #[serde(borrow)]
    Vibration(VibrationClip<'a>),
}

impl Ord for Clip<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by start time
        let ord_start_time = self.start_time().cmp(&other.start_time());
        if ord_start_time != Ordering::Equal {
            return ord_start_time;
        }
        // Sort longer duration events first
        let ord_duration = other.duration().cmp(&self.duration());
        if ord_duration != Ordering::Equal {
            return ord_duration;
        }
        match (self, other) {
            (Clip::Alpha(left), Clip::Alpha(right)) => left.cmp(right),
            (Clip::Color(left), Clip::Color(right)) => left.cmp(right),
            (Clip::GameplayEvent(left), Clip::GameplayEvent(right)) => left.cmp(right),
            (Clip::GoldEffect(left), Clip::GoldEffect(right)) => left.cmp(right),
            (Clip::HideUserInterface(left), Clip::HideUserInterface(right)) => left.cmp(right),
            (Clip::Karaoke(left), Clip::Karaoke(right)) => left.cmp(right),
            (Clip::MaterialGraphicEnableLayer(left), Clip::MaterialGraphicEnableLayer(right)) => {
                left.cmp(right)
            }
            (Clip::Motion(left), Clip::Motion(right)) => left.cmp(right),
            (Clip::Pictogram(left), Clip::Pictogram(right)) => left.cmp(right),
            (Clip::Proportion(left), Clip::Proportion(right)) => left.cmp(right),
            (Clip::Rotation(left), Clip::Rotation(right)) => left.cmp(right),
            (Clip::SoundSet(left), Clip::SoundSet(right)) => left.cmp(right),
            (Clip::Translation(left), Clip::Translation(right)) => left.cmp(right),
            (Clip::Vibration(left), Clip::Vibration(right)) => left.cmp(right),
            (Clip::GoldEffect(_), _) => Ordering::Less,
            (_, Clip::GoldEffect(_)) => Ordering::Greater,
            (Clip::Motion(_), _) => Ordering::Less,
            (_, Clip::Motion(_)) => Ordering::Greater,
            (Clip::Pictogram(_), _) => Ordering::Less,
            (_, Clip::Pictogram(_)) => Ordering::Greater,
            (Clip::Karaoke(_), _) => Ordering::Less,
            (_, Clip::Karaoke(_)) => Ordering::Greater,
            (Clip::HideUserInterface(_), _) => Ordering::Less,
            (_, Clip::HideUserInterface(_)) => Ordering::Greater,
            (Clip::SoundSet(_), _) => Ordering::Less,
            (_, Clip::SoundSet(_)) => Ordering::Greater,
            (Clip::Vibration(_), _) => Ordering::Less,
            (_, Clip::Vibration(_)) => Ordering::Greater,
            (Clip::GameplayEvent(_), _) => Ordering::Less,
            (_, Clip::GameplayEvent(_)) => Ordering::Greater,
            (Clip::Color(_), _) => Ordering::Less,
            (_, Clip::Color(_)) => Ordering::Greater,
            (Clip::Alpha(_), _) => Ordering::Less,
            (_, Clip::Alpha(_)) => Ordering::Greater,
            (Clip::MaterialGraphicEnableLayer(_), _) => Ordering::Less,
            (_, Clip::MaterialGraphicEnableLayer(_)) => Ordering::Greater,
            (Clip::Proportion(_), _) => Ordering::Less,
            (_, Clip::Proportion(_)) => Ordering::Greater,
            (Clip::Rotation(_), _) => Ordering::Less,
            (_, Clip::Rotation(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Clip<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Clip<'a> {
    /// Convert this clip to the UbiArt representation
    ///
    /// Will not work for [`Clip::SoundSet`].
    ///
    /// # Errors
    /// Will return an error if song is a [`Clip::SoundSet`]
    pub fn into_tape(self, song: &Song) -> Result<tape::Clip<'a>, Error> {
        match self {
            Self::SoundSet(_) => Err(anyhow!(
                "Converting SoundSetClip through the Clip enum is not supported!"
            )),
            Self::Alpha(data) => Ok(tape::Clip::Alpha(data.into())),
            Self::Color(data) => Ok(tape::Clip::Color(data.into())),
            Self::GameplayEvent(data) => Ok(tape::Clip::GameplayEvent(data.into())),
            Self::GoldEffect(data) => Ok(tape::Clip::GoldEffect(data.into())),
            Self::HideUserInterface(data) => Ok(tape::Clip::HideUserInterface(data.into())),
            Self::Karaoke(data) => Ok(tape::Clip::Karaoke(data.into())),
            Self::MaterialGraphicEnableLayer(data) => {
                Ok(tape::Clip::MaterialGraphicEnableLayer(data.into()))
            }
            Self::Motion(data) => Ok(tape::Clip::Motion(data.to_tape(song))),
            Self::Pictogram(data) => Ok(tape::Clip::Pictogram(data.to_tape(song))),
            Self::Proportion(data) => Ok(tape::Clip::Proportion(data.into())),
            Self::Rotation(data) => Ok(tape::Clip::Rotation(data.into())),
            Self::Translation(data) => Ok(tape::Clip::Translation(data.into())),
            Self::Vibration(data) => Ok(tape::Clip::Vibration(data.into())),
        }
    }

    /// The start time of a clip
    #[must_use]
    pub const fn start_time(&self) -> i32 {
        match self {
            Clip::Alpha(data) => data.start_time,
            Clip::Color(data) => data.start_time,
            Clip::GameplayEvent(data) => data.start_time,
            Clip::GoldEffect(data) => data.start_time,
            Clip::HideUserInterface(data) => data.start_time,
            Clip::Karaoke(data) => data.start_time,
            Clip::MaterialGraphicEnableLayer(data) => data.start_time,
            Clip::Motion(data) => data.start_time,
            Clip::Pictogram(data) => data.start_time,
            Clip::Proportion(data) => data.start_time,
            Clip::Rotation(data) => data.start_time,
            Clip::SoundSet(data) => data.start_time,
            Clip::Translation(data) => data.start_time,
            Clip::Vibration(data) => data.start_time,
        }
    }

    /// The duration of a clip
    #[must_use]
    pub const fn duration(&self) -> i32 {
        match self {
            Clip::Alpha(data) => data.duration,
            Clip::Color(data) => data.duration,
            Clip::GameplayEvent(data) => data.duration,
            Clip::GoldEffect(data) => data.duration,
            Clip::HideUserInterface(data) => data.duration,
            Clip::Karaoke(data) => data.duration,
            Clip::MaterialGraphicEnableLayer(data) => data.duration,
            Clip::Motion(data) => data.duration,
            Clip::Pictogram(data) => data.duration,
            Clip::Proportion(data) => data.duration,
            Clip::Rotation(data) => data.duration,
            Clip::SoundSet(data) => data.duration,
            Clip::Translation(data) => data.duration,
            Clip::Vibration(data) => data.duration,
        }
    }
}

impl<'a> TryFrom<tape::Clip<'a>> for Clip<'a> {
    type Error = Error;

    fn try_from(value: tape::Clip<'a>) -> Result<Self, Self::Error> {
        match value {
            tape::Clip::Alpha(data) => Ok(Self::Alpha(data.into())),
            tape::Clip::Color(data) => Ok(Self::Color(data.into())),
            tape::Clip::GameplayEvent(data) => Ok(Self::GameplayEvent(data.into())),
            tape::Clip::GoldEffect(data) => Ok(Self::GoldEffect(data.into())),
            tape::Clip::HideUserInterface(data) => Ok(Self::HideUserInterface(data.into())),
            tape::Clip::Karaoke(data) => Ok(Self::Karaoke(data.into())),
            tape::Clip::MaterialGraphicEnableLayer(data) => {
                Ok(Self::MaterialGraphicEnableLayer(data.into()))
            }
            tape::Clip::Motion(data) => Ok(Self::Motion(data.try_into()?)),
            tape::Clip::Pictogram(data) => Ok(Self::Pictogram(data.try_into()?)),
            tape::Clip::Proportion(data) => Ok(Self::Proportion(data.into())),
            tape::Clip::Rotation(data) => Ok(Self::Rotation(data.into())),
            tape::Clip::Translation(data) => Ok(Self::Translation(data.into())),
            tape::Clip::Vibration(data) => Ok(Self::Vibration(data.into())),
            _ => Err(anyhow!("Unsupported clip: {value:?}")),
        }
    }
}

/// Unknown
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AlphaClip {
    /// Is the clip active
    pub is_active: bool,
    /// When this clip starts
    pub start_time: i32,
    /// Duration of the clip
    pub duration: i32,
    /// Unknown
    pub actor_indices: Vec<u8>,
    /// Unknown
    pub curve: Option<CurveFloat>,
}

impl From<AlphaClip> for tape::AlphaClip<'static> {
    fn from(value: AlphaClip) -> Self {
        let mut hasher = Murmur3Hasher::default();
        value.hash(&mut hasher);
        let id = hasher.finish32();

        Self {
            class: None,
            id,
            track_id: 4_094_799_440,
            is_active: u8::from(value.is_active),
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            target_actors: Vec::new(),
            curve: value
                .curve
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
        }
    }
}

impl From<tape::AlphaClip<'_>> for AlphaClip {
    fn from(value: tape::AlphaClip<'_>) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            curve: CurveFloat::from_curve(&value.curve),
        }
    }
}

/// Set an actor to a color?
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColorClip {
    /// Is the clip active
    pub is_active: bool,
    /// When to start the vibration
    pub start_time: i32,
    /// Duration of the vibration
    pub duration: i32,
    /// The actors to color
    pub actor_indices: Vec<u8>,
    /// Red color curve
    pub curve_red: Option<CurveFloat>,
    /// Red color curve
    pub curve_green: Option<CurveFloat>,
    /// Red color curve
    pub curve_blue: Option<CurveFloat>,
}

impl From<ColorClip> for tape::ColorClip<'static> {
    fn from(value: ColorClip) -> Self {
        let mut hasher = Murmur3Hasher::default();
        value.hash(&mut hasher);
        let id = hasher.finish32();
        Self {
            class: None,
            id,
            track_id: 1_369_275_280,
            is_active: u8::from(value.is_active),
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            target_actors: Vec::new(),
            curve_red: value
                .curve_red
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
            curve_green: value
                .curve_green
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
            curve_blue: value
                .curve_blue
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
        }
    }
}

impl From<tape::ColorClip<'_>> for ColorClip {
    fn from(value: tape::ColorClip<'_>) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            curve_red: CurveFloat::from_curve(&value.curve_red),
            curve_green: CurveFloat::from_curve(&value.curve_green),
            curve_blue: CurveFloat::from_curve(&value.curve_blue),
        }
    }
}

/// Unknown
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameplayEventClip<'a> {
    /// Is the clip active
    pub is_active: bool,
    /// When this clip starts
    pub start_time: i32,
    /// Duration of the clip
    pub duration: i32,
    /// Unknown
    pub actor_indices: Vec<u8>,
    /// Unknown
    pub event_type: u32,
    /// Unknown
    #[serde(borrow)]
    pub custom_param: HipStr<'a>,
}

impl<'a> From<GameplayEventClip<'a>> for tape::GameplayEventClip<'a> {
    fn from(value: GameplayEventClip<'a>) -> Self {
        let mut hasher = Murmur3Hasher::default();
        value.hash(&mut hasher);
        let id = hasher.finish32();

        Self {
            class: None,
            id,
            track_id: 4_094_799_440,
            is_active: u8::from(value.is_active),
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            target_actors: Vec::new(),
            event_type: value.event_type,
            custom_param: value.custom_param,
        }
    }
}

impl<'a> From<tape::GameplayEventClip<'a>> for GameplayEventClip<'a> {
    fn from(value: tape::GameplayEventClip<'a>) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            event_type: value.event_type,
            custom_param: value.custom_param,
        }
    }
}

/// Show the gold move effect
#[derive(Hash, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GoldEffectClip {
    /// Is the clip active
    pub is_active: bool,
    /// When to show the gold move effect
    pub start_time: i32,
    /// Duration to show the gold move effect for
    pub duration: i32,
    /// Unknown
    pub effect_type: u8,
}

impl From<GoldEffectClip> for tape::GoldEffectClip<'static> {
    fn from(value: GoldEffectClip) -> Self {
        let mut hasher = Murmur3Hasher::default();
        value.hash(&mut hasher);
        let id = hasher.finish32();
        Self {
            class: None,
            id,
            track_id: 1_369_275_280,
            is_active: u8::from(value.is_active),
            start_time: value.start_time,
            duration: value.duration,
            effect_type: value.effect_type,
        }
    }
}

impl From<tape::GoldEffectClip<'_>> for GoldEffectClip {
    fn from(value: tape::GoldEffectClip) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            effect_type: value.effect_type,
        }
    }
}

/// Clip to hide the user interface
#[derive(Hash, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HideUserInterfaceClip {
    /// Is the clip active
    pub is_active: bool,
    /// When to hide the user interface
    pub start_time: i32,
    /// Duration to hide the user interface for
    pub duration: i32,
    /// Unknown
    pub event_type: u32,
}

impl From<HideUserInterfaceClip> for tape::HideUserInterfaceClip<'static> {
    fn from(value: HideUserInterfaceClip) -> Self {
        let mut hasher = Murmur3Hasher::default();
        value.hash(&mut hasher);
        let id = hasher.finish32();
        Self {
            class: None,
            id,
            track_id: 1_028_802_763,
            start_time: value.start_time,
            duration: value.duration,
            event_type: value.event_type,
            custom_param: HipStr::new(),
            is_active: u8::from(value.is_active),
            actor_indices: Vec::new(),
            target_actors: Vec::new(),
        }
    }
}

impl From<tape::HideUserInterfaceClip<'_>> for HideUserInterfaceClip {
    fn from(value: tape::HideUserInterfaceClip) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            event_type: value.event_type,
        }
    }
}

/// Display a lyric
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KaraokeClip<'a> {
    /// Is the clip active
    pub is_active: bool,
    /// When to show the lyric
    pub start_time: i32,
    /// Duration to show the lyric for
    pub duration: i32,
    /// Expected pitch of the lyric (for use with microphone enabled systems)
    pub pitch: f32,
    /// The lyric
    #[serde(borrow)]
    pub lyrics: HipStr<'a>,
    /// Should the next lyric be on a new line
    pub is_end_of_line: bool,
    /// Unknown
    pub content_type: u32,
}

impl PartialEq for KaraokeClip<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.is_active == other.is_active
            && self.start_time == other.start_time
            && self.duration == other.duration
            && self.pitch.total_cmp(&other.pitch) == Ordering::Equal
            && self.lyrics == other.lyrics
            && self.is_end_of_line == other.is_end_of_line
    }
}

impl Eq for KaraokeClip<'_> {}

impl Ord for KaraokeClip<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.is_active
            .cmp(&other.is_active)
            .then(self.start_time.cmp(&other.start_time))
            .then(self.duration.cmp(&self.duration))
            .then(self.pitch.total_cmp(&other.pitch))
            .then(self.lyrics.cmp(&other.lyrics))
            .then(self.is_end_of_line.cmp(&other.is_end_of_line))
    }
}

impl PartialOrd for KaraokeClip<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for KaraokeClip<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.is_active.hash(state);
        self.start_time.hash(state);
        self.duration.hash(state);
        self.pitch.to_bits().hash(state);
        self.lyrics.hash(state);
        self.is_end_of_line.hash(state);
    }
}

impl<'a> From<KaraokeClip<'a>> for tape::KaraokeClip<'a> {
    fn from(value: KaraokeClip<'a>) -> Self {
        let mut hasher = Murmur3Hasher::default();
        value.hash(&mut hasher);
        let id = hasher.finish32();
        Self {
            class: None,
            id,
            track_id: 0,
            is_active: u8::from(value.is_active),
            start_time: value.start_time,
            duration: value.duration,
            pitch: value.pitch,
            lyrics: value.lyrics,
            is_end_of_line: u8::from(value.is_end_of_line),
            content_type: value.content_type,
            start_time_tolerance: 4,
            end_time_tolerance: 4,
            semitone_tolerance: 5.0,
        }
    }
}

impl<'a> From<tape::KaraokeClip<'a>> for KaraokeClip<'a> {
    fn from(value: tape::KaraokeClip<'a>) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            pitch: value.pitch,
            lyrics: value.lyrics,
            is_end_of_line: value.is_end_of_line == 1,
            content_type: value.content_type,
        }
    }
}

/// Unknown
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaterialGraphicEnableLayerClip {
    /// Is the clip active
    pub is_active: bool,
    /// When this clip starts
    pub start_time: i32,
    /// Duration of the clip
    pub duration: i32,
    /// Unknown
    pub actor_indices: Vec<u8>,
    /// Unknown
    pub layer_idx: u32,
    /// Unknown
    pub uv_modifier_idx: u32,
    /// Unknown
    pub layer_enabled: bool,
}

impl From<MaterialGraphicEnableLayerClip> for tape::MaterialGraphicEnableLayerClip<'static> {
    fn from(value: MaterialGraphicEnableLayerClip) -> Self {
        let mut hasher = Murmur3Hasher::default();
        value.hash(&mut hasher);
        let id = hasher.finish32();

        Self {
            class: None,
            id,
            track_id: 4_094_799_440,
            is_active: u8::from(value.is_active),
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            target_actors: Vec::new(),
            layer_idx: value.layer_idx,
            uv_modifier_idx: value.uv_modifier_idx,
            layer_enabled: u8::from(value.layer_enabled),
        }
    }
}

impl From<tape::MaterialGraphicEnableLayerClip<'_>> for MaterialGraphicEnableLayerClip {
    fn from(value: tape::MaterialGraphicEnableLayerClip<'_>) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            layer_idx: value.layer_idx,
            uv_modifier_idx: value.uv_modifier_idx,
            layer_enabled: value.layer_enabled == 1,
        }
    }
}

/// Provide movement space check
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MotionClip<'a> {
    /// Is the clip active
    pub is_active: bool,
    /// When to the move start
    pub start_time: i32,
    /// Duration of the move
    pub duration: i32,
    /// The classifier (.msm file for NX/WiiU/Phone)
    #[serde(borrow)]
    pub classifier_filename: HipStr<'a>,
    /// Is this a gold move
    pub gold_move: bool,
    /// Which coach this move is tracking
    pub coach_id: u8,
    /// The color of something?
    pub color: Color,
}

impl MotionClip<'_> {
    /// Convert this clip to the UbiArt representation
    #[must_use]
    pub fn to_tape(&self, song: &Song) -> tape::MotionClip<'static> {
        let mut hasher = Murmur3Hasher::default();
        self.hash(&mut hasher);
        let id = hasher.finish32();

        let lower_map_name = song.map_name.to_lowercase();
        let filename = &self.classifier_filename;

        tape::MotionClip {
            class: None,
            id,
            track_id: 4_094_799_440,
            is_active: u8::from(self.is_active),
            start_time: self.start_time,
            duration: self.duration,
            classifier_path: HipStr::from(format!(
                "world/maps/{lower_map_name}/timeline/moves/{lower_map_name}_{filename}"
            )),
            gold_move: u8::from(self.gold_move),
            coach_id: self.coach_id,
            move_type: 0,
            color: (&self.color).into(),
            motion_platform_specifics: HashMap::new(),
        }
    }

    /// The classifier path needs to be changed to include the /wiiu/ component
    ///
    /// # Errors
    /// Will return an error if the platform is not supported or the path is broken
    pub fn fix_classifier_path(classifier_path: &str, platform: Platform) -> Result<String, Error> {
        // Classifier path does not include platform specifier
        let index = classifier_path
            .rfind('/')
            .ok_or_else(|| anyhow!("Weird classifier path"))?;
        let (left, right) = classifier_path.split_at(index);
        let mut classifier_path = String::with_capacity(classifier_path.len() + 5);
        classifier_path.push_str(left);
        match (right.ends_with(".gesture"), platform) {
            (false, Platform::Nx | Platform::WiiU | Platform::Win) => {
                classifier_path.push_str("/wiiu");
            }
            (true, _) => classifier_path.push_str("/orbis"),
            _ => unimplemented!("Not implemented for {}", platform),
        }
        classifier_path.push_str(right);
        Ok(classifier_path)
    }
}

impl<'a> TryFrom<tape::MotionClip<'a>> for MotionClip<'a> {
    type Error = Error;

    fn try_from(value: tape::MotionClip<'a>) -> Result<Self, Self::Error> {
        let class = value.classifier_path.as_str();
        let classifier_filename =
            HipStr::from(class.rsplit_once('/').unwrap_or(("", class)).1.to_string());

        Ok(Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            classifier_filename,
            gold_move: value.gold_move == 1,
            coach_id: value.coach_id,
            color: (&value.color).into(),
        })
    }
}

/// Show a pictogram
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PictogramClip<'a> {
    /// Is the clip active
    pub is_active: bool,
    /// When to show the picto
    pub start_time: i32,
    /// Duration to show the picto for
    pub duration: i32,
    /// The picto texture
    #[serde(borrow)]
    pub picto_filename: HipStr<'a>,
}

impl PictogramClip<'_> {
    /// Convert this clip to the UbiArt representation
    #[must_use]
    pub fn to_tape(&self, song: &Song) -> tape::PictogramClip<'static> {
        let mut hasher = Murmur3Hasher::default();
        self.hash(&mut hasher);
        let id = hasher.finish32();

        let lower_map_name = song.map_name.to_lowercase();
        let filename = &self.picto_filename;

        tape::PictogramClip {
            class: None,
            id,
            track_id: 4_094_799_440,
            is_active: u8::from(self.is_active),
            start_time: self.start_time,
            duration: self.duration,
            picto_path: HipStr::from(format!(
                "world/maps/{lower_map_name}/timeline/pictos/{filename}"
            )),
            coach_count: 4_294_967_295,
            montage_path: None,
            atl_index: None,
        }
    }
}

impl<'a> TryFrom<tape::PictogramClip<'a>> for PictogramClip<'a> {
    type Error = Error;

    fn try_from(value: tape::PictogramClip<'a>) -> Result<Self, Self::Error> {
        // Normally all picto filenames end in .png, but some mods have the filename ending in .tga
        let regex = regex!(r".*/(.*\.(?:png|tga))$");
        let picto_filename = hipstr_regex_single_capture(regex, &value.picto_path)?;

        Ok(Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            picto_filename,
        })
    }
}

/// Resize an actor?
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProportionClip {
    /// Is the clip active
    pub is_active: bool,
    /// When to start the vibration
    pub start_time: i32,
    /// Duration of the vibration
    pub duration: i32,
    /// The actors to resize
    pub actor_indices: Vec<u8>,
    /// X curve
    pub curve_x: Option<CurveFloat>,
    /// Y curve
    pub curve_y: Option<CurveFloat>,
}

impl From<ProportionClip> for tape::ProportionClip<'static> {
    fn from(value: ProportionClip) -> Self {
        let mut hasher = Murmur3Hasher::default();
        value.hash(&mut hasher);
        let id = hasher.finish32();
        Self {
            class: None,
            id,
            track_id: 1_369_275_280,
            is_active: u8::from(value.is_active),
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            target_actors: Vec::new(),
            curve_x: value
                .curve_x
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
            curve_y: value
                .curve_y
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
        }
    }
}

impl From<tape::ProportionClip<'_>> for ProportionClip {
    fn from(value: tape::ProportionClip<'_>) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            curve_x: CurveFloat::from_curve(&value.curve_x),
            curve_y: CurveFloat::from_curve(&value.curve_y),
        }
    }
}

/// Rotate an actor?
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RotationClip {
    /// Is the clip active
    pub is_active: bool,
    /// When to start the vibration
    pub start_time: i32,
    /// Duration of the vibration
    pub duration: i32,
    /// The actors to resize
    pub actor_indices: Vec<u8>,
    /// X curve
    pub curve_x: Option<CurveFloat>,
    /// Y curve
    pub curve_y: Option<CurveFloat>,
    /// Z curve
    pub curve_z: Option<CurveFloat>,
}

impl From<RotationClip> for tape::RotationClip<'static> {
    fn from(value: RotationClip) -> Self {
        let mut hasher = Murmur3Hasher::default();
        value.hash(&mut hasher);
        let id = hasher.finish32();
        Self {
            class: None,
            id,
            track_id: 1_369_275_280,
            is_active: u8::from(value.is_active),
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            target_actors: Vec::new(),
            curve_x: value
                .curve_x
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
            curve_y: value
                .curve_y
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
            curve_z: value
                .curve_z
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
        }
    }
}

impl From<tape::RotationClip<'_>> for RotationClip {
    fn from(value: tape::RotationClip<'_>) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            curve_x: CurveFloat::from_curve(&value.curve_x),
            curve_y: CurveFloat::from_curve(&value.curve_y),
            curve_z: CurveFloat::from_curve(&value.curve_z),
        }
    }
}

/// A audio clip
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SoundSetClip<'a> {
    /// Is the clip active
    pub is_active: bool,
    /// When to start the audio clip
    pub start_time: i32,
    /// Duration of the audio
    pub duration: i32,
    /// The audio file
    #[serde(borrow)]
    pub audio_filename: HipStr<'a>,
    /// Name of the audio clip
    #[serde(borrow)]
    pub name: HipStr<'a>,
}

impl SoundSetClip<'_> {
    /// Convert the SoundSetClip to the UbiArt representation with `sound_set_path`
    #[must_use]
    pub fn to_tape<'a>(&self, sound_set_path: HipStr<'a>) -> tape::SoundSetClip<'a> {
        let mut hasher = Murmur3Hasher::default();
        self.hash(&mut hasher);
        let id = hasher.finish32();

        tape::SoundSetClip {
            class: None,
            id,
            track_id: 1_369_275_280,
            is_active: u8::from(self.is_active),
            start_time: self.start_time,
            duration: self.duration,
            sound_set_path,
            sound_channel: 0,
            start_offset: 0,
            stops_on_end: 0,
            accounted_for_duration: 0,
        }
    }
}

/// Move an actor?
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TranslationClip {
    /// Is the clip active
    pub is_active: bool,
    /// When to start the vibration
    pub start_time: i32,
    /// Duration of the vibration
    pub duration: i32,
    /// The actors to resize
    pub actor_indices: Vec<u8>,
    /// X curve
    pub curve_x: Option<CurveFloat>,
    /// Y curve
    pub curve_y: Option<CurveFloat>,
    /// Z curve
    pub curve_z: Option<CurveFloat>,
}

impl From<TranslationClip> for tape::TranslationClip<'static> {
    fn from(value: TranslationClip) -> Self {
        let mut hasher = Murmur3Hasher::default();
        value.hash(&mut hasher);
        let id = hasher.finish32();
        Self {
            class: None,
            id,
            track_id: 1_369_275_280,
            is_active: u8::from(value.is_active),
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            target_actors: Vec::new(),
            curve_x: value
                .curve_x
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
            curve_y: value
                .curve_y
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
            curve_z: value
                .curve_z
                .as_ref()
                .map(tape::BezierCurveFloat::from)
                .unwrap_or_default(),
        }
    }
}

impl From<tape::TranslationClip<'_>> for TranslationClip {
    fn from(value: tape::TranslationClip<'_>) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            actor_indices: value.actor_indices,
            curve_x: CurveFloat::from_curve(&value.curve_x),
            curve_y: CurveFloat::from_curve(&value.curve_y),
            curve_z: CurveFloat::from_curve(&value.curve_z),
        }
    }
}

/// A vibration of the controller
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VibrationClip<'a> {
    /// Is the clip active
    pub is_active: bool,
    /// When to start the vibration
    pub start_time: i32,
    /// Duration of the vibration
    pub duration: i32,
    /// The vibration file
    #[serde(borrow)]
    pub vibration: HipStr<'a>,
}

impl<'a> From<VibrationClip<'a>> for tape::VibrationClip<'a> {
    fn from(value: VibrationClip<'a>) -> Self {
        let mut hasher = Murmur3Hasher::default();
        value.hash(&mut hasher);
        let id = hasher.finish32();
        Self {
            class: None,
            id,
            track_id: 1_369_275_280,
            is_active: u8::from(value.is_active),
            start_time: value.start_time,
            duration: value.duration,
            vibration_file_path: value.vibration,
            loop_it: 0,
            device_side: 0,
            player_id: -1,
            context: 0,
            start_time_offset: 0.0,
            modulation: 0.5,
        }
    }
}

impl<'a> From<tape::VibrationClip<'a>> for VibrationClip<'a> {
    fn from(value: tape::VibrationClip<'a>) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            vibration: value.vibration_file_path,
        }
    }
}

/// Describes how a value can change (if not constant)
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CurveFloat {
    /// Constant value (no curve)
    Constant(CurveFloatConstant),
    /// Bezier curve with two points
    Linear(CurveFloatLinear),
    /// Bezier curve with multiple points
    Multi(CurveFloatMulti),
}

impl CurveFloat {
    /// Convert a [`tape::Curve`] into a `Option<CurveFloat>`
    #[must_use]
    pub fn from_curve(value: &tape::BezierCurveFloat<'_>) -> Option<Self> {
        match &value.value {
            tape::BezierCurveFloatValue::Empty(_) => None,
            tape::BezierCurveFloatValue::Constant(value) => Some(Self::Constant(value.into())),
            tape::BezierCurveFloatValue::Linear(value) => Some(Self::Linear(value.into())),
            tape::BezierCurveFloatValue::Multi(value) => Some(Self::Multi(value.into())),
        }
    }
}

impl From<&CurveFloat> for tape::BezierCurveFloat<'static> {
    fn from(value: &CurveFloat) -> Self {
        let value = match value {
            CurveFloat::Constant(value) => tape::BezierCurveFloatValue::Constant(value.into()),
            CurveFloat::Linear(value) => tape::BezierCurveFloatValue::Linear(value.into()),
            CurveFloat::Multi(value) => tape::BezierCurveFloatValue::Multi(value.into()),
        };

        Self { class: None, value }
    }
}

/// Constant value for the 'curve'
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CurveFloatConstant(pub f32);

impl From<&tape::BezierCurveFloatConstant<'_>> for CurveFloatConstant {
    fn from(value: &tape::BezierCurveFloatConstant) -> Self {
        Self(value.value)
    }
}

impl From<&CurveFloatConstant> for tape::BezierCurveFloatConstant<'static> {
    fn from(value: &CurveFloatConstant) -> Self {
        Self {
            class: None,
            value: value.0,
        }
    }
}

impl Hash for CurveFloatConstant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialEq for CurveFloatConstant {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}

impl Eq for CurveFloatConstant {}

impl Ord for CurveFloatConstant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for CurveFloatConstant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Linear bezier curve (2 points)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CurveFloatLinear {
    /// Unknown
    pub value_left: (f32, f32),
    /// Unknown
    pub normal_left_out: (f32, f32),
    /// Unknown
    pub value_right: (f32, f32),
    /// Unknown
    pub normal_right_in: (f32, f32),
}

impl From<&tape::BezierCurveFloatLinear<'_>> for CurveFloatLinear {
    fn from(value: &tape::BezierCurveFloatLinear) -> Self {
        Self {
            value_left: value.value_left,
            normal_left_out: value.normal_left_out,
            value_right: value.value_right,
            normal_right_in: value.normal_right_in,
        }
    }
}

impl From<&CurveFloatLinear> for tape::BezierCurveFloatLinear<'static> {
    fn from(value: &CurveFloatLinear) -> Self {
        Self {
            class: None,
            value_left: value.value_left,
            normal_left_out: value.normal_left_out,
            value_right: value.value_right,
            normal_right_in: value.normal_right_in,
        }
    }
}

impl Hash for CurveFloatLinear {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value_left.0.to_bits().hash(state);
        self.value_left.1.to_bits().hash(state);
        self.normal_left_out.0.to_bits().hash(state);
        self.normal_left_out.1.to_bits().hash(state);
        self.value_right.0.to_bits().hash(state);
        self.value_right.1.to_bits().hash(state);
        self.normal_right_in.0.to_bits().hash(state);
        self.normal_right_in.1.to_bits().hash(state);
    }
}

impl PartialEq for CurveFloatLinear {
    fn eq(&self, other: &Self) -> bool {
        self.value_left.0.total_cmp(&other.value_left.0) == Ordering::Equal
            && self.value_left.1.total_cmp(&other.value_left.0) == Ordering::Equal
            && self.normal_left_out.0.total_cmp(&other.normal_left_out.0) == Ordering::Equal
            && self.normal_left_out.1.total_cmp(&other.normal_left_out.0) == Ordering::Equal
            && self.value_right.0.total_cmp(&other.value_right.0) == Ordering::Equal
            && self.value_right.1.total_cmp(&other.value_right.0) == Ordering::Equal
            && self.normal_right_in.0.total_cmp(&other.normal_right_in.0) == Ordering::Equal
            && self.normal_right_in.1.total_cmp(&other.normal_right_in.0) == Ordering::Equal
    }
}

impl Eq for CurveFloatLinear {}

impl Ord for CurveFloatLinear {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value_left
            .0
            .total_cmp(&other.value_left.0)
            .then(self.value_left.1.total_cmp(&other.value_left.0))
            .then(self.normal_left_out.0.total_cmp(&other.normal_left_out.0))
            .then(self.normal_left_out.1.total_cmp(&other.normal_left_out.0))
            .then(self.value_right.0.total_cmp(&other.value_right.0))
            .then(self.value_right.1.total_cmp(&other.value_right.0))
            .then(self.normal_right_in.0.total_cmp(&other.normal_right_in.0))
            .then(self.normal_right_in.1.total_cmp(&other.normal_right_in.0))
    }
}

impl PartialOrd for CurveFloatLinear {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Mutli point bezier curve (more than 4)
#[derive(Hash, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurveFloatMulti {
    /// The points of the curve
    pub keys: Vec<KeyFloat>,
}

impl From<&tape::BezierCurveFloatMulti<'_>> for CurveFloatMulti {
    fn from(value: &tape::BezierCurveFloatMulti) -> Self {
        Self {
            keys: value.keys.iter().map(KeyFloat::from).collect(),
        }
    }
}

impl From<&CurveFloatMulti> for tape::BezierCurveFloatMulti<'static> {
    fn from(value: &CurveFloatMulti) -> Self {
        Self {
            class: None,
            keys: value.keys.iter().map(tape::KeyFloat::from).collect(),
        }
    }
}

/// Key float for multi point bezier curve
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyFloat {
    /// Unknown
    pub value: (f32, f32),
    /// Unknown
    pub normal_out: (f32, f32),
    /// Unknown
    pub normal_in: (f32, f32),
}

impl From<&tape::KeyFloat<'_>> for KeyFloat {
    fn from(value: &tape::KeyFloat) -> Self {
        Self {
            value: value.value,
            normal_out: value.normal_out,
            normal_in: value.normal_in,
        }
    }
}

impl From<&KeyFloat> for tape::KeyFloat<'static> {
    fn from(value: &KeyFloat) -> Self {
        Self {
            class: Some(HipStr::from("KeyFloat")),
            value: value.value,
            normal_out: value.normal_out,
            normal_in: value.normal_in,
        }
    }
}

impl Hash for KeyFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.0.to_bits().hash(state);
        self.value.1.to_bits().hash(state);
        self.normal_out.0.to_bits().hash(state);
        self.normal_out.1.to_bits().hash(state);
        self.normal_in.0.to_bits().hash(state);
        self.normal_in.1.to_bits().hash(state);
    }
}

impl PartialEq for KeyFloat {
    fn eq(&self, other: &Self) -> bool {
        self.value.0.total_cmp(&other.value.0) == Ordering::Equal
            && self.value.1.total_cmp(&other.value.0) == Ordering::Equal
            && self.normal_out.0.total_cmp(&other.normal_out.0) == Ordering::Equal
            && self.normal_out.1.total_cmp(&other.normal_out.0) == Ordering::Equal
            && self.normal_in.0.total_cmp(&other.normal_in.0) == Ordering::Equal
            && self.normal_in.1.total_cmp(&other.normal_in.0) == Ordering::Equal
    }
}

impl Eq for KeyFloat {}

impl Ord for KeyFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value
            .0
            .total_cmp(&other.value.0)
            .then(self.value.1.total_cmp(&other.value.0))
            .then(self.normal_out.0.total_cmp(&other.normal_out.0))
            .then(self.normal_out.1.total_cmp(&other.normal_out.0))
            .then(self.normal_in.0.total_cmp(&other.normal_in.0))
            .then(self.normal_in.1.total_cmp(&other.normal_in.0))
    }
}

impl PartialOrd for KeyFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
