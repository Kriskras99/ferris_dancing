//! Contains descriptions of Just Dance types
use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use ubiart_toolkit_shared_types::{Color, LocaleId};

use super::{isg::AutodanceVideoStructure, PhoneImages};

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AgingBotBehaviourAllTrees<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub treelist: Vec<AgingBotBehaviourTree<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AgingBotBehaviourTree<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub name: Cow<'a, str>,
    #[serde(rename = "BehaviourGroups")]
    pub behaviour_groups: Vec<AgingBotBehaviourGroup<'a>>,
    pub success_if_log_contains: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AgingBotBehaviourGroup<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub related_gamescreen: Cow<'a, str>,
    #[serde(rename = "BehaviourTypeList")]
    pub behaviour_type_list: Vec<BehaviourType<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "__class")]
pub enum BehaviourType<'a> {
    #[serde(borrow, rename = "JD_AgingBot_BehaviourTypes")]
    Base(BehaviourTypeBase<'a>),
    #[serde(borrow, rename = "JD_AgingBot_BehaviourTypes_PageValidation")]
    PageValidation(BehaviourTypeBase<'a>),
    #[serde(borrow, rename = "JD_AgingBot_BehaviourTypes_SpecificCarrouselItem")]
    SpecificCarrouselItem(BehaviourTypeSpecificCarrouselItem<'a>),
    #[serde(borrow, rename = "JD_AgingBot_BehaviourTypes_ExpositionMove")]
    ExpositionMove(BehaviourTypeBase<'a>),
    #[serde(borrow, rename = "JD_AgingBot_BehaviourTypes_AutoEndMap")]
    AutoEndMap(BehaviourTypeAutoEndMap<'a>),
    #[serde(borrow, rename = "JD_AgingBot_BehaviourTypes_TraverseCarouselRows")]
    TraverseCarouselRows(BehaviourTypeBase<'a>),
    #[serde(borrow, rename = "JD_AgingBot_BehaviourTypes_EachCarouselItemInRow")]
    EachCarouselItemInRow(BehaviourTypeBase<'a>),
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BehaviourTypeBase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    #[serde(rename = "type")]
    pub typed: u32,
    pub connection_name: Cow<'a, str>,
    pub control_name: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BehaviourTypeSpecificCarrouselItem<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    #[serde(rename = "type")]
    pub typed: u32,
    pub connection_name: Cow<'a, str>,
    pub control_name: Cow<'a, str>,
    pub item_index: u32,
    #[serde(rename = "ItemIndexList", default)]
    pub item_index_list: Vec<u32>,
    #[serde(rename = "BackEveryNCalls")]
    pub back_every_n_calls: u32,
    // Not on WiiU
    #[serde(
        rename = "ProfileEveryNCalls",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_every_n_calls: Option<u32>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BehaviourTypeAutoEndMap<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    #[serde(rename = "type")]
    pub typed: u32,
    pub connection_name: Cow<'a, str>,
    pub control_name: Cow<'a, str>,
    #[serde(rename = "NumberOfMapsBeforeAutoEnd")]
    pub number_of_maps_before_auto_end: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AutodanceComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub song: Cow<'a, str>,
    pub autodance_data: AutodanceData<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct AutodanceData<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub recording_structure: AutodanceRecordingStructure<'a>,
    pub video_structure: AutodanceVideoStructure<'a>,
    #[serde(rename = "autodanceSoundPath")]
    pub autodance_sound_path: Cow<'a, str>,
}

impl AutodanceData<'_> {
    pub const CLASS: &'static str = "JD_AutodanceData";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AutodanceRecordingStructure<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub records: Vec<Record<'a>>,
}

impl AutodanceRecordingStructure<'_> {
    pub const CLASS: &'static str = "JD_AutodanceRecordingStructure";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AutoDanceFxDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub opacity: u32,
    pub color_low: GFXVector4<'a>,
    pub color_mid: GFXVector4<'a>,
    pub color_high: GFXVector4<'a>,
    pub low_to_mid: f32,
    pub low_to_mid_width: f32,
    pub mid_to_high: f32,
    pub mid_to_high_width: f32,
    pub sob_color: GFXVector4<'a>,
    pub out_color: GFXVector4<'a>,
    pub thick_middle: f32,
    pub thick_inner: f32,
    pub thick_smooth: f32,
    pub shv_nb_frames: u32,
    pub parts_scale: (u32, u32, u32, u32, u32),
    pub halftone_factor: u32,
    pub halftone_cutout_levels: u32,
    #[serde(rename = "UVBlackoutFactor")]
    pub uv_blackout_factor: u32,
    #[serde(rename = "UVBlackoutDesaturation")]
    pub uv_blackout_desaturation: f32,
    #[serde(rename = "UVBlackoutContrast")]
    pub uv_blackout_contrast: u32,
    #[serde(rename = "UVBlackoutBrightness")]
    pub uv_blackout_brightness: u32,
    #[serde(rename = "UVBlackoutColor")]
    pub uv_blackout_color: GFXVector4<'a>,
    pub toon_factor: u32,
    pub toon_cutout_levels: u32,
    pub refraction_factor: u32,
    pub refraction_tint: GFXVector4<'a>,
    pub refraction_scale: GFXVector4<'a>,
    pub refraction_opacity: f32,
    pub colored_shiva_thresholds: GFXVector4<'a>,
    pub colored_shiva_color_0: GFXVector4<'a>,
    pub colored_shiva_color_1: GFXVector4<'a>,
    pub colored_shiva_color_2: GFXVector4<'a>,
    pub saturation_modifier: f32,
    pub slime_factor: f32,
    pub slime_color: GFXVector4<'a>,
    pub slime_opacity: f32,
    pub slime_ambient: f32,
    pub slime_normal_tiling: u32,
    pub slime_light_angle: f32,
    pub slime_refraction: f32,
    pub slime_refraction_index: f32,
    pub slime_specular: f32,
    pub slime_specular_power: u32,
    pub overlay_blend_factor: f32,
    pub overlay_blend_color: GFXVector4<'a>,
    pub background_sobel_factor: f32,
    pub background_sobel_color: GFXVector4<'a>,
    pub player_glow_factor: f32,
    pub player_glow_color: GFXVector4<'a>,
    pub swap_head_with_player: (u32, u32, u32, u32, u32, u32),
    pub animate_player_head: (u32, u32, u32, u32, u32, u32),
    pub animated_head_total_time: u32,
    pub animated_head_rest_time: u32,
    pub animated_head_frame_time: f32,
    pub animated_head_max_distance: f32,
    pub animated_head_max_angle: f32,
    pub screen_blend_inverse_alpha_factor: u32,
    pub screen_blend_inverse_alpha_scale_x: u32,
    pub screen_blend_inverse_alpha_scale_y: u32,
    pub screen_blend_inverse_alpha_trans_x: u32,
    pub screen_blend_inverse_alpha_trans_y: u32,
    pub tint_mul_color_factor: u32,
    pub tint_mul_color: GFXVector4<'a>,
    pub floor_plane_factor: u32,
    pub floor_plane_tiles: GFXVector4<'a>,
    pub floor_speed_x: f32,
    pub floor_speed_y: f32,
    pub floor_wave_speed: f32,
    pub floor_blend_mode: u32,
    #[serde(rename = "FloorPlaneImageID")]
    pub floor_plane_image_id: u32,
    pub start_radius: f32,
    pub end_radius: f32,
    pub radius_variance: f32,
    pub radius_noise_rate: u32,
    pub radius_noise_amp: f32,
    pub min_spin: f32,
    pub max_spin: f32,
    pub dir_angle: f32,
    pub min_wander_rate: f32,
    pub max_wander_rate: f32,
    pub min_wander_amp: f32,
    pub max_wander_amp: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub motion_power: f32,
    pub amount: f32,
    #[serde(rename = "ImageID")]
    pub image_id: u32,
    pub start_r: f32,
    pub start_g: f32,
    pub start_b: f32,
    pub end_r: f32,
    pub end_g: f32,
    pub end_b: f32,
    pub start_alpha: f32,
    pub end_alpha: f32,
    pub textured_outline_factor: u32,
    pub textured_outline_tiling: u32,
    pub triple_layer_background_factor: u32,
    pub triple_layer_background_tint_color: GFXVector4<'a>,
    pub triple_layer_background_speed_x: u32,
    pub triple_layer_background_speed_y: u32,
    pub trail_effect_id: u32,
}

impl AutoDanceFxDesc<'_> {
    pub const CLASS: &'static str = "AutoDanceFxDesc";
}

impl Default for AutoDanceFxDesc<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            opacity: 1,
            color_low: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            color_mid: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            color_high: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            low_to_mid: 0.333,
            low_to_mid_width: 0.15,
            mid_to_high: 0.666,
            mid_to_high_width: 0.15,
            sob_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            out_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            thick_middle: 0.4,
            thick_inner: 0.1,
            thick_smooth: 0.1,
            shv_nb_frames: 0,
            parts_scale: (0, 0, 0, 0, 0),
            halftone_factor: 0,
            halftone_cutout_levels: 256,
            uv_blackout_factor: 0,
            uv_blackout_desaturation: 0.2,
            uv_blackout_contrast: 4,
            uv_blackout_brightness: 0,
            uv_blackout_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.54902,
                y: 0.54902,
                z: 1.0,
                w: 1.0,
            },
            toon_factor: 0,
            toon_cutout_levels: 256,
            refraction_factor: 0,
            refraction_tint: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            refraction_scale: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.03,
                y: 0.03,
                z: 0.03,
                w: 0.03,
            },
            refraction_opacity: 0.2,
            colored_shiva_thresholds: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.1,
                y: 0.3,
                z: 0.6,
                w: 0.95,
            },
            colored_shiva_color_0: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            colored_shiva_color_1: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            colored_shiva_color_2: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            saturation_modifier: 0.0,
            slime_factor: 0.0,
            slime_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.49902,
                y: 0.629_176,
                z: 0.136_039,
                w: 1.0,
            },
            slime_opacity: 0.2,
            slime_ambient: 0.2,
            slime_normal_tiling: 7,
            slime_light_angle: 0.0,
            slime_refraction: 0.0913,
            slime_refraction_index: 1.0837,
            slime_specular: 1.0,
            slime_specular_power: 10,
            overlay_blend_factor: 0.0,
            overlay_blend_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.721_569,
                y: 0.639_216,
                z: 0.756_863,
                w: 1.0,
            },
            background_sobel_factor: 0.0,
            background_sobel_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            player_glow_factor: 0.0,
            player_glow_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            swap_head_with_player: (0, 1, 2, 3, 4, 5),
            animate_player_head: (0, 0, 0, 0, 0, 0),
            animated_head_total_time: 20,
            animated_head_rest_time: 16,
            animated_head_frame_time: 0.6,
            animated_head_max_distance: 1.25,
            animated_head_max_angle: 1.2,
            screen_blend_inverse_alpha_factor: 0,
            screen_blend_inverse_alpha_scale_x: 0,
            screen_blend_inverse_alpha_scale_y: 0,
            screen_blend_inverse_alpha_trans_x: 0,
            screen_blend_inverse_alpha_trans_y: 0,
            tint_mul_color_factor: 0,
            tint_mul_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            floor_plane_factor: 0,
            floor_plane_tiles: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 8.0,
                y: 8.0,
                z: 0.0,
                w: 0.0,
            },
            floor_speed_x: 0.0,
            floor_speed_y: 0.0,
            floor_wave_speed: 0.0,
            floor_blend_mode: 0,
            floor_plane_image_id: 0,
            start_radius: 3.0,
            end_radius: 2.0,
            radius_variance: 0.5,
            radius_noise_rate: 0,
            radius_noise_amp: 0.0,
            min_spin: -4.0,
            max_spin: 4.0,
            dir_angle: 0.0,
            min_wander_rate: 2.0,
            max_wander_rate: 3.0,
            min_wander_amp: 0.1,
            max_wander_amp: 0.2,
            min_speed: 0.2,
            max_speed: 0.4,
            motion_power: 1.5,
            amount: 0.0,
            image_id: 7,
            start_r: 1.0,
            start_g: 0.1,
            start_b: 0.1,
            end_r: 0.1,
            end_g: 0.2,
            end_b: 1.0,
            start_alpha: 1.0,
            end_alpha: 1.0,
            textured_outline_factor: 0,
            textured_outline_tiling: 1,
            triple_layer_background_factor: 0,
            triple_layer_background_tint_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            triple_layer_background_speed_x: 0,
            triple_layer_background_speed_y: 0,
            trail_effect_id: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct GFXVector4<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl GFXVector4<'_> {
    pub const CLASS: &'static str = "GFX_Vector4";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PlaybackEvent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub clip_number: u32,
    pub start_clip: f32,
    pub start_time: f32,
    pub duration: f32,
    pub speed: f32,
}

