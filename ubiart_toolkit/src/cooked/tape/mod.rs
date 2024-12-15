mod binary;
//mod new_json;

use std::collections::HashMap;

use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use hipstr::HipStr;
use ownable::IntoOwned;
use serde::{Deserialize, Serialize};
use ubiart_toolkit_shared_types::{errors::ParserError, Color};

use crate::{
    shared_json_types::Empty,
    utils::{Game, UniqueGameId},
};

pub fn parse(data: &[u8], ugi: UniqueGameId, lax: bool) -> Result<Tape<'_>, ParserError> {
    let tape = match ugi.game {
        game if game >= Game::JustDance2016 => crate::utils::json::parse(data, lax)?,
        Game::JustDance2015 => Tape::deserialize_with(data, ugi)?,
        _ => todo!(),
    };
    Ok(tape)
}

const fn default_tape_bar_count() -> u32 {
    1
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Tape<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub clips: Vec<Clip<'a>>,
    // Sometimes missing in mods
    #[serde(default)]
    pub tape_clock: u32,
    // Sometimes missing in mods
    #[serde(default = "default_tape_bar_count")]
    pub tape_bar_count: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub free_resources_after_play: u32,
    #[serde(borrow)]
    pub map_name: HipStr<'a>,
    /// Not present in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub soundwich_event: Option<HipStr<'a>>,
}

