use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[cfg(feature = "full_json_types")]
use super::Empty;
use crate::utils::Color;

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SingleInstanceMesh3DComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub patch_level: u32,
    pub patch_h_level: u32,
    pub patch_v_level: u32,
    #[serde(rename = "visualAABB")]
    pub visual_aabb: AaBb<'a>,
    pub renderintarget: u32,
    pub pos_offset: (u32, u32),
    pub angle_offset: f32,
    pub blendmode: u32,
    pub materialtype: u32,
    pub self_illum_color: Color,
    pub disable_light: u32,
    pub force_disable_light: u32,
    pub use_shadow: u32,
    pub use_root_bone: u32,
    pub shadow_size: (f32, f32),
    pub shadow_material: Box<GFXMaterialSerializable<'a>>,
    pub shadow_attenuation: u32,
    pub shadow_dist: u32,
    pub shadow_offset_pos: (u32, u32, u32),
    pub angle_limit: u32,
    #[serde(rename = "forcedAABB")]
    pub forced_aabb: AaBb<'a>,
    #[serde(rename = "mesh3D")]
    pub mesh_3d: Cow<'a, str>,
    #[serde(rename = "skeleton3D")]
    pub skeleton_3d: Cow<'a, str>,
    #[serde(rename = "animation3D")]
    pub animation_3d: Cow<'a, str>,
    pub default_color: Color,
    pub animation_tree: Empty<'a>,
    pub animation_list: Empty<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UINineSliceMaskComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub patch_level: u32,
    pub patch_h_level: u32,
    pub patch_v_level: u32,
    #[serde(rename = "visualAABB")]
    pub visual_aabb: AaBb<'a>,
    pub renderintarget: u32,
    pub pos_offset: (u32, u32),
    pub angle_offset: f32,
    pub blendmode: u32,
    pub materialtype: u32,
    pub self_illum_color: Color,
    pub disable_light: u32,
    pub force_disable_light: u32,
    pub use_shadow: u32,
    pub use_root_bone: u32,
    pub shadow_size: (f32, f32),
    pub shadow_material: GFXMaterialSerializable<'a>,
    pub shadow_attenuation: u32,
    pub shadow_dist: u32,
    pub shadow_offset_pos: (u32, u32, u32),
    pub angle_limit: u32,
    pub material: GFXMaterialSerializable<'a>,
    pub default_color: Color,
    pub z_offset: u32,
    pub mask_ratio: u32,
    pub mask_texture_path: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UITextBox<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub styles: Vec<Style<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Style<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub font_set: FontSet<'a>,
    pub font_size: u32,
    pub font_size_min: i32,
    pub color: Color,
    pub progressive_color: Color,
    pub blend_mode: u32,
    pub shadow_offset: (u32, u32),
    pub shadow_color: Color,
    pub line_spacing: u32,
    pub paragraph_spacing: u32,
    pub anchor: u32,
    pub h_alignment: u32,
    pub v_alignment: u32,
    pub use_gradient: u32,
    pub gradient_size: u32,
    pub gradient_color: Color,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FontSet<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub default: Cow<'a, str>,
    pub japanese: Cow<'a, str>,
    pub korean: Cow<'a, str>,
    pub trad_chinese: Cow<'a, str>,
    pub simple_chinese: Cow<'a, str>,
    pub russian: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BezierTreeComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub sample_count: u32,
    #[serde(rename = "widthForAABB")]
    pub width_for_aabb: u32,
    pub link_main_branch: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branch_components: Vec<BezierBranchFxComponent<'a>>,
    pub tween_interpreter: Empty<'a>,
    pub length_cursor_input: Cow<'a, str>,
    #[serde(rename = "editor_lockNodeScale")]
    pub editor_lock_node_scale: u32,
    #[serde(rename = "editor_lockLastNodeScale")]
    pub editor_lock_last_node_scale: u32,
    #[serde(rename = "editor_lockFirstNode")]
    pub editor_lock_first_node: u32,
    #[serde(rename = "editor_allowSubBranches")]
    pub editor_allow_sub_branches: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BezierBranchFxComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    fx_start: Cow<'a, str>,
    fx_loop: Cow<'a, str>,
    fx_stop: Cow<'a, str>,
    fx_default: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FxControllerComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub feedback_tags: Vec<Cow<'a, str>>,
    pub fx_control_list: Vec<FXControl<'a>>,
    pub trigger_fx: Cow<'a, str>,
    pub default_fx: Cow<'a, str>,
    #[serde(rename = "onDieStopNotLoopedFX_HACK_DO_NOT_USE")]
    pub on_die_stop_not_loop_fx_hack_do_not_use: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FXControl<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub fx_stop_on_end_anim: u32,
    pub fx_play_once: u32,
    pub fx_emit_from_base: u32,
    pub fx_use_actor_speed: u32,
    pub fx_use_actor_orientation: u32,
    pub fx_use_actor_alpha: u32,
    pub fx_bone_name: Cow<'a, str>,
    pub fx_use_bone_orientation: u32,
    pub particles: Vec<Cow<'a, str>>,
    pub sound_channel: u32,
    pub music: Cow<'a, str>,
    pub bus_mix: Cow<'a, str>,
    pub owner: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FxBankComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub patch_level: u32,
    pub patch_h_level: u32,
    pub patch_v_level: u32,
    #[serde(rename = "visualAABB")]
    pub visual_aabb: AaBb<'a>,
    pub renderintarget: u32,
    pub pos_offset: (u32, u32),
    pub angle_offset: f32,
    pub blendmode: u32,
    pub materialtype: u32,
    pub self_illum_color: Color,
    pub disable_light: u32,
    pub force_disable_light: u32,
    pub use_shadow: u32,
    pub use_root_bone: u32,
    pub shadow_size: (f32, f32),
    pub shadow_material: GFXMaterialSerializable<'a>,
    pub shadow_attenuation: u32,
    pub shadow_dist: u32,
    pub shadow_offset_pos: (u32, u32, u32),
    pub angle_limit: u32,
    #[serde(rename = "Fx")]
    pub fx: Vec<FxDescriptor<'a>>,
    pub visibility_test: u32,
    #[serde(rename = "MaxActiveInstance")]
    pub max_active_instance: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FxDescriptor<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub gen: ItfParticleGenerator<'a>,
    pub material: GFXMaterialSerializable<'a>,
    pub angle_offset: f32,
    pub min_delay: f32,
    pub max_delay: f32,
    pub frequency_input: ProceduralInputData<'a>,
    pub emit_count_input: ProceduralInputData<'a>,
    pub max_particles_input: ProceduralInputData<'a>,
    pub velocity_input: ProceduralInputData<'a>,
    pub velocity_delta_input: ProceduralInputData<'a>,
    pub angular_speed_input: ProceduralInputData<'a>,
    pub angular_speed_delta_input: ProceduralInputData<'a>,
    pub default_alpha_input: ProceduralInputData<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProceduralInputData<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub input: Cow<'a, str>,
    pub min: u32,
    pub max: u32,
    pub min_value: u32,
    pub max_value: u32,
    #[serde(rename = "mod")]
    pub modulus: u32,
    pub abs: u32,
    pub add: u32,
    pub sin: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ItfParticleGenerator<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    #[serde(rename = "computeAABB")]
    pub compute_aabb: u32,
    pub use_anim: u32,
    #[serde(rename = "loop")]
    pub loop_it: u32,
    pub amv_path: Cow<'a, str>,
    #[serde(rename = "useUVRandom")]
    pub use_uv_random: u32,
    pub animstart: i32,
    pub animend: i32,
    pub animname: Cow<'a, str>,
    #[serde(rename = "AnimUVfreq")]
    pub anim_uv_freq: u32,
    pub params: Box<ParticleGeneratorParameters<'a>>,
    pub z_sort_mode: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParticleGeneratorParameters<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub max_particles: u32,
    pub default_color: Color,
    pub emit_particles_count: u32,
    pub force_no_dynamic_fog: u32,
    pub render_in_reflection: u32,
    pub die_fade_time: i32,
    pub emitter_max_life_time: f32,
    pub pos: (i32, i32, i32),
    pub pivot: (f32, f32),
    pub vel_norm: u32,
    pub vel_angle: u32,
    pub vel_angle_delta: u32,
    pub grav: (u32, u32, u32),
    pub acc: (u32, u32, u32),
    pub depth: u32,
    pub use_z_as_depth: u32,
    pub velocity_var: u32,
    pub friction: u32,
    pub freq: f32,
    pub freq_delta: u32,
    pub force_emit_at_start: u32,
    pub emit_batch_count: u32,
    #[serde(rename = "emitBatchCount_AAO")]
    pub emit_batch_count_aao: u32,
    #[serde(rename = "emitBatchCount_AAO_max")]
    pub emit_batch_count_aao_max: u32,
    pub init_angle: u32,
    pub angle_delta: u32,
    pub angular_speed: u32,
    pub angular_speed_delta: u32,
    pub time_target: u32,
    pub nb_phase: u32,
    pub render_prio: u32,
    pub init_life_time: u32,
    pub circle_radius: u32,
    pub inner_circle_radius: f32,
    pub scale_shape: (u32, u32, u32),
    pub rotate_shape: (u32, u32, u32),
    pub randomize_direction: u32,
    pub follow_bezier: i32,
    pub get_atlas_size: u32,
    pub gen_box: AaBb<'a>,
    #[serde(rename = "GenSize")]
    pub gen_size: u32,
    #[serde(rename = "GenSide")]
    pub gen_side: u32,
    #[serde(rename = "GenPosMin")]
    pub gen_pos_min: u32,
    #[serde(rename = "GenPosMax")]
    pub gen_pos_max: u32,
    #[serde(rename = "GenDensity")]
    pub gen_density: i32,
    pub bounding_box: AaBb<'a>,
    pub orient_dir: u32,
    #[serde(rename = "UVmode")]
    pub uvmode: u32,
    #[serde(rename = "UVmodeFlag")]
    pub uvmode_flag: u32,
    pub uniformscale: u32,
    pub impostorrendering: u32,
    pub showimpostorrender: u32,
    pub impostor_texture_size_x: u32,
    pub impostor_texture_size_y: u32,
    pub genangmin: f32,
    pub genangmax: f32,
    pub can_flip_angle_offset: u32,
    pub can_flip_init_angle: u32,
    pub can_flip_angular_speed: u32,
    pub can_flip_pivot: u32,
    pub can_flip_pos: u32,
    #[serde(rename = "canFlipUV")]
    pub can_flip_uv: u32,
    pub can_flip_angle_min: u32,
    pub can_flip_angle_max: u32,
    pub can_flip_accel: u32,
    pub can_flip_orient_dir: u32,
    pub number_split: u32,
    pub split_delta: u32,
    pub use_matrix: u32,
    pub use_phases_color_and_size: u32,
    pub use_actor_translation: u32,
    pub actor_translation_offset: (u32, u32),
    pub disable_light: u32,
    pub curve_position: ParLifeTimeCurve<'a>,
    pub curve_angle: ParLifeTimeCurve<'a>,
    pub curve_velocity_mult: ParLifeTimeCurve<'a>,
    pub curve_accel_x: ParLifeTimeCurve<'a>,
    pub curve_accel_y: ParLifeTimeCurve<'a>,
    pub curve_accel_z: ParLifeTimeCurve<'a>,
    pub curve_size: ParLifeTimeCurve<'a>,
    pub curve_size_y: ParLifeTimeCurve<'a>,
    pub curve_alpha: ParLifeTimeCurve<'a>,
    #[serde(rename = "curveRGB")]
    pub curve_rgb: ParLifeTimeCurve<'a>,
    #[serde(rename = "curveRGB1")]
    pub curve_rgb1: ParLifeTimeCurve<'a>,
    #[serde(rename = "curveRGB2")]
    pub curve_rgb2: ParLifeTimeCurve<'a>,
    #[serde(rename = "curveRGB3")]
    pub curve_rgb3: ParLifeTimeCurve<'a>,
    pub curve_anim: ParLifeTimeCurve<'a>,
    pub par_emit_velocity: EmitLifeTimeCurve<'a>,
    pub par_emit_velocity_angle: EmitLifeTimeCurve<'a>,
    pub par_emit_angle: EmitLifeTimeCurve<'a>,
    pub par_emit_angular_speed: EmitLifeTimeCurve<'a>,
    pub curve_freq: EmitLifeTimeCurve<'a>,
    pub curve_par_life_time: EmitLifeTimeCurve<'a>,
    pub curve_emit_alpha: EmitLifeTimeCurve<'a>,
    pub curve_emit_color_factor: EmitLifeTimeCurve<'a>,
    #[serde(rename = "curveEmitSizeXY")]
    pub curve_emit_size_xy: EmitLifeTimeCurve<'a>,
    pub curve_emit_acceleration: EmitLifeTimeCurve<'a>,
    pub curve_emit_gravity: EmitLifeTimeCurve<'a>,
    pub curve_emit_anim: EmitLifeTimeCurve<'a>,
    pub gen_gen_type: u32,
    pub gen_mode: u32,
    pub gen_emit_mode: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EmitLifeTimeCurve<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub base_time: u32,
    pub output_min: (u32, u32, u32),
    pub output_max: (u32, u32, u32),
    pub curve: Spline<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParLifeTimeCurve<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub base_time: u32,
    pub output_min: (u32, u32, u32),
    pub output_max: (u32, u32, u32),
    pub curve: Spline<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Spline<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub time_loop_mode: u32,
    pub time_loop: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub points: Vec<SplinePoint<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SplinePoint<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub point: (f32, f32, f32),
    pub time: f32,
    pub normal_in: (f32, f32, f32),
    pub normal_in_time: (f32, f32, f32),
    pub normal_out: (f32, f32, f32),
    pub normal_out_time: (f32, f32, f32),
    pub interpolation: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MusicTrackComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub track_data: MusicTrackData<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MusicTrackData<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub structure: MusicTrackStructure<'a>,
    pub path: Cow<'a, str>,
    pub url: Cow<'a, str>,
}

