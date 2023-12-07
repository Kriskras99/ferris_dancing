//! # Song types
//! Types used to describe songs

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::path::{Path, PathBuf};
use std::{borrow::Cow, hash::Hash};

use anyhow::{anyhow, Error};
use hash32::{Hasher, Murmur3Hasher};
use path_clean::PathClean;
use serde::{Deserialize, Serialize};

use ubiart_toolkit::json_types::BezierCurveFloatValue;
use ubiart_toolkit::utils::Platform;
use ubiart_toolkit::{json_types, utils::LocaleId};

use crate::regex;
use crate::utils::cow_regex_single_capture;

/// Directory structure of a song
pub struct SongDirectoryTree {
    /// Root song dir
    dir_song: PathBuf,
    /// Contains the msm files
    dir_song_moves: PathBuf,
    /// Contains the pictos
    dir_song_pictos: PathBuf,
    /// Contains the menuart
    dir_song_menuart: PathBuf,
    /// Contains the audio clips
    dir_song_audio: PathBuf,
}

impl SongDirectoryTree {
    /// Create a new directory tree from root.
    ///
    /// This does not create directories or check if they exists!
    pub fn new(dir_song: &Path) -> Self {
        let dir_song = dir_song.clean();
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

    /// Create the directory tree.
    pub fn create_all(&self) -> std::io::Result<()> {
        std::fs::create_dir(&self.dir_song)
            .and_then(|()| std::fs::create_dir(&self.dir_song_moves))
            .and_then(|()| std::fs::create_dir(&self.dir_song_pictos))
            .and_then(|()| std::fs::create_dir(&self.dir_song_menuart))
            .and_then(|()| std::fs::create_dir(&self.dir_song_audio))
    }

    /// Check if the directory tree exists.
    pub fn exists(&self) -> bool {
        self.dir_song.exists()
            && self.dir_song_moves.exists()
            && self.dir_song_pictos.exists()
            && self.dir_song_menuart.exists()
            && self.dir_song_audio.exists()
    }

    /// The root of the song directory.
    pub fn song(&self) -> &Path {
        &self.dir_song
    }

    /// Used to store the MovementSpace files.
    pub fn moves(&self) -> &Path {
        &self.dir_song_moves
    }

    /// Used to store the pictos.
    pub fn pictos(&self) -> &Path {
        &self.dir_song_pictos
    }

    /// Used to store the pictos.
    pub fn menuart(&self) -> &Path {
        &self.dir_song_menuart
    }

    /// Used to store the pictos.
    pub fn audio(&self) -> &Path {
        &self.dir_song_audio
    }
}

/// Possible tags for a song
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Tag {
    /// Song from the main content
    Main,
    /// Kids song
    KidsOnly,
    /// Alternate version of a song
    Alternate,
    /// Song that uses a (wheel)chair
    BringChairTutorial,
    /// Song that uses a (wheel)chair
    ChairTutorial,
    /// Cool song?
    Cool,
    /// Dance machine map
    DanceMachine,
    /// ??
    Exclusive,
    /// Extreme difficulty song
    Extreme,
    /// Intense song
    Intense,
    /// ??
    KidsMode,
    /// ??
    KidsModeTeaser,
    /// Unknown?
    JdMbs,
    /// Mashup map
    Mashup,
    /// Low intensity song
    NoSweat,
    /// Unlocked using Uplay
    Uplay2016,
    /// Unlocked using Uplay
    Uplay2017,
    /// High intensity song
    Sweat,
    /// Song that uses a bike?
    BikeTutorial,
    /// Song that uses a (wheel)chair
    Chair2Tutorial,
}

impl Tag {
    // TODO: Add normalisation?
    /// Convert the tag to a static str
    pub const fn to_cow(self) -> Cow<'static, str> {
        match self {
            Self::Main => Cow::Borrowed("main"),
            Self::KidsOnly => Cow::Borrowed("kidsonly"),
            Self::Alternate => Cow::Borrowed("alternate"),
            Self::BringChairTutorial => Cow::Borrowed("bringchairtutorial"),
            Self::ChairTutorial => Cow::Borrowed("chairtutorial"),
            Self::Cool => Cow::Borrowed("cool"),
            Self::DanceMachine => Cow::Borrowed("dancemachine"),
            Self::Exclusive => Cow::Borrowed("exclusive"),
            Self::Extreme => Cow::Borrowed("extreme"),
            Self::Intense => Cow::Borrowed("intense"),
            Self::KidsMode => Cow::Borrowed("kidsmode"),
            Self::KidsModeTeaser => Cow::Borrowed("kidsmodeteaser"),
            Self::JdMbs => Cow::Borrowed("jdmbs"),
            Self::Mashup => Cow::Borrowed("mashup"),
            Self::NoSweat => Cow::Borrowed("nosweat"),
            Self::Uplay2016 => Cow::Borrowed("uplay2016"),
            Self::Uplay2017 => Cow::Borrowed("uplay2017"),
            Self::Sweat => Cow::Borrowed("sweat"),
            Self::BikeTutorial => Cow::Borrowed("biketutorial"),
            Self::Chair2Tutorial => Cow::Borrowed("chair2tutorial"),
        }
    }
}

