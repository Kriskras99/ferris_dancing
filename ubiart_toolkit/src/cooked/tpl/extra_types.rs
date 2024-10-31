use hipstr::HipStr;
use serde::{Deserialize, Serialize};
use ubiart_toolkit_shared_types::Color;

use crate::{
    cooked::tpl::types::{AaBb, GFXMaterialSerializable},
    json_types::Empty,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AgingBotBehaviourAllTrees<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub treelist: Vec<AgingBotBehaviourTree<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AgingBotBehaviourGroup<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub related_gamescreen: HipStr<'a>,
    #[serde(borrow, rename = "BehaviourTypeList")]
    pub behaviour_type_list: Vec<BehaviourType<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AgingBotBehaviourTree<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(borrow, rename = "BehaviourGroups")]
    pub behaviour_groups: Vec<AgingBotBehaviourGroup<'a>>,
    #[serde(borrow)]
    pub success_if_log_contains: HipStr<'a>,
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BehaviourTypeAutoEndMap<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "type")]
    pub typed: u32,
    pub connection_name: HipStr<'a>,
    pub control_name: HipStr<'a>,
    #[serde(rename = "NumberOfMapsBeforeAutoEnd")]
    pub number_of_maps_before_auto_end: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BehaviourTypeBase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "type")]
    pub typed: u32,
    pub connection_name: HipStr<'a>,
    pub control_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BehaviourTypeSpecificCarrouselItem<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "type")]
    pub typed: u32,
    pub connection_name: HipStr<'a>,
    pub control_name: HipStr<'a>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FixedCameraComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub start_as_main_cam: u32,
    pub ramp_up_coeff: u32,
    pub focale: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SingleInstanceMesh3DComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    pub mesh_3d: HipStr<'a>,
    #[serde(rename = "skeleton3D")]
    pub skeleton_3d: HipStr<'a>,
    #[serde(rename = "animation3D")]
    pub animation_3d: HipStr<'a>,
    pub default_color: Color,
    pub animation_tree: Empty<'a>,
    pub animation_list: Empty<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UINineSliceMaskComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    pub mask_texture_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UITextBox<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub styles: Vec<Style<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Style<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FontSet<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub default: HipStr<'a>,
    pub japanese: HipStr<'a>,
    pub korean: HipStr<'a>,
    pub trad_chinese: HipStr<'a>,
    pub simple_chinese: HipStr<'a>,
    pub russian: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BezierTreeComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sample_count: u32,
    #[serde(rename = "widthForAABB")]
    pub width_for_aabb: u32,
    pub link_main_branch: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branch_components: Vec<BezierBranchFxComponent<'a>>,
    pub tween_interpreter: Empty<'a>,
    pub length_cursor_input: HipStr<'a>,
    #[serde(rename = "editor_lockNodeScale")]
    pub editor_lock_node_scale: u32,
    #[serde(rename = "editor_lockLastNodeScale")]
    pub editor_lock_last_node_scale: u32,
    #[serde(rename = "editor_lockFirstNode")]
    pub editor_lock_first_node: u32,
    #[serde(rename = "editor_allowSubBranches")]
    pub editor_allow_sub_branches: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BezierBranchFxComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    fx_start: HipStr<'a>,
    fx_loop: HipStr<'a>,
    fx_stop: HipStr<'a>,
    fx_default: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FxControllerComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub feedback_tags: Vec<HipStr<'a>>,
    pub fx_control_list: Vec<FXControl<'a>>,
    pub trigger_fx: HipStr<'a>,
    pub default_fx: HipStr<'a>,
    #[serde(rename = "onDieStopNotLoopedFX_HACK_DO_NOT_USE")]
    pub on_die_stop_not_loop_fx_hack_do_not_use: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FXControl<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub name: HipStr<'a>,
    pub fx_stop_on_end_anim: u32,
    pub fx_play_once: u32,
    pub fx_emit_from_base: u32,
    pub fx_use_actor_speed: u32,
    pub fx_use_actor_orientation: u32,
    pub fx_use_actor_alpha: u32,
    pub fx_bone_name: HipStr<'a>,
    pub fx_use_bone_orientation: u32,
    pub particles: Vec<HipStr<'a>>,
    pub sound_channel: u32,
    pub music: HipStr<'a>,
    pub bus_mix: HipStr<'a>,
    pub owner: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FxBankComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FxDescriptor<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub name: HipStr<'a>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProceduralInputData<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub input: HipStr<'a>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ItfParticleGenerator<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "computeAABB")]
    pub compute_aabb: u32,
    pub use_anim: u32,
    #[serde(rename = "loop")]
    pub loop_it: u32,
    pub amv_path: HipStr<'a>,
    #[serde(rename = "useUVRandom")]
    pub use_uv_random: u32,
    pub animstart: i32,
    pub animend: i32,
    pub animname: HipStr<'a>,
    #[serde(rename = "AnimUVfreq")]
    pub anim_uv_freq: u32,
    pub params: Box<ParticleGeneratorParameters<'a>>,
    pub z_sort_mode: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParticleGeneratorParameters<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    pub genangmin: f64,
    pub genangmax: f64,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EmitLifeTimeCurve<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub base_time: u32,
    pub output_min: (u32, u32, u32),
    pub output_max: (u32, u32, u32),
    pub curve: Spline<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParLifeTimeCurve<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub base_time: u32,
    pub output_min: (u32, u32, u32),
    pub output_max: (u32, u32, u32),
    pub curve: Spline<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Spline<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub time_loop_mode: u32,
    pub time_loop: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub points: Vec<SplinePoint<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SplinePoint<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub point: (f64, f64, f64),
    pub time: f32,
    pub normal_in: (f64, f64, f64),
    pub normal_in_time: (f32, f32, f32),
    pub normal_out: (f64, f64, f64),
    pub normal_out_time: (f32, f32, f32),
    pub interpolation: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BoxInterpolatorComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub input: HipStr<'a>,
    pub use_main_character: u32,
    pub can_use_touch_screen_players: u32,
    pub view: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AFXPostProcessComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub input: InputDesc<'a>,
    pub input_factor: InputDesc<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InputDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub name: HipStr<'a>,
    pub var_type: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TextureGraphicComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PleoTextureGraphicComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    pub shadow_attenuation: f32,
    pub shadow_dist: f32,
    pub shadow_offset_pos: (u32, u32, u32),
    pub angle_limit: u32,
    pub material: Box<GFXMaterialSerializable<'a>>,
    pub default_color: Color,
    pub z_offset: u32,
    #[serde(rename = "channelID")]
    pub channel_id: HipStr<'a>,
    pub auto_activate: u32,
    pub use_conductor: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModeType<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode_type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SkinDescription<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub jd_version: u32,
    #[serde(borrow)]
    pub relative_song_name: HipStr<'a>,
    #[serde(borrow, rename = "RelativeQuestID")]
    pub relative_quest_id: HipStr<'a>,
    #[serde(borrow, rename = "RelativeWDFBossName")]
    pub relative_wdf_boss_name: HipStr<'a>,
    #[serde(borrow, rename = "RelativeWDFTournamentName")]
    pub relative_wdf_tournament_name: HipStr<'a>,
    #[serde(borrow, rename = "RelativeJDRank")]
    pub relative_jd_rank: HipStr<'a>,
    #[serde(borrow)]
    pub relative_game_mode_name: HipStr<'a>,
    #[serde(borrow)]
    pub sound_family: HipStr<'a>,
    pub status: u32,
    pub unlock_type: u32,
    pub mojo_price: u32,
    pub wdf_level: u32,
    pub count_in_progression: u32,
    #[serde(borrow)]
    pub actor_path: HipStr<'a>,
    #[serde(borrow)]
    pub phone_image: HipStr<'a>,
    #[serde(rename = "skinId")]
    pub skin_id: u32,
}