impl MusicTrackData<'_> {
    pub const CLASS: &'static str = "MusicTrackData";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MusicTrackStructure<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub markers: Vec<u32>,
    pub signatures: Vec<MusicSignature<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<MusicSection<'a>>,
    pub start_beat: i32,
    pub end_beat: u32,
    #[serde(default)]
    pub fade_start_beat: u32,
    #[serde(default)]
    pub use_fade_start_beat: bool,
    #[serde(default)]
    pub fade_end_beat: u32,
    #[serde(default)]
    pub use_fade_end_beat: bool,
    pub video_start_time: f32,
    pub preview_entry: f32,
    pub preview_loop_start: f32,
    pub preview_loop_end: f32,
    pub volume: f32,
    #[serde(default)]
    pub fade_in_duration: u32,
    #[serde(default)]
    pub fade_in_type: u32,
    #[serde(default)]
    pub fade_out_duration: u32,
    #[serde(default)]
    pub fade_out_type: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_points: Vec<u32>,
}

impl MusicTrackStructure<'_> {
    pub const CLASS: &'static str = "MusicTrackStructure";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MusicSignature<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub marker: f32,
    pub beats: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Cow<'a, str>>,
}

impl MusicSignature<'_> {
    pub const CLASS: &'static str = "MusicSignature";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MusicSection<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub marker: f32,
    pub section_type: u32,
    pub comment: Cow<'a, str>,
}