impl TryFrom<&str> for Tag {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value_lower = value.to_lowercase();
        match value_lower.as_str() {
            "main" => Ok(Self::Main),
            "kidsonly" => Ok(Self::KidsOnly),
            "alternate" => Ok(Self::Alternate),
            "bringchairtutorial" => Ok(Self::BringChairTutorial),
            "chairtutorial" => Ok(Self::ChairTutorial),
            "cool" => Ok(Self::Cool),
            "dancemachine" => Ok(Self::DanceMachine),
            "exclusive" => Ok(Self::Exclusive),
            "extreme" => Ok(Self::Extreme),
            "intense" => Ok(Self::Intense),
            "kidsmode" => Ok(Self::KidsMode),
            "kidsmodeteaser" => Ok(Self::KidsModeTeaser),
            "jdmbs" => Ok(Self::JdMbs),
            "mashup" => Ok(Self::Mashup),
            "nosweat" => Ok(Self::NoSweat),
            "uplay2016" => Ok(Self::Uplay2016),
            "uplay2017" => Ok(Self::Uplay2017),
            "sweat" => Ok(Self::Sweat),
            "biketutorial" => Ok(Self::BikeTutorial),
            "chair2tutorial" => Ok(Self::Chair2Tutorial),
            _ => Err(anyhow!("Unknown tag!: {value}")),
        }
    }
}

/// Number of coaches for this song
#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
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

impl TryFrom<u8> for NumberOfCoaches {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

impl From<NumberOfCoaches> for u8 {
    #[allow(clippy::as_conversions)]
    fn from(value: NumberOfCoaches) -> Self {
        value as Self
    }
}

/// Main metadata about the song
#[derive(Serialize, Deserialize, Clone)]
pub struct Song<'a> {
    /// Codename for the song (Capitalised)
    pub map_name: Cow<'a, str>,
    /// Original Just Dance version
    pub original_jd_version: u16,
    /// Artist
    pub artist: Cow<'a, str>,
    /// Coach name
    pub dancer_name: Cow<'a, str>,
    /// Song name
    pub title: Cow<'a, str>,
    /// Writing credits
    pub credits: Cow<'a, str>,
    /// Number of coaches
    pub number_of_coaches: NumberOfCoaches,
    /// Which of the coaches is the main coach? None if only one coach
    pub main_coach: Option<u8>,
    /// Difficulty of the song
    pub difficulty: Difficulty,
    /// Intensity of the song
    pub sweat_difficulty: SweatDifficulty,
    /// Related songs (other difficulties, covers)
    pub related_songs: Vec<Cow<'a, str>>,
    /// How is the song unlocked
    pub status: MapStatus,
    /// Tags related to this song
    pub tags: Vec<Tag>,
    /// Subtitle locale id (like: Extreme Version)
    pub subtitle: LocaleId,
    /// Theme colors of the song
    pub default_colors: SongColors,
    /// The audio file for the song
    pub audiofile: Cow<'a, str>,
    /// The videofile for the song
    pub videofile: Cow<'a, str>,
}