impl Tape<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("Tape");
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(tag = "__class")]
pub enum Clip<'a> {
    #[serde(borrow, rename = "ActorEnableClip")]
    ActorEnable(ActorEnableClip<'a>),
    #[serde(borrow, rename = "AlphaClip")]
    Alpha(AlphaClip<'a>),
    #[serde(borrow, rename = "CameraFeedClip")]
    CameraFeed(CameraFeedClip<'a>),
    #[serde(borrow, rename = "ColorClip")]
    Color(ColorClip<'a>),
    #[serde(borrow, rename = "CommunityDancerClip")]
    CommunityDancer(CommunityDancerClip<'a>),
    #[serde(borrow, rename = "FXClip")]
    FX(FXClip<'a>),
    #[serde(borrow, rename = "GameplayEventClip")]
    GameplayEvent(GameplayEventClip<'a>),
    #[serde(borrow, rename = "GoldEffectClip")]
    GoldEffect(GoldEffectClip<'a>),
    #[serde(borrow, rename = "HideUserInterfaceClip")]
    HideUserInterface(HideUserInterfaceClip<'a>),
    #[serde(borrow, rename = "KaraokeClip")]
    Karaoke(KaraokeClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicAlphaThresholdClip")]
    MaterialGraphicAlphaThreshold(MaterialGraphicDiffuseAlphaClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicDiffuseAlphaClip")]
    MaterialGraphicDiffuseAlpha(MaterialGraphicDiffuseAlphaClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicDiffuseColorClip")]
    MaterialGraphicDiffuseColor(MaterialGraphicDiffuseColorClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicEnableLayerClip")]
    MaterialGraphicEnableLayer(MaterialGraphicEnableLayerClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicUVAnimRotationClip")]
    MaterialGraphicUVAnimRotation(MaterialGraphicUVAnimRotationClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicUVRotationClip")]
    MaterialGraphicUVRotation(MaterialGraphicUVRotationClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicUVScaleClip")]
    MaterialGraphicUVScale(MaterialGraphicUVScaleClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicUVScrollClip")]
    MaterialGraphicUVScroll(MaterialGraphicUVScrollClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicUVTranslationClip")]
    MaterialGraphicUVTranslation(MaterialGraphicUVTranslationClip<'a>),
    #[serde(borrow, rename = "MotionClip")]
    Motion(MotionClip<'a>),
    #[serde(borrow, rename = "PictogramClip")]
    Pictogram(PictogramClip<'a>),
    #[serde(borrow, rename = "ProportionClip")]
    Proportion(ProportionClip<'a>),
    #[serde(borrow, rename = "Proportion3DClip")]
    Proportion3D(Proportion3DClip<'a>),
    #[serde(borrow, rename = "RotationClip")]
    Rotation(RotationClip<'a>),
    #[serde(borrow, rename = "SizeClip")]
    Size(SizeClip<'a>),
    #[serde(borrow, rename = "SlotClip")]
    Slot(SlotClip<'a>),
    #[serde(borrow, rename = "SpawnActorClip")]
    SpawnActor(SpawnActorClip<'a>),
    #[serde(borrow, rename = "SoundSetClip")]
    SoundSet(SoundSetClip<'a>),
    #[serde(borrow, rename = "SoundwichClip")]
    Soundwich(SoundwichClip<'a>),
    #[serde(borrow, rename = "SoundwichClipWithId")]
    SoundwichWithId(SoundwichClipWithId<'a>),
    #[serde(borrow, rename = "TapeLauncherClip")]
    TapeLauncher(TapeLauncherClip<'a>),
    #[serde(borrow, rename = "TapeReferenceClip")]
    TapeReference(TapeReferenceClip<'a>),
    #[serde(borrow, rename = "TextClip")]
    Text(TextClip<'a>),
    #[serde(borrow, rename = "TextAreaSizeClip")]
    TextAreaSize(TextAreaSizeClip<'a>),
    #[serde(borrow, rename = "TranslationClip")]
    Translation(TranslationClip<'a>),
    #[serde(borrow, rename = "VibrationClip")]
    Vibration(VibrationClip<'a>),
    #[serde(borrow, rename = "Unknown59FCC733Clip")]
    Unknown59FCC733(Unknown59FCC733Clip<'a>),
    #[serde(borrow, rename = "UnknownCBB7C029Clip")]
    UnknownCBB7C029(UnknownCBB7C029Clip<'a>),
    #[serde(borrow, rename = "Unknown5C944B01Clip")]
    Unknown5C944B01(Unknown5C944B01Clip<'a>),
}

const fn is_active_default() -> u8 {
    1
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ActorEnableClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub actor_enable: u32,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AlphaClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CameraFeedClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub capture_type: u32,
    pub record_beat: u32,
    pub feed_type: u32,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ColorClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_red: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_green: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_blue: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityDancerClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(borrow)]
    pub dancer_country_code: HipStr<'a>,
    pub dancer_avatar_id: u32,
    #[serde(borrow)]
    pub dancer_name: HipStr<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FXClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub fx_name: StringOrId<'a>,
    pub kill_particles_on_end: u32,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GameplayEventClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub event_type: u32,
    // Sometimes missing in mods
    #[serde(borrow, default)]
    pub custom_param: HipStr<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GoldEffectClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default)]
    pub effect_type: u8,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct HideUserInterfaceClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u32>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub event_type: u32,
    // Sometimes missing in mods
    #[serde(borrow, default)]
    pub custom_param: HipStr<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct KaraokeClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    pub pitch: f32,
    #[serde(borrow)]
    pub lyrics: HipStr<'a>,
    // Sometimes missing in mods
    #[serde(default)]
    pub is_end_of_line: u8,
    pub content_type: u32,
    pub start_time_tolerance: u32,
    pub end_time_tolerance: u32,
    pub semitone_tolerance: f32,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicDiffuseAlphaClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_a: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicDiffuseColorClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_r: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_g: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_b: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicEnableLayerClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub layer_enabled: u8,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVAnimRotationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_anim_angle: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_y: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVRotationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_angle: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_y: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVScaleClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_scale_u: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_scale_v: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_y: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVScrollClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_scroll_u: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_scroll_v: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVTranslationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_u: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_v: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MotionClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(borrow)]
    pub classifier_path: HipStr<'a>,
    pub gold_move: u8,
    // Sometimes CoachID (instead of CoachId) in mods
    #[serde(alias = "CoachID")]
    pub coach_id: u8,
    pub move_type: u8,
    pub color: Color,
    #[serde(borrow, default)]
    pub motion_platform_specifics: HashMap<HipStr<'a>, MotionPlatformSpecific<'a>>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MotionPlatformSpecific<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Sometimes missing in mods
    #[serde(default)]
    pub score_scale: f32,
    // Sometimes missing in mods
    #[serde(default)]
    pub score_smoothing: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoring_mode: Option<f32>,
    /// Not used in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low_threshold: Option<f32>,
    /// Not used in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_threshold: Option<f32>,
}

const fn default_coach_count() -> u32 {
    u32::MAX
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PictogramClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(borrow)]
    pub picto_path: HipStr<'a>,
    /// Only in nx2017-nx2018, only has non-empty values in nx2018
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub montage_path: Option<HipStr<'a>>,
    /// Only in nx2017-nx2018
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atl_index: Option<u32>,
    // Sometimes missing in mods
    #[serde(default = "default_coach_count")]
    pub coach_count: u32,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProportionClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_y: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Proportion3DClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_y: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_z: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RotationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_y: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_z: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SizeClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_y: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SlotClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    pub bpm: f32,
    #[serde(borrow)]
    pub signature: HipStr<'a>,
    #[serde(borrow)]
    pub guid: HipStr<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SpawnActorClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(borrow)]
    pub actor_path: HipStr<'a>,
    #[serde(borrow)]
    pub actor_name: HipStr<'a>,
    pub spawn_position: (f32, f32, f32),
    #[serde(borrow)]
    pub parent_actor: HipStr<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SoundSetClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(borrow)]
    pub sound_set_path: HipStr<'a>,
    pub sound_channel: i32,
    #[serde(default)]
    pub start_offset: u32,
    pub stops_on_end: u32,
    pub accounted_for_duration: u32,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SoundwichClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    /// Not present in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub soundwich_event: Option<HipStr<'a>>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SoundwichClipWithId<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    /// Not present in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub soundwich_event: Option<HipStr<'a>>,
    pub soundwich_id: i32,
}

#[allow(clippy::module_name_repetitions, reason = "Name is enforced by UbiArt")]
#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TapeLauncherClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub action: u32,
    /// Not in WiiU 2016
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tape_choice: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub tape_labels: Vec<HipStr<'a>>,
}

#[allow(
    clippy::module_name_repetitions,
    reason = "Name is required by the engine"
)]
#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TapeReferenceClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(borrow)]
    pub path: HipStr<'a>,
    #[serde(rename = "Loop")]
    pub loop_it: u32,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TextClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub localization_key: u32,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TextAreaSizeClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_max_width: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_max_height: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_area_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_area_y: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TranslationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_y: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_z: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct VibrationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(borrow)]
    pub vibration_file_path: HipStr<'a>,
    #[serde(rename = "Loop")]
    pub loop_it: u8,
    pub device_side: u8,
    pub player_id: i8,
    pub context: u32,
    pub start_time_offset: f32,
    pub modulation: f32,
}

impl VibrationClip<'_> {
    #[must_use]
    pub fn to_owned(self) -> VibrationClip<'static> {
        let class = None;
        let vibration_file_path = self.vibration_file_path.into_owned();
        VibrationClip {
            class,
            id: self.id,
            track_id: self.track_id,
            is_active: self.is_active,
            start_time: self.start_time,
            duration: self.duration,
            vibration_file_path,
            loop_it: self.loop_it,
            device_side: self.device_side,
            player_id: self.player_id,
            context: self.context,
            start_time_offset: self.start_time_offset,
            modulation: self.modulation,
        }
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Unknown59FCC733Clip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_one: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_two: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_three: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_four: BezierCurveFloat<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnknownCBB7C029Clip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub unk2_stringid: u32,
    pub unk3: u32,
    pub unk4: f32,
    pub unk5: u32,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Unknown5C944B01Clip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    // Sometimes missing in mods
    #[serde(default)]
    pub track_id: u32,
    // Sometimes missing in mods
    #[serde(default = "is_active_default")]
    pub is_active: u8,
    // Sometimes a float in mods
    #[serde(deserialize_with = "crate::utils::json::deserialize_f32_or_i32")]
    pub start_time: i32,
    pub duration: i32,
    pub string: HipStr<'a>,
}

pub struct Unknown9BC67FC4;

#[derive(IntoOwned, Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloat<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "Curve",
        deserialize_with = "deser_bezier_curve_float_value"
    )]
    pub value: BezierCurveFloatValue<'a>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BezierCurveFloatValue<'a> {
    #[serde(borrow)]
    Empty(Empty<'a>),
    #[serde(borrow, rename = "BezierCurveFloatConstant")]
    Constant(BezierCurveFloatConstant<'a>),
    #[serde(borrow, rename = "BezierCurveFloatLinear")]
    Linear(BezierCurveFloatLinear<'a>),
    #[serde(borrow, rename = "BezierCurveFloatMulti")]
    Multi(BezierCurveFloatMulti<'a>),
}

impl Default for BezierCurveFloatValue<'_> {
    fn default() -> Self {
        BezierCurveFloatValue::Empty(Empty::default())
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatConstant<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub value: f32,
}

impl Default for BezierCurveFloatConstant<'static> {
    fn default() -> Self {
        Self {
            class: Some(HipStr::borrowed("BezierCurveFloatConstant")),
            value: Default::default(),
        }
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatLinear<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub value_left: (f32, f32),
    pub normal_left_out: (f32, f32),
    pub value_right: (f32, f32),
    pub normal_right_in: (f32, f32),
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatMulti<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub keys: Vec<KeyFloat<'a>>,
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct KeyFloat<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub value: (f32, f32),
    pub normal_in: (f32, f32),
    pub normal_out: (f32, f32),
}

/// Deserialize a [`BezierCurveFloatValue`] which is weirdly formattend in the JSON files
fn deser_bezier_curve_float_value<'de, D>(deser: D) -> Result<BezierCurveFloatValue<'de>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let bcfv: Result<BezierCurveFloatValue, D::Error> = Deserialize::deserialize(deser);
    match bcfv {
        Ok(something) => Ok(something),
        Err(err) => {
            if err.to_string().as_str() == "missing field `__class`" {
                Ok(BezierCurveFloatValue::Empty(Empty::default()))
            } else {
                Err(err)
            }
        }
    }
}

#[derive(Debug, IntoOwned, Serialize, Deserialize)]
pub struct TargetActor<'a> {
    #[serde(borrow)]
    pub path: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub scenes: Vec<HipStr<'a>>,
}

#[derive(IntoOwned, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrId<'a> {
    #[serde(borrow)]
    String(HipStr<'a>),
    Id(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_JSON: &str = r#"{"__class": "BezierCurveFloat","Curve": {}}"#;
    #[test]
    fn test_empty_bezier_curve_float_value() {
        let curve: BezierCurveFloat = serde_json::from_str(EMPTY_JSON).unwrap();
        assert!(matches!(curve.value, BezierCurveFloatValue::Empty(_)));
    }

    const CONSTANT_JSON: &str = r#"{"__class":"BezierCurveFloat","Curve":{"__class":"BezierCurveFloatConstant","Value":0}}"#;
    #[test]
    fn test_constant_bezier_curve_float_value() {
        let curve: BezierCurveFloat = serde_json::from_str(CONSTANT_JSON).unwrap();
        assert!(matches!(curve.value, BezierCurveFloatValue::Constant(_)));
    }

    const MIXED_JSON: &str = r#"[{"__class": "BezierCurveFloat","Curve": {}},{"__class":"BezierCurveFloat","Curve":{"__class":"BezierCurveFloatConstant","Value":0}}]"#;
    #[test]
    fn test_mix_bezier_curve_float_value() {
        let _: Vec<BezierCurveFloat> = serde_json::from_str(MIXED_JSON).unwrap();
    }
}