impl MusicSection<'_> {
    pub const CLASS: &'static str = "MusicSection";
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MasterTape<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tapes_rack: Vec<TapeGroup<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct TapeGroup<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub entries: Vec<TapeEntry<'a>>,
}

impl TapeGroup<'_> {
    pub const CLASS: &'static str = "TapeGroup";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct TapeEntry<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub label: Cow<'a, str>,
    pub path: Cow<'a, str>,
}

impl TapeEntry<'_> {
    pub const CLASS: &'static str = "TapeEntry";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PleoComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub video: Cow<'a, str>,
    #[serde(rename = "videoURL")]
    pub video_url: Cow<'a, str>,
    pub auto_play: u32,
    pub play_from_memory: u32,
    pub play_to_texture: u32,
    #[serde(rename = "loop")]
    pub loop_it: u32,
    pub clean_loop: u32,
    pub alpha: u32,
    pub sound: u32,
    #[serde(rename = "channelID")]
    pub channel_id: Cow<'a, str>,
    pub adaptive: u32,
    pub auto_stop_at_the_end: u32,
    // Not in WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_discard_after_stop: Option<u32>,
    #[serde(rename = "dashMPD")]
    pub dash_mpd: Cow<'a, str>,
    pub audio_bus: Cow<'a, str>,
    pub loop_frame: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BoxInterpolatorComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub input: Cow<'a, str>,
    pub use_main_character: u32,
    pub can_use_touch_screen_players: u32,
    pub view: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AFXPostProcessComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub input: InputDesc<'a>,
    pub input_factor: InputDesc<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InputDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub var_type: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MaterialGraphicComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub patch_level: u32,
    pub patch_h_level: u32,
    pub patch_v_level: u32,
    #[serde(rename = "visualAABB")]
    pub visual_aabb: AaBb<'a>,
    pub renderintarget: u32,
    pub pos_offset: (u32, u32),
    pub angle_offset: f32,
    pub blendmode: u32,
    pub materialtype: u32,
    pub self_illum_color: Color,
    pub disable_light: u32,
    pub force_disable_light: u32,
    pub use_shadow: u32,
    pub use_root_bone: u32,
    pub shadow_size: (f32, f32),
    pub shadow_material: Box<GFXMaterialSerializable<'a>>,
    pub shadow_attenuation: u32,
    pub shadow_dist: u32,
    pub shadow_offset_pos: (u32, u32, u32),
    pub angle_limit: u32,
    pub material: Box<GFXMaterialSerializable<'a>>,
    pub default_color: Color,
    pub z_offset: u32,
}