/// Settings used by the autodance preview
#[derive(Serialize, Deserialize, Clone)]
pub struct Autodance<'a> {
    /// Soundclip to play
    pub autodance_sound: Cow<'a, str>,
    /// Position in the clip to start at
    pub song_start_position: i32,
    /// Duration to play
    pub duration: f32,
    /// Unknown
    pub record: Vec<Record>,
    /// Unknown
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub playback_events: Vec<PlaybackEvent>,
}

/// Image used in the menus
#[derive(Serialize, Deserialize, Clone)]
pub enum MenuArt<'a> {
    /// Image for the game itself
    Texture(MenuArtTexture<'a>),
    /// Image for a phone controller
    Phone(PhoneImage<'a>),
}

/// Texture used by the game
#[derive(Serialize, Deserialize, Clone)]
pub struct MenuArtTexture<'a> {
    /// Userfriendly name
    pub name: Cow<'a, str>,
    /// Filename
    pub filename: Cow<'a, str>,
    /// Scale as used in [`ubiart_toolkit::cooked::isc::MaterialGraphicComponent`]
    pub scale: (f32, f32),
    /// 2d position as used in [`ubiart_toolkit::cooked::isc::MaterialGraphicComponent`]
    pub pos2d: (f64, f64),
    /// Disable shadow as used in [`ubiart_toolkit::cooked::isc::MaterialGraphicComponent`]
    pub disable_shadow: u32,
    /// Unknown? Used in [`ubiart_toolkit::cooked::isc::MaterialGraphicComponent`]
    pub anchor: i32,
}

/// Image used for the phone application
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhoneImage<'a> {
    /// Filename
    pub filename: Cow<'a, str>,
    /// Userfriendly name
    pub name: Cow<'a, str>,
}

/// Unknown
#[derive(Serialize, Deserialize, Clone)]
pub struct Record {
    /// Start of the record?
    pub start: f32,
    /// Duration of the record?
    pub duration: f32,
}

impl From<&Record> for json_types::Record<'static> {
    fn from(value: &Record) -> Self {
        Self {
            class: Some(Self::CLASS),
            start: value.start,
            duration: value.duration,
        }
    }
}

impl From<&json_types::Record<'_>> for Record {
    fn from(value: &json_types::Record) -> Self {
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

impl From<&PlaybackEvent> for json_types::PlaybackEvent<'static> {
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

impl From<&json_types::PlaybackEvent<'_>> for PlaybackEvent {
    fn from(value: &json_types::PlaybackEvent) -> Self {
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
#[derive(Hash, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
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

impl From<&Color> for (f32, f32, f32, f32) {
    fn from(color: &Color) -> Self {
        (
            f32::from(color.transparency) / 255.0,
            f32::from(color.red) / 255.0,
            f32::from(color.green) / 255.0,
            f32::from(color.blue) / 255.0,
        )
    }
}

impl From<&(f32, f32, f32, f32)> for Color {
    fn from(value: &(f32, f32, f32, f32)) -> Self {
        Self {
            transparency: map_range_to_u8(value.0, 0.0, 1.0),
            red: map_range_to_u8(value.1, 0.0, 1.0),
            green: map_range_to_u8(value.2, 0.0, 1.0),
            blue: map_range_to_u8(value.3, 0.0, 1.0),
        }
    }
}

/// Map a `value` in range `min` to `max` to a u8
#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::as_conversions
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
        assert!(
            (0.0..=255.0).contains(&new_value),
            "Range conversion failed! {new_value}"
        );
        new_value as u8
    }
}

/// Theme colors of the song
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
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

