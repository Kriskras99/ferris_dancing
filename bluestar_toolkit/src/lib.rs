use std::collections::HashMap;

use hipstr::HipStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Song<'a> {
    #[serde(borrow)]
    pub _id: HipStr<'a>,
    #[serde(borrow)]
    pub id: HipStr<'a>,
    #[serde(borrow)]
    pub artist: HipStr<'a>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub coaches: usize,
    #[serde(borrow)]
    pub status: HipStr<'a>,
    #[serde(borrow)]
    pub credits: Vec<HipStr<'a>>,
    pub avatars: Vec<usize>,
    pub duration: u64,
    pub version: usize,
    pub difficulty: usize,
    pub jdversion: isize,
    #[serde(borrow)]
    pub base: HipStr<'a>,
    #[serde(borrow)]
    pub app_avatars: HipStr<'a>,
    #[serde(borrow)]
    pub bkg_image: HipStr<'a>,
}

const fn default_num_coach() -> u8 {
    1
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SongDetails<'a> {
    #[serde(borrow)]
    pub map_name: HipStr<'a>,
    #[serde(rename = "JDVersion")]
    pub jd_version: isize,
    #[serde(rename = "OriginalJDVersion")]
    pub original_jd_version: u32,
    #[serde(borrow)]
    pub artist: HipStr<'a>,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(borrow)]
    pub credits: HipStr<'a>,
    #[serde(default = "default_num_coach")]
    pub num_coach: u8,
    #[serde(default)]
    pub count_in_progression: u32,
    #[serde(borrow, default)]
    pub dancer_name: HipStr<'a>,
    #[serde(default, rename = "LocaleID")]
    pub locale_id: u32,
    #[serde(default)]
    pub mojo_value: u32,
    #[serde(default)]
    pub mode: u32,
    #[serde(default)]
    pub status: u32,
    #[serde(default)]
    pub lyrics_type: isize,
    #[serde(default)]
    pub background_type: u32,
    pub difficulty: u32,
    #[serde(borrow)]
    pub default_colors: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow, rename = "lyricsColor")]
    pub lyrics_color: HipStr<'a>,
    #[serde(rename = "videoOffset")]
    pub video_offset: u32,
    #[serde(rename = "beats")]
    pub beats: Vec<u32>,
    #[serde(borrow, rename = "lyrics")]
    pub lyrics: Vec<Lyric<'a>>,
    #[serde(borrow, rename = "pictos")]
    pub pictos: Moves<'a>,
    pub audio_preview: AudioPreview,
    pub audio_preview_fade_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lyric<'a> {
    pub time: i32,
    pub duration: u32,
    #[serde(borrow)]
    pub text: HipStr<'a>,
    pub is_line_ending: u32,
}

pub type Moves<'a> = Vec<Picto<'a>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Picto<'a> {
    pub time: i32,
    pub duration: u32,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(default)]
    pub gold_move: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioPreview {
    pub coverflow: AudoPreviewBeats,
    pub prelobby: AudoPreviewBeats,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AudoPreviewBeats {
    pub startbeat: u32,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PictoAtlas<'a> {
    pub image_size: ImageSize,
    #[serde(borrow)]
    pub images: HashMap<HipStr<'a>, (u32, u32)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}