impl Default for MaterialGraphicComponent<'static> {
    fn default() -> Self {
        Self {
            class: Option::default(),
            patch_level: Default::default(),
            patch_h_level: 2,
            patch_v_level: 2,
            visual_aabb: AaBb::default(),
            renderintarget: Default::default(),
            pos_offset: Default::default(),
            angle_offset: Default::default(),
            blendmode: 2,
            materialtype: Default::default(),
            self_illum_color: Default::default(),
            disable_light: Default::default(),
            force_disable_light: Default::default(),
            use_shadow: Default::default(),
            use_root_bone: Default::default(),
            shadow_size: Default::default(),
            shadow_material: Box::<GFXMaterialSerializable<'_>>::default(),
            shadow_attenuation: 1,
            shadow_dist: Default::default(),
            shadow_offset_pos: Default::default(),
            angle_limit: Default::default(),
            material: Box::<GFXMaterialSerializable<'_>>::default(),
            default_color: (1.0, 1.0, 1.0, 1.0),
            z_offset: Default::default(),
        }
    }
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TextureGraphicComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub patch_level: u32,
    pub patch_h_level: u32,
    pub patch_v_level: u32,
    #[serde(rename = "visualAABB")]
    pub visual_aabb: AaBb<'a>,
    pub renderintarget: u32,
    pub pos_offset: (u32, u32),
    pub angle_offset: f32,
    pub blendmode: u32,
    pub materialtype: u32,
    pub self_illum_color: Color,
    pub disable_light: u32,
    pub force_disable_light: u32,
    pub use_shadow: u32,
    pub use_root_bone: u32,
    pub shadow_size: (f32, f32),
    pub shadow_material: Box<GFXMaterialSerializable<'a>>,
    pub shadow_attenuation: u32,
    pub shadow_dist: u32,
    pub shadow_offset_pos: (u32, u32, u32),
    pub angle_limit: u32,
    pub material: Box<GFXMaterialSerializable<'a>>,
    pub default_color: Color,
    pub angle_x: u32,
    pub angle_y: u32,
    pub angle_z: u32,
    pub speed_rot_x: u32,
    pub speed_rot_y: u32,
    pub speed_rot_z: u32,
    pub size: (f32, f32),
    pub z_offset: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PleoTextureGraphicComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub patch_level: u32,
    pub patch_h_level: u32,
    pub patch_v_level: u32,
    #[serde(rename = "visualAABB")]
    pub visual_aabb: AaBb<'a>,
    pub renderintarget: u32,
    pub pos_offset: (u32, u32),
    pub angle_offset: f32,
    pub blendmode: u32,
    pub materialtype: u32,
    pub self_illum_color: Color,
    pub disable_light: u32,
    pub force_disable_light: u32,
    pub use_shadow: u32,
    pub use_root_bone: u32,
    pub shadow_size: (f32, f32),
    pub shadow_material: Box<GFXMaterialSerializable<'a>>,
    pub shadow_attenuation: u32,
    pub shadow_dist: u32,
    pub shadow_offset_pos: (u32, u32, u32),
    pub angle_limit: u32,
    pub material: Box<GFXMaterialSerializable<'a>>,
    pub default_color: Color,
    pub z_offset: u32,
    #[serde(rename = "channelID")]
    pub channel_id: Cow<'a, str>,
    pub auto_activate: u32,
    pub use_conductor: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GFXMaterialSerializable<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub texture_set: GFXMaterialTexturePathSet<'a>,
    #[serde(rename = "ATL_Channel")]
    pub atl_channel: u32,
    #[serde(rename = "ATL_Path")]
    pub atl_path: Cow<'a, str>,
    pub shader_path: Cow<'a, str>,
    pub material_params: GFXMaterialSerializableParam<'a>,
    /// Not in nx2017-nx2019
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outlined_mask_params: Option<OutlinedMaskMaterialParams<'a>>,
    /// Only in nx2017-nx2019 and nx2020_japan
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stencil_test: Option<u32>,
    pub alpha_test: u32,
    pub alpha_ref: u32,
}