impl From<&SongColors> for json_types::DefaultColors {
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

impl From<&json_types::DefaultColors> for SongColors {
    fn from(value: &json_types::DefaultColors) -> Self {
        Self {
            theme: (&value.theme).into(),
            lyrics: (&value.lyrics).into(),
            one_a: (value.songcolor_1a.as_ref().unwrap_or(&value.theme)).into(),
            one_b: (value.songcolor_1b.as_ref().unwrap_or(&value.theme)).into(),
            two_a: (value.songcolor_2a.as_ref().unwrap_or(&value.theme)).into(),
            two_b: (value.songcolor_2b.as_ref().unwrap_or(&value.theme)).into(),
        }
    }
}

/// How is this map unlocked
#[repr(u8)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MapStatus {
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

impl From<MapStatus> for u8 {
    #[allow(clippy::as_conversions)]
    fn from(value: MapStatus) -> Self {
        value as Self
    }
}

impl MapStatus {
    /// Normalize the map status to those used by JD2022
    pub const fn normalize(self) -> Self {
        match self {
            Self::Quest => Self::Quest,
            _ => Self::Unlocked,
        }
    }
}

impl TryFrom<u8> for MapStatus {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
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
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

impl From<Difficulty> for u8 {
    #[allow(clippy::as_conversions)]
    fn from(value: Difficulty) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for Difficulty {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SweatDifficulty {
    /// Low intensity
    Low = 1,
    /// Medium intensity
    Moderate = 2,
    /// High intensity
    Intense = 3,
}

impl From<SweatDifficulty> for u8 {
    #[allow(clippy::as_conversions)]
    fn from(value: SweatDifficulty) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for SweatDifficulty {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
    pub preview_entry: f32,
    /// Preview audio track start
    pub preview_loop_start: f32,
    /// Preview audio track end
    pub preview_loop_end: f32,
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
    pub marker: f32,
    /// Unknown
    pub beats: u32,
}

impl From<Signature> for json_types::MusicSignature<'static> {
    fn from(value: Signature) -> Self {
        Self {
            class: Some(Self::CLASS),
            marker: value.marker,
            beats: value.beats,
            comment: None,
        }
    }
}

impl From<json_types::MusicSignature<'_>> for Signature {
    fn from(value: json_types::MusicSignature) -> Self {
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
    pub marker: f32,
    /// Unknown
    pub section_type: u32,
}

impl From<Section> for json_types::MusicSection<'static> {
    fn from(value: Section) -> Self {
        Self {
            class: Some(Self::CLASS),
            marker: value.marker,
            section_type: value.section_type,
            comment: Cow::Borrowed(""),
        }
    }
}

impl From<json_types::MusicSection<'_>> for Section {
    fn from(value: json_types::MusicSection) -> Self {
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
    pub timeline: BinaryHeap<Clip<'a>>,
}

/// A event that happens during a song
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Clip<'a> {
    /// Unknown
    Alpha(AlphaClip),
    /// Unknown
    Color(ColorClip),
    /// Unknown
    GameplayEvent(GameplayEventClip<'a>),
    /// Gold move effect
    GoldEffect(GoldEffectClip),
    /// Hide user interface
    HideUserInterface(HideUserInterfaceClip),
    /// Show lyric
    Karaoke(KaraokeClip<'a>),
    /// Unknown
    MaterialGraphicEnableLayer(MaterialGraphicEnableLayerClip),
    /// Grade dance move
    Motion(MotionClip<'a>),
    /// Show picto
    Pictogram(PictogramClip<'a>),
    /// Unknown
    Proportion(ProportionClip),
    /// Unknown
    Rotation(RotationClip),
    /// Play audio sample
    SoundSet(SoundSetClip<'a>),
    /// Unknown
    Translation(TranslationClip),
    /// Vibrate the controller
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
    pub fn to_tape(self, song: &Song) -> Result<json_types::Clip<'a>, Error> {
        match self {
            Self::SoundSet(_) => Err(anyhow!(
                "Converting SoundSetClip through the Clip enum is not supported!"
            )),
            Self::Alpha(data) => Ok(json_types::Clip::Alpha(data.into())),
            Self::Color(data) => Ok(json_types::Clip::Color(data.into())),
            Self::GameplayEvent(data) => Ok(json_types::Clip::GameplayEvent(data.into())),
            Self::GoldEffect(data) => Ok(json_types::Clip::GoldEffect(data.into())),
            Self::HideUserInterface(data) => Ok(json_types::Clip::HideUserInterface(data.into())),
            Self::Karaoke(data) => Ok(json_types::Clip::Karaoke(data.into())),
            Self::MaterialGraphicEnableLayer(data) => {
                Ok(json_types::Clip::MaterialGraphicEnableLayer(data.into()))
            }
            Self::Motion(data) => Ok(json_types::Clip::Motion(data.to_tape(song))),
            Self::Pictogram(data) => Ok(json_types::Clip::Pictogram(data.to_tape(song))),
            Self::Proportion(data) => Ok(json_types::Clip::Proportion(data.into())),
            Self::Rotation(data) => Ok(json_types::Clip::Rotation(data.into())),
            Self::Translation(data) => Ok(json_types::Clip::Translation(data.into())),
            Self::Vibration(data) => Ok(json_types::Clip::Vibration(data.into())),
        }
    }

    /// The start time of a clip
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
    pub const fn duration(&self) -> u32 {
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

impl<'a> TryFrom<json_types::Clip<'a>> for Clip<'a> {
    type Error = Error;

    fn try_from(value: json_types::Clip<'a>) -> Result<Self, Self::Error> {
        match value {
            json_types::Clip::Alpha(data) => Ok(Self::Alpha(data.into())),
            json_types::Clip::Color(data) => Ok(Self::Color(data.into())),
            json_types::Clip::GameplayEvent(data) => Ok(Self::GameplayEvent(data.into())),
            json_types::Clip::GoldEffect(data) => Ok(Self::GoldEffect(data.into())),
            json_types::Clip::HideUserInterface(data) => Ok(Self::HideUserInterface(data.into())),
            json_types::Clip::Karaoke(data) => Ok(Self::Karaoke(data.into())),
            json_types::Clip::MaterialGraphicEnableLayer(data) => {
                Ok(Self::MaterialGraphicEnableLayer(data.into()))
            }
            json_types::Clip::Motion(data) => Ok(Self::Motion(data.try_into()?)),
            json_types::Clip::Pictogram(data) => Ok(Self::Pictogram(data.try_into()?)),
            json_types::Clip::Proportion(data) => Ok(Self::Proportion(data.into())),
            json_types::Clip::Rotation(data) => Ok(Self::Rotation(data.into())),
            json_types::Clip::SoundSet(_) => Err(anyhow!(
                "Converting SoundSet clip through the Clip enum is not supported!"
            )),
            json_types::Clip::TapeReference(_) => Err(anyhow!(
                "Converting TapeReference clip through the Clip enum is not supported!"
            )),
            json_types::Clip::Translation(data) => Ok(Self::Translation(data.into())),
            json_types::Clip::Vibration(data) => Ok(Self::Vibration(data.into())),
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
    pub duration: u32,
    /// Unknown
    pub actor_indices: Vec<u8>,
    /// Unknown
    pub curve: Option<CurveFloat>,
}

impl From<AlphaClip> for json_types::AlphaClip<'static> {
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
            curve: value
                .curve
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
        }
    }
}

impl From<json_types::AlphaClip<'_>> for AlphaClip {
    fn from(value: json_types::AlphaClip<'_>) -> Self {
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
    pub duration: u32,
    /// The actors to color
    pub actor_indices: Vec<u8>,
    /// Red color curve
    pub curve_red: Option<CurveFloat>,
    /// Red color curve
    pub curve_green: Option<CurveFloat>,
    /// Red color curve
    pub curve_blue: Option<CurveFloat>,
}

impl From<ColorClip> for json_types::ColorClip<'static> {
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
            curve_red: value
                .curve_red
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
            curve_green: value
                .curve_green
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
            curve_blue: value
                .curve_blue
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
        }
    }
}

impl From<json_types::ColorClip<'_>> for ColorClip {
    fn from(value: json_types::ColorClip<'_>) -> Self {
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
    pub duration: u32,
    /// Unknown
    pub actor_indices: Vec<u8>,
    /// Unknown
    pub event_type: u32,
    /// Unknown
    pub custom_param: Cow<'a, str>,
}

impl<'a> From<GameplayEventClip<'a>> for json_types::GameplayEventClip<'a> {
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
            event_type: value.event_type,
            custom_param: value.custom_param,
        }
    }
}

