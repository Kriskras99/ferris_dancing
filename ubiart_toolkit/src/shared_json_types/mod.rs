use hipstr::HipStr;
use ownable::IntoOwned;
use serde::{Deserialize, Serialize};
use ubiart_toolkit_shared_types::LocaleId;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AutodancePropData<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub index: u32,
    pub pivot_x: f32,
    pub pivot_y: f32,
    pub size: f32,
    /// Not in 2016
    #[serde(
        rename = "fx_assetID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fx_asset_id: Option<HipStr<'a>>,
    pub prop_part: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AutodanceVideoStructure<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    /// Only in 2016
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<u32>,
    pub song_start_position: f32,
    pub duration: f32,
    pub thumbnail_time: u32,
    pub fade_out_duration: f32,
    /// Only in 2016
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub animated_frame_path: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub ground_plane_path: HipStr<'a>,
    #[serde(borrow)]
    pub first_layer_triple_background_path: HipStr<'a>,
    #[serde(borrow)]
    pub second_layer_triple_background_path: HipStr<'a>,
    #[serde(borrow)]
    pub third_layer_triple_background_path: HipStr<'a>,
    #[serde(
        borrow,
        rename = "playback_events",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub playback_events: Vec<PlaybackEvent<'a>>,
    #[serde(borrow, rename = "background_effect")]
    pub background_effect: Box<AutoDanceFxDesc<'a>>,
    #[serde(
        borrow,
        rename = "background_effect_events",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub background_effect_events: Vec<FxEvent<'a>>,
    #[serde(borrow, rename = "player_effect")]
    pub player_effect: Box<AutoDanceFxDesc<'a>>,
    #[serde(
        borrow,
        rename = "player_effect_events",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub player_effect_events: Vec<FxEvent<'a>>,
    #[serde(
        borrow,
        rename = "prop_events",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prop_events: Vec<PropEvent<'a>>,
    #[serde(
        borrow,
        rename = "props",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub props: Vec<AutodancePropData<'a>>,
    #[serde(
        borrow,
        rename = "props_players_config",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub props_players_config: Vec<PropPlayerConfig<'a>>,
}

impl AutodanceVideoStructure<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_AutodanceVideoStructure");
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AutoDanceFxDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub opacity: f32,
    #[serde(borrow)]
    pub color_low: GFXVector4<'a>,
    #[serde(borrow)]
    pub color_mid: GFXVector4<'a>,
    #[serde(borrow)]
    pub color_high: GFXVector4<'a>,
    pub low_to_mid: f32,
    pub low_to_mid_width: f32,
    pub mid_to_high: f32,
    pub mid_to_high_width: f32,
    #[serde(borrow)]
    pub sob_color: GFXVector4<'a>,
    #[serde(borrow)]
    pub out_color: GFXVector4<'a>,
    pub thick_middle: f32,
    pub thick_inner: f32,
    pub thick_smooth: f32,
    pub shv_nb_frames: u32,
    pub parts_scale: Vec<u32>,
    pub halftone_factor: u32,
    pub halftone_cutout_levels: f32,
    #[serde(rename = "UVBlackoutFactor")]
    pub uv_blackout_factor: u32,
    #[serde(rename = "UVBlackoutDesaturation")]
    pub uv_blackout_desaturation: f32,
    #[serde(rename = "UVBlackoutContrast")]
    pub uv_blackout_contrast: f32,
    #[serde(rename = "UVBlackoutBrightness")]
    pub uv_blackout_brightness: u32,
    #[serde(borrow, rename = "UVBlackoutColor")]
    pub uv_blackout_color: GFXVector4<'a>,
    pub toon_factor: f32,
    pub toon_cutout_levels: f32,
    pub refraction_factor: u32,
    #[serde(borrow)]
    pub refraction_tint: GFXVector4<'a>,
    #[serde(borrow)]
    pub refraction_scale: GFXVector4<'a>,
    pub refraction_opacity: f32,
    #[serde(borrow)]
    pub colored_shiva_thresholds: GFXVector4<'a>,
    #[serde(borrow)]
    pub colored_shiva_color_0: GFXVector4<'a>,
    #[serde(borrow)]
    pub colored_shiva_color_1: GFXVector4<'a>,
    #[serde(borrow)]
    pub colored_shiva_color_2: GFXVector4<'a>,
    pub saturation_modifier: f32,
    pub slime_factor: f32,
    #[serde(borrow)]
    pub slime_color: GFXVector4<'a>,
    pub slime_opacity: f32,
    pub slime_ambient: f32,
    pub slime_normal_tiling: f32,
    pub slime_light_angle: f32,
    pub slime_refraction: f32,
    pub slime_refraction_index: f32,
    pub slime_specular: f32,
    pub slime_specular_power: f32,
    pub overlay_blend_factor: f32,
    #[serde(borrow)]
    pub overlay_blend_color: GFXVector4<'a>,
    pub background_sobel_factor: f32,
    #[serde(borrow)]
    pub background_sobel_color: GFXVector4<'a>,
    pub player_glow_factor: f32,
    #[serde(borrow)]
    pub player_glow_color: GFXVector4<'a>,
    pub swap_head_with_player: Vec<u32>,
    pub animate_player_head: Vec<u32>,
    pub animated_head_total_time: f32,
    pub animated_head_rest_time: f32,
    pub animated_head_frame_time: f32,
    pub animated_head_max_distance: f32,
    pub animated_head_max_angle: f32,
    pub screen_blend_inverse_alpha_factor: u32,
    pub screen_blend_inverse_alpha_scale_x: f32,
    pub screen_blend_inverse_alpha_scale_y: f32,
    pub screen_blend_inverse_alpha_trans_x: u32,
    pub screen_blend_inverse_alpha_trans_y: u32,
    pub tint_mul_color_factor: u32,
    #[serde(borrow)]
    pub tint_mul_color: GFXVector4<'a>,
    pub floor_plane_factor: f32,
    #[serde(borrow)]
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
    pub textured_outline_tiling: f32,
    pub triple_layer_background_factor: u32,
    #[serde(borrow)]
    pub triple_layer_background_tint_color: GFXVector4<'a>,
    pub triple_layer_background_speed_x: u32,
    pub triple_layer_background_speed_y: u32,
    pub trail_effect_id: u32,
}