impl GFXMaterialSerializable<'_> {
    pub const CLASS: &'static str = "GFXMaterialSerializable";
}

impl Default for GFXMaterialSerializable<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            texture_set: GFXMaterialTexturePathSet::default(),
            atl_channel: 0,
            atl_path: Cow::default(),
            shader_path: Cow::default(),
            material_params: GFXMaterialSerializableParam::default(),
            outlined_mask_params: Some(OutlinedMaskMaterialParams::default()),
            stencil_test: None,
            alpha_test: u32::MAX,
            alpha_ref: u32::MAX,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GFXMaterialTexturePathSet<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub diffuse: Cow<'a, str>,
    pub back_light: Cow<'a, str>,
    pub normal: Cow<'a, str>,
    #[serde(rename = "separateAlpha")]
    pub separate_alpha: Cow<'a, str>,
    pub diffuse_2: Cow<'a, str>,
    pub back_light_2: Cow<'a, str>,
    pub anim_impostor: Cow<'a, str>,
    pub diffuse_3: Cow<'a, str>,
    pub diffuse_4: Cow<'a, str>,
}

impl GFXMaterialTexturePathSet<'_> {
    pub const CLASS: &'static str = "GFXMaterialTexturePathSet";
}

impl Default for GFXMaterialTexturePathSet<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            diffuse: Cow::default(),
            back_light: Cow::default(),
            normal: Cow::default(),
            separate_alpha: Cow::default(),
            diffuse_2: Cow::default(),
            back_light_2: Cow::default(),
            anim_impostor: Cow::default(),
            diffuse_3: Cow::default(),
            diffuse_4: Cow::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GFXMaterialSerializableParam<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "Reflector_factor")]
    pub reflector_factor: u32,
}