impl<'a> From<json_types::GameplayEventClip<'a>> for GameplayEventClip<'a> {
    fn from(value: json_types::GameplayEventClip<'a>) -> Self {
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
    pub duration: u32,
    /// Unknown
    pub effect_type: u8,
}

impl From<GoldEffectClip> for json_types::GoldEffectClip<'static> {
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

impl From<json_types::GoldEffectClip<'_>> for GoldEffectClip {
    fn from(value: json_types::GoldEffectClip) -> Self {
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
    pub duration: u32,
    /// Unknown
    pub event_type: u32,
}

impl From<HideUserInterfaceClip> for json_types::HideUserInterfaceClip<'static> {
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
            custom_param: Cow::Borrowed(""),
            is_active: u8::from(value.is_active),
        }
    }
}

impl From<json_types::HideUserInterfaceClip<'_>> for HideUserInterfaceClip {
    fn from(value: json_types::HideUserInterfaceClip) -> Self {
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
    pub duration: u32,
    /// Expected pitch of the lyric (for use with microphone enabled systems)
    pub pitch: f32,
    /// The lyric
    pub lyrics: Cow<'a, str>,
    /// Should the next lyric be on a new line
    pub is_end_of_line: bool,
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

impl<'a> From<KaraokeClip<'a>> for json_types::KaraokeClip<'a> {
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
            content_type: 2,
            start_time_tolerance: 4,
            end_time_tolerance: 4,
            semitone_tolerance: 5.0,
        }
    }
}