impl PlaybackEvent<'_> {
    pub const CLASS: &'static str = "PlaybackEvent";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Record<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub start: f32,
    pub duration: f32,
}

impl Record<'_> {
    pub const CLASS: &'static str = "Record";
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FixedCameraComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub start_as_main_cam: u32,
    pub ramp_up_coeff: u32,
    pub focale: f32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SkinDescription<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub jd_version: u32,
    pub relative_song_name: Cow<'a, str>,
    #[serde(rename = "RelativeQuestID")]
    pub relative_quest_id: Cow<'a, str>,
    #[serde(rename = "RelativeWDFBossName")]
    pub relative_wdf_boss_name: Cow<'a, str>,
    #[serde(rename = "RelativeWDFTournamentName")]
    pub relative_wdf_tournament_name: Cow<'a, str>,
    #[serde(rename = "RelativeJDRank")]
    pub relative_jd_rank: Cow<'a, str>,
    pub relative_game_mode_name: Cow<'a, str>,
    pub sound_family: Cow<'a, str>,
    pub status: u32,
    pub unlock_type: u32,
    pub mojo_price: u32,
    pub wdf_level: u32,
    pub count_in_progression: u32,
    pub actor_path: Cow<'a, str>,
    pub phone_image: Cow<'a, str>,
    #[serde(rename = "skinId")]
    pub skin_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SongDatabase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub path_creation_formats: PathCreationFormat<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PathCreationFormat<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub timeline: Cow<'a, str>,
    // In WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cine_mashup: Option<Cow<'a, str>>,
    // Only in 2018
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub videowebm: Option<Cow<'a, str>>,
    pub cine: Cow<'a, str>,
    pub graph: Cow<'a, str>,
    // In WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mainscene: Option<Cow<'a, str>>,
    pub video: Cow<'a, str>,
    pub videoalpha: Cow<'a, str>,
    // Not in WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub videompd: Option<Cow<'a, str>>,
    // Not in WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub videomappreview: Option<Cow<'a, str>>,
    // In WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cine_partymaster: Option<Cow<'a, str>>,
    pub audio: Cow<'a, str>,
    // Can be all caps on WiiU
    #[serde(alias = "FX")]
    pub fx: Cow<'a, str>,
    // Can be all caps on WiiU
    #[serde(alias = "AUTODANCE")]
    pub autodance: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SongDescription<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub map_name: Cow<'a, str>,
    #[serde(rename = "JDVersion")]
    pub jd_version: u16,
    #[serde(rename = "OriginalJDVersion")]
    pub original_jd_version: u16,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_albums: Vec<Cow<'a, str>>,
    pub artist: Cow<'a, str>,
    /// Only in Chinese version
    #[serde(rename = "CN_Lyrics", default, skip_serializing_if = "Option::is_none")]
    pub cn_lyrics: Option<Cow<'a, str>>,
    pub dancer_name: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub credits: Cow<'a, str>,
    /// Only in Chinese version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_title: Option<Cow<'a, str>>,
    /// Only in Chinese version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_credits: Option<Cow<'a, str>>,
    pub phone_images: PhoneImages<'a>,
    pub num_coach: u8,
    pub main_coach: i8,
    /// Only in versions before nx2020
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_scoring_type: Option<i8>,
    pub difficulty: u8,
    #[serde(default = "default_sweat_difficulty")]
    pub sweat_difficulty: u8,
    #[serde(rename = "backgroundType")]
    pub background_type: u8,
    pub lyrics_type: i8,
    /// Only in nx2017, always 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<u8>,
    pub tags: Vec<Cow<'a, str>>,
    /// Only in nx2017
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jdm_attributes: Option<Vec<Cow<'a, str>>>,
    pub status: u8,
    #[serde(rename = "LocaleID")]
    pub locale_id: LocaleId,
    pub mojo_value: u16,
    pub count_in_progression: u32,
    pub default_colors: DefaultColors,
    /// Not present in nx2017
    #[serde(default)]
    pub video_preview_path: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_with_both_controllers: Option<usize>,
    /// Only in versions before nx2020
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Paths>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Paths {
    #[serde(alias = "AsyncPlayers")]
    pub asyncplayers: Option<Vec<String>>,
    #[serde(alias = "Avatars")]
    pub avatars: Option<Vec<String>>,
}

const fn default_sweat_difficulty() -> u8 {
    1
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DefaultColors {
    pub theme: Color,
    pub lyrics: Color,
    #[serde(alias = "songColor_1A", skip_serializing_if = "Option::is_none")]
    pub songcolor_1a: Option<Color>,
    #[serde(alias = "songColor_1B", skip_serializing_if = "Option::is_none")]
    pub songcolor_1b: Option<Color>,
    #[serde(alias = "songColor_2A", skip_serializing_if = "Option::is_none")]
    pub songcolor_2a: Option<Color>,
    #[serde(alias = "songColor_2B", skip_serializing_if = "Option::is_none")]
    pub songcolor_2b: Option<Color>,
}

impl SongDescription<'_> {
    pub const CLASS: &'static str = "JD_SongDescTemplate";
}