impl GFXMaterialSerializableParam<'_> {
    pub const CLASS: &'static str = "GFXMaterialSerializableParam";
}

impl Default for GFXMaterialSerializableParam<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            reflector_factor: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OutlinedMaskMaterialParams<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub mask_color: Color,
    pub outline_color: Color,
    pub thickness: u32,
}

impl OutlinedMaskMaterialParams<'_> {
    pub const CLASS: &'static str = "OutlinedMaskMaterialParams";
}

impl Default for OutlinedMaskMaterialParams<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            mask_color: Default::default(),
            outline_color: Default::default(),
            thickness: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct AaBb<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub min: (f32, f32),
    pub max: (f32, f32),
}

impl AaBb<'_> {
    pub const CLASS: &'static str = "AABB";
}

impl Default for AaBb<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            min: Default::default(),
            max: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoundComponent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub sound_list: Vec<SoundDescriptor<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoundDescriptor<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub volume: f32,
    pub category: Cow<'a, str>, // TODO: Make enum
    pub limit_category: Cow<'a, str>,
    pub limit_mode: u32,
    pub max_instances: u32,
    pub files: Vec<Cow<'a, str>>,
    pub serial_playing_mode: u32,
    pub serial_stopping_mode: u32,
    pub params: SoundParams<'a>,
    pub pause_insensitive_flags: u32,
    pub out_devices: u32,
    #[serde(rename = "soundPlayAfterdestroy")]
    pub sound_play_after_destroy: u32,
}