impl<'a> From<json_types::KaraokeClip<'a>> for KaraokeClip<'a> {
    fn from(value: json_types::KaraokeClip<'a>) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            pitch: value.pitch,
            lyrics: value.lyrics,
            is_end_of_line: value.is_end_of_line == 1,
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
    pub duration: u32,
    /// Unknown
    pub actor_indices: Vec<u8>,
    /// Unknown
    pub layer_idx: u8,
    /// Unknown
    pub uv_modifier_idx: u8,
    /// Unknown
    pub layer_enabled: bool,
}

impl From<MaterialGraphicEnableLayerClip> for json_types::MaterialGraphicEnableLayerClip<'static> {
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
            layer_idx: value.layer_idx,
            uv_modifier_idx: value.uv_modifier_idx,
            layer_enabled: u8::from(value.layer_enabled),
        }
    }
}

impl From<json_types::MaterialGraphicEnableLayerClip<'_>> for MaterialGraphicEnableLayerClip {
    fn from(value: json_types::MaterialGraphicEnableLayerClip<'_>) -> Self {
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
    pub duration: u32,
    /// The classifier (.msm file for NX)
    pub classifier_filename: Cow<'a, str>,
    /// Is this a gold move
    pub gold_move: bool,
    /// Which coach this move is tracking
    pub coach_id: u8,
    /// The color of something?
    pub color: Color,
}

