use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Tape<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clips: Vec<Clip<'a>>,
    pub tape_clock: u32,
    pub tape_bar_count: u32,
    pub free_resources_after_play: u32,
    pub map_name: Cow<'a, str>,
    /// Not present in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub soundwich_event: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_paths: Vec<Cow<'a, str>>,
}

impl Tape<'_> {
    pub const CLASS: &'static str = "Tape";
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "__class")]
pub enum Clip<'a> {
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ActorEnableClip")]
    ActorEnable(ActorEnableClip<'a>),
    #[serde(borrow, rename = "AlphaClip")]
    Alpha(AlphaClip<'a>),
    #[serde(borrow, rename = "ColorClip")]
    Color(ColorClip<'a>),
    #[cfg(feature = "full_json_types")]
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
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "MaterialGraphicDiffuseAlphaClip")]
    MaterialGraphicDiffuseAlpha(MaterialGraphicDiffuseAlphaClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "MaterialGraphicDiffuseColorClip")]
    MaterialGraphicDiffuseColor(MaterialGraphicDiffuseColorClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicEnableLayerClip")]
    MaterialGraphicEnableLayer(MaterialGraphicEnableLayerClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "MaterialGraphicUVAnimRotationClip")]
    MaterialGraphicUVAnimRotation(MaterialGraphicUVAnimRotationClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "MaterialGraphicUVRotationClip")]
    MaterialGraphicUVRotation(MaterialGraphicUVRotationClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "MaterialGraphicUVScaleClip")]
    MaterialGraphicUVScale(MaterialGraphicUVScaleClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "MaterialGraphicUVScrollClip")]
    MaterialGraphicUVScroll(MaterialGraphicUVScrollClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "MaterialGraphicUVTranslationClip")]
    MaterialGraphicUVTranslation(MaterialGraphicUVTranslationClip<'a>),
    #[serde(borrow, rename = "MotionClip")]
    Motion(MotionClip<'a>),
    #[serde(borrow, rename = "PictogramClip")]
    Pictogram(PictogramClip<'a>),
    #[serde(borrow, rename = "ProportionClip")]
    Proportion(ProportionClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "Proportion3DClip")]
    Proportion3D(Proportion3DClip<'a>),
    #[serde(borrow, rename = "RotationClip")]
    Rotation(RotationClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "SizeClip")]
    Size(SizeClip<'a>),
    #[serde(borrow, rename = "SoundSetClip")]
    SoundSet(SoundSetClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "SoundwichClip")]
    Soundwich(SoundwichClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TapeLauncherClip")]
    TapeLauncher(TapeLauncherClip<'a>),
    #[serde(borrow, rename = "TapeReferenceClip")]
    TapeReference(TapeReferenceClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TextClip")]
    Text(TextClip<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TextAreaSizeClip")]
    TextAreaSize(TextAreaSizeClip<'a>),
    #[serde(borrow, rename = "TranslationClip")]
    Translation(TranslationClip<'a>),
    #[serde(borrow, rename = "VibrationClip")]
    Vibration(VibrationClip<'a>),
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Proportion3DClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_x: Curve<'a>,
    pub curve_y: Curve<'a>,
    pub curve_z: Curve<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TextAreaSizeClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_max_width: Curve<'a>,
    pub curve_max_height: Curve<'a>,
    pub curve_area_x: Curve<'a>,
    pub curve_area_y: Curve<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SoundwichClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    /// Not present in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub soundwich_event: Option<Cow<'a, str>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GameplayEventClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub event_type: u32,
    pub custom_param: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SizeClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_x: Curve<'a>,
    pub curve_y: Curve<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TextClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub localization_key: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PictogramClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub picto_path: Cow<'a, str>,
    /// Not used in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub montage_path: Option<Cow<'a, str>>,
    /// Not used in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atl_index: Option<u32>,
    pub coach_count: i64,
}

pub type Color = (f32, f32, f32, f32);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MotionClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub classifier_path: Cow<'a, str>,
    pub gold_move: u8,
    pub coach_id: u8,
    pub move_type: u8,
    pub color: Color,
    #[serde(default)]
    pub motion_platform_specifics: HashMap<Cow<'a, str>, MotionPlatformSpecific<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MotionPlatformSpecific<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub score_scale: f32,
    pub score_smoothing: u32,
    /// Not used in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low_threshold: Option<f32>,
    /// Not used in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_threshold: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GoldEffectClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default)]
    pub effect_type: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct KaraokeClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub pitch: f32,
    pub lyrics: Cow<'a, str>,
    pub is_end_of_line: u8,
    pub content_type: u8,
    pub start_time_tolerance: u8,
    pub end_time_tolerance: u8,
    pub semitone_tolerance: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SoundSetClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub sound_set_path: Cow<'a, str>,
    pub sound_channel: i32,
    #[serde(default)]
    pub start_offset: u32,
    pub stops_on_end: u32,
    pub accounted_for_duration: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct VibrationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub vibration_file_path: Cow<'a, str>,
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
        let vibration_file_path = Cow::Owned(self.vibration_file_path.into_owned());
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct HideUserInterfaceClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub event_type: u32,
    pub custom_param: Cow<'a, str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AlphaClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve: Curve<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RotationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_x: Curve<'a>,
    pub curve_y: Curve<'a>,
    pub curve_z: Curve<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TranslationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_x: Curve<'a>,
    pub curve_y: Curve<'a>,
    pub curve_z: Curve<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ActorEnableClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub actor_enable: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProportionClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_x: Curve<'a>,
    pub curve_y: Curve<'a>,
}

#[cfg(feature = "full_json_types")]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TapeLauncherClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub action: u32,
    pub tape_choice: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tape_labels: Vec<Cow<'a, str>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FXClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub fx_name: Cow<'a, str>,
    pub kill_particles_on_end: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ColorClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_red: Curve<'a>,
    pub curve_green: Curve<'a>,
    pub curve_blue: Curve<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicDiffuseColorClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_r: Curve<'a>,
    pub curve_g: Curve<'a>,
    pub curve_b: Curve<'a>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TapeReferenceClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub path: Cow<'a, str>,
    #[serde(rename = "Loop")]
    pub loop_it: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVScaleClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_scale_u: Curve<'a>,
    pub curve_scale_v: Curve<'a>,
    pub curve_pivot_x: Curve<'a>,
    pub curve_pivot_y: Curve<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicDiffuseAlphaClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_a: Curve<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVTranslationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_u: Curve<'a>,
    pub curve_v: Curve<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicEnableLayerClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u8,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u8,
    pub layer_enabled: u8,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVAnimRotationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_anim_angle: Curve<'a>,
    pub curve_pivot_x: Curve<'a>,
    pub curve_pivot_y: Curve<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVRotationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_angle: Curve<'a>,
    pub curve_pivot_x: Curve<'a>,
    pub curve_pivot_y: Curve<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVScrollClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_scroll_u: Curve<'a>,
    pub curve_scroll_v: Curve<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "__class")]
pub enum Curve<'a> {
    #[serde(borrow)]
    BezierCurveFloat(BezierCurveFloat<'a>),
}

impl Default for Curve<'static> {
    fn default() -> Self {
        Self::BezierCurveFloat(BezierCurveFloat::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloat<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "Curve", deserialize_with = "deser_bezier_curve_float_value")]
    pub value: BezierCurveFloatValue<'a>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(tag = "__class")]
pub enum BezierCurveFloatValue<'a> {
    #[default]
    Empty,
    #[serde(borrow, rename = "BezierCurveFloatConstant")]
    Constant(BezierCurveFloatConstant<'a>),
    #[serde(borrow, rename = "BezierCurveFloatLinear")]
    Linear(BezierCurveFloatLinear<'a>),
    #[serde(borrow, rename = "BezierCurveFloatMulti")]
    Multi(BezierCurveFloatMulti<'a>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatConstant<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub value: f32,
}

impl Default for BezierCurveFloatConstant<'static> {
    fn default() -> Self {
        Self {
            class: Some("BezierCurveFloatConstant"),
            value: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatLinear<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub value_left: (f32, f32),
    pub normal_left_out: (f32, f32),
    pub value_right: (f32, f32),
    pub normal_right_in: (f32, f32),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatMulti<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub keys: Vec<KeyFloat<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct KeyFloat<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub value: (f32, f32),
    pub normal_in: (f32, f32),
    pub normal_out: (f32, f32),
}

fn deser_bezier_curve_float_value<'de, D>(deser: D) -> Result<BezierCurveFloatValue<'de>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let bcfv: Result<BezierCurveFloatValue, D::Error> = Deserialize::deserialize(deser);
    match bcfv {
        Ok(something) => Ok(something),
        Err(err) => {
            if err.to_string().as_str() == "missing field `__class`" {
                Ok(BezierCurveFloatValue::Empty)
            } else {
                Err(err)
            }
        }
    }
}