impl SoundDescriptor<'_> {
    pub const CLASS: &'static str = "SoundDescriptor_Template";
}

impl Default for SoundDescriptor<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            name: Cow::default(),
            volume: 0.0,
            category: Cow::Borrowed("amb"),
            limit_category: Cow::default(),
            limit_mode: 0,
            max_instances: u32::MAX,
            files: Vec::new(),
            serial_playing_mode: 0,
            serial_stopping_mode: 0,
            params: SoundParams::default(),
            pause_insensitive_flags: 0,
            out_devices: u32::MAX,
            sound_play_after_destroy: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoundParams<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    #[serde(rename = "loop")]
    pub loop_it: u32,
    pub play_mode: u32,
    pub play_mode_input: Cow<'a, str>,
    pub random_vol_min: f32,
    pub random_vol_max: f32,
    pub delay: u32,
    pub random_delay: u32,
    /// Not present in nx2017
    #[serde(default = "default_pitch")]
    pub pitch: f32,
    pub random_pitch_min: f32,
    pub random_pitch_max: f32,
    pub fade_in_time: f32,
    pub fade_out_time: f32,
    pub filter_frequency: u32,
    pub filter_type: u32,
    pub transition_sample_offset: u32,
}

const fn default_pitch() -> f32 {
    1.0
}

impl SoundParams<'_> {
    pub const CLASS: &'static str = "SoundParams";
}

impl Default for SoundParams<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            loop_it: 0,
            play_mode: 1,
            play_mode_input: Cow::default(),
            random_vol_min: 0.0,
            random_vol_max: 0.0,
            delay: 0,
            random_delay: 0,
            pitch: 1.0,
            random_pitch_min: 1.0,
            random_pitch_max: 1.0,
            fade_in_time: 0.0,
            fade_out_time: 0.0,
            filter_frequency: 0,
            filter_type: 2,
            transition_sample_offset: 0,
        }
    }
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModeType<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode_type: Option<i32>,
}