impl MotionClip<'_> {
    /// Convert this clip to the UbiArt representation
    pub fn to_tape(&self, song: &Song) -> json_types::MotionClip<'static> {
        let mut hasher = Murmur3Hasher::default();
        self.hash(&mut hasher);
        let id = hasher.finish32();

        let lower_map_name = song.map_name.to_lowercase();
        let filename = self.classifier_filename.as_ref();

        json_types::MotionClip {
            class: None,
            id,
            track_id: 4_094_799_440,
            is_active: u8::from(self.is_active),
            start_time: self.start_time,
            duration: self.duration,
            classifier_path: Cow::Owned(format!(
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
    pub fn fix_classifier_path(classifier_path: &str, platform: Platform) -> Result<String, Error> {
        // Classifier path does not include platform specifier
        let index = classifier_path
            .rfind('/')
            .ok_or_else(|| anyhow!("Weird classifier path"))?;
        let (left, right) = classifier_path.split_at(index);
        let mut classifier_path = String::with_capacity(classifier_path.len() + 5);
        classifier_path.push_str(left);
        match platform {
            Platform::Nx => classifier_path.push_str("/wiiu"),
            _ => unimplemented!("Not implemented for {}", platform),
        }
        classifier_path.push_str(right);
        Ok(classifier_path)
    }
}

impl<'a> TryFrom<json_types::MotionClip<'a>> for MotionClip<'a> {
    type Error = Error;

    fn try_from(value: json_types::MotionClip<'a>) -> Result<Self, Self::Error> {
        let regex = regex!(r".*/[a-z0-9]*_(.*\.msm|.*\.gesture)$");
        let classifier_filename = cow_regex_single_capture(regex, value.classifier_path)?;

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
    pub duration: u32,
    /// The picto texture
    pub picto_filename: Cow<'a, str>,
}

impl PictogramClip<'_> {
    /// Convert this clip to the UbiArt representation
    pub fn to_tape(&self, song: &Song) -> json_types::PictogramClip<'static> {
        let mut hasher = Murmur3Hasher::default();
        self.hash(&mut hasher);
        let id = hasher.finish32();

        let lower_map_name = song.map_name.to_lowercase();
        let filename = self.picto_filename.as_ref();

        json_types::PictogramClip {
            class: None,
            id,
            track_id: 4_094_799_440,
            is_active: u8::from(self.is_active),
            start_time: self.start_time,
            duration: self.duration,
            picto_path: Cow::Owned(format!(
                "world/maps/{lower_map_name}/timeline/pictos/{filename}"
            )),
            coach_count: 4_294_967_295,
            montage_path: None,
            atl_index: None,
        }
    }
}

impl<'a> TryFrom<json_types::PictogramClip<'a>> for PictogramClip<'a> {
    type Error = Error;

    fn try_from(value: json_types::PictogramClip<'a>) -> Result<Self, Self::Error> {
        let regex = regex!(r".*/(.*\.png)$");
        let picto_filename = cow_regex_single_capture(regex, value.picto_path)?;

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
    pub duration: u32,
    /// The actors to resize
    pub actor_indices: Vec<u8>,
    /// X curve
    pub curve_x: Option<CurveFloat>,
    /// Y curve
    pub curve_y: Option<CurveFloat>,
}

impl From<ProportionClip> for json_types::ProportionClip<'static> {
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
            curve_x: value
                .curve_x
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
            curve_y: value
                .curve_y
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
        }
    }
}

impl From<json_types::ProportionClip<'_>> for ProportionClip {
    fn from(value: json_types::ProportionClip<'_>) -> Self {
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
    pub duration: u32,
    /// The actors to resize
    pub actor_indices: Vec<u8>,
    /// X curve
    pub curve_x: Option<CurveFloat>,
    /// Y curve
    pub curve_y: Option<CurveFloat>,
    /// Z curve
    pub curve_z: Option<CurveFloat>,
}

impl From<RotationClip> for json_types::RotationClip<'static> {
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
            curve_x: value
                .curve_x
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
            curve_y: value
                .curve_y
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
            curve_z: value
                .curve_z
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
        }
    }
}

impl From<json_types::RotationClip<'_>> for RotationClip {
    fn from(value: json_types::RotationClip<'_>) -> Self {
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
    pub duration: u32,
    /// The audio file
    pub audio_filename: Cow<'a, str>,
    /// Name of the audio clip
    pub name: Cow<'a, str>,
}

impl SoundSetClip<'_> {
    /// Convert the SoundSetClip to the UbiArt representation with `sound_set_path`
    pub fn to_tape<'a>(&self, sound_set_path: Cow<'a, str>) -> json_types::SoundSetClip<'a> {
        let mut hasher = Murmur3Hasher::default();
        self.hash(&mut hasher);
        let id = hasher.finish32();

        json_types::SoundSetClip {
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
    pub duration: u32,
    /// The actors to resize
    pub actor_indices: Vec<u8>,
    /// X curve
    pub curve_x: Option<CurveFloat>,
    /// Y curve
    pub curve_y: Option<CurveFloat>,
    /// Z curve
    pub curve_z: Option<CurveFloat>,
}