impl AutoDanceFxDesc<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("AutoDanceFxDesc");
}

impl Default for AutoDanceFxDesc<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            opacity: 1.0,
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
            parts_scale: vec![0, 0, 0, 0, 0],
            halftone_factor: 0,
            halftone_cutout_levels: 256.0,
            uv_blackout_factor: 0,
            uv_blackout_desaturation: 0.2,
            uv_blackout_contrast: 4.0,
            uv_blackout_brightness: 0,
            uv_blackout_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.54902,
                y: 0.54902,
                z: 1.0,
                w: 1.0,
            },
            toon_factor: 0.0,
            toon_cutout_levels: 256.0,
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
            slime_normal_tiling: 7.0,
            slime_light_angle: 0.0,
            slime_refraction: 0.0913,
            slime_refraction_index: 1.0837,
            slime_specular: 1.0,
            slime_specular_power: 10.0,
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
            swap_head_with_player: vec![0, 1, 2, 3, 4, 5],
            animate_player_head: vec![0, 0, 0, 0, 0, 0],
            animated_head_total_time: 20.0,
            animated_head_rest_time: 16.0,
            animated_head_frame_time: 0.6,
            animated_head_max_distance: 1.25,
            animated_head_max_angle: 1.2,
            screen_blend_inverse_alpha_factor: 0,
            screen_blend_inverse_alpha_scale_x: 0.0,
            screen_blend_inverse_alpha_scale_y: 0.0,
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
            floor_plane_factor: 0.0,
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
            textured_outline_tiling: 1.0,
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

#[derive(IntoOwned, Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Empty<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    class: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct FxEvent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub start_time: u32,
    pub duration: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct GFXVector4<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl GFXVector4<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("GFX_Vector4");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__class")]
pub enum ObjectiveDesc<'a> {
    #[serde(borrow, rename = "JD_ObjectiveDesc")]
    Base(ObjectiveDescBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_MinStarsReachedSongCount")]
    MinStarsReachedSongCount(ObjectiveDescMinStarsReachedSongCount<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_SweatSongCount")]
    SweatSongCount(ObjectiveDescBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_WDFSongCount")]
    WDFSongCount(ObjectiveDescBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_GatherStarsWDF")]
    GatherStarsWDF(ObjectiveDescBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_PlaySpecificMap")]
    PlaySpecificMap(ObjectiveDescPlaySpecificMap<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_RecommendedSongCount")]
    RecommendSongCount(ObjectiveDescBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_ClassicTournamentRank")]
    ClassicTournamentRank(ObjectiveDescBase<'a>),
}

impl ObjectiveDesc<'_> {
    #[must_use]
    pub const fn probability_weight(&self) -> u32 {
        match self {
            Self::Base(data)
            | Self::SweatSongCount(data)
            | Self::RecommendSongCount(data)
            | Self::WDFSongCount(data)
            | Self::GatherStarsWDF(data)
            | Self::ClassicTournamentRank(data) => data.probability_weight,
            Self::MinStarsReachedSongCount(data) => data.probability_weight,
            Self::PlaySpecificMap(data) => data.probability_weight,
        }
    }

    #[must_use]
    pub const fn description(&self) -> LocaleId {
        match self {
            Self::Base(data)
            | Self::SweatSongCount(data)
            | Self::RecommendSongCount(data)
            | Self::WDFSongCount(data)
            | Self::ClassicTournamentRank(data)
            | Self::GatherStarsWDF(data) => data.description,
            Self::MinStarsReachedSongCount(data) => data.description,
            Self::PlaySpecificMap(data) => data.description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescBase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    pub objective_type: u8,
    pub minimum_value: u32,
    pub probability_weight: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescMinStarsReachedSongCount<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    pub objective_type: u8,
    pub minimum_value: u32,
    pub probability_weight: u32,
    pub min_star_to_reach: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescPlaySpecificMap<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    pub objective_type: u8,
    pub minimum_value: u32,
    pub probability_weight: u32,
    #[serde(borrow)]
    pub map_name: HipStr<'a>,
}

/// Shared between .isg and .tpl
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PlaybackEvent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub clip_number: u32,
    pub start_clip: f32,
    pub start_time: f32,
    pub duration: f32,
    pub speed: f32,
}

impl PlaybackEvent<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("PlaybackEvent");
}

/// Shared between .isg and .tpl
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PropEvent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub start_time: u32,
    pub duration: f32,
    pub associated_props: Vec<u32>,
}

/// Shared between .isg and .tpl
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PropPlayerConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub index: u32,
    pub active_props: Vec<u32>,
}