impl From<TranslationClip> for json_types::TranslationClip<'static> {
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
            curve_x: value
                .curve_x
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
            curve_y: value
                .curve_y
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
            curve_z: value
                .curve_z
                .as_ref()
                .map(json_types::Curve::from)
                .unwrap_or_default(),
        }
    }
}

impl From<json_types::TranslationClip<'_>> for TranslationClip {
    fn from(value: json_types::TranslationClip<'_>) -> Self {
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
    pub duration: u32,
    /// The vibration file
    pub vibration: Cow<'a, str>,
}

impl<'a> From<VibrationClip<'a>> for json_types::VibrationClip<'a> {
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

impl<'a> From<json_types::VibrationClip<'a>> for VibrationClip<'a> {
    fn from(value: json_types::VibrationClip<'a>) -> Self {
        Self {
            is_active: value.is_active == 1,
            start_time: value.start_time,
            duration: value.duration,
            vibration: value.vibration_file_path.clone(),
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
    /// Convert a [`json_types::Curve`] into a Option<CurveFloat>
    pub fn from_curve(value: &json_types::Curve<'_>) -> Option<Self> {
        let json_types::Curve::BezierCurveFloat(value) = value;
        match &value.value {
            BezierCurveFloatValue::Empty => None,
            BezierCurveFloatValue::Constant(value) => Some(Self::Constant(value.into())),
            BezierCurveFloatValue::Linear(value) => Some(Self::Linear(value.into())),
            BezierCurveFloatValue::Multi(value) => Some(Self::Multi(value.into())),
        }
    }
}

impl From<&CurveFloat> for json_types::Curve<'static> {
    fn from(value: &CurveFloat) -> Self {
        let value = match value {
            CurveFloat::Constant(value) => BezierCurveFloatValue::Constant(value.into()),
            CurveFloat::Linear(value) => BezierCurveFloatValue::Linear(value.into()),
            CurveFloat::Multi(value) => BezierCurveFloatValue::Multi(value.into()),
        };

        Self::BezierCurveFloat(json_types::BezierCurveFloat { class: None, value })
    }
}

/// Constant value for the 'curve'
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CurveFloatConstant(pub f32);

impl From<&json_types::BezierCurveFloatConstant<'_>> for CurveFloatConstant {
    fn from(value: &json_types::BezierCurveFloatConstant) -> Self {
        Self(value.value)
    }
}

impl From<&CurveFloatConstant> for json_types::BezierCurveFloatConstant<'static> {
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

impl From<&json_types::BezierCurveFloatLinear<'_>> for CurveFloatLinear {
    fn from(value: &json_types::BezierCurveFloatLinear) -> Self {
        Self {
            value_left: value.value_left,
            normal_left_out: value.normal_left_out,
            value_right: value.value_right,
            normal_right_in: value.normal_right_in,
        }
    }
}

impl From<&CurveFloatLinear> for json_types::BezierCurveFloatLinear<'static> {
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

impl From<&json_types::BezierCurveFloatMulti<'_>> for CurveFloatMulti {
    fn from(value: &json_types::BezierCurveFloatMulti) -> Self {
        Self {
            keys: value.keys.iter().map(KeyFloat::from).collect(),
        }
    }
}

impl From<&CurveFloatMulti> for json_types::BezierCurveFloatMulti<'static> {
    fn from(value: &CurveFloatMulti) -> Self {
        Self {
            class: None,
            keys: value.keys.iter().map(json_types::KeyFloat::from).collect(),
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

impl From<&json_types::KeyFloat<'_>> for KeyFloat {
    fn from(value: &json_types::KeyFloat) -> Self {
        Self {
            value: value.value,
            normal_out: value.normal_out,
            normal_in: value.normal_in,
        }
    }
}

impl From<&KeyFloat> for json_types::KeyFloat<'static> {
    fn from(value: &KeyFloat) -> Self {
        Self {
            class: Some("KeyFloat"),
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
