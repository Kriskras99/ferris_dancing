//! Contains the types that describe the usefull information in this filetype

use std::borrow::Cow;

use hipstr::HipStr;
use superstruct::superstruct;
use ubiart_toolkit_shared_types::Color;

use crate::utils::{errors::ParserError, SplitPath};

#[derive(Debug, Clone)]
pub struct Actor<'a> {
    pub lua: SplitPath<'a>,
    pub unk1: f32,
    pub unk2: f32,
    pub unk2_5: f32,
    pub unk3_5: u32,
    pub components: Vec<Component<'a>>,
}

impl PartialEq for Actor<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.unk1 == other.unk1
            && self.unk2 == other.unk2
            && self.lua == other.lua
            && self.unk2_5 == other.unk2_5
            && self.components == other.components
    }
}

impl Eq for Actor<'_> {}

#[derive(Debug, Clone, PartialEq)]
pub enum Component<'a> {
    AutodanceComponent,
    BeatPulseComponent(BeatPulseComponent<'a>),
    BlockFlowComponent,
    BoxInterpolatorComponent(BoxInterpolatorComponent),
    CameraFeedComponent(CameraFeedComponent),
    CameraGraphicComponent(Box<CameraGraphicComponent<'a>>),
    Carousel(Carousel<'a>),
    ClearColorComponent(ClearColorComponent),
    ConvertedTmlTapeComponent(ConvertedTmlTapeComponent<'a>),
    CreditsComponent(CreditsComponent<'a>),
    FixedCameraComponent(FixedCameraComponent),
    FXControllerComponent(FXControllerComponent),
    GoldMoveComponent,
    MasterTape,
    MaterialGraphicComponent(MaterialGraphicComponent<'a>),
    PictoComponent,
    PictoTimeline(PictoTimeline<'a>),
    PleoComponent(PleoComponent<'a>),
    PleoTextureGraphicComponent(MaterialGraphicComponent<'a>),
    PropertyPatcher,
    RegistrationComponent(RegistrationComponent<'a>),
    SingleInstanceMesh3DComponent(Box<SingleInstanceMesh3DComponent<'a>>),
    SongDatabaseComponent,
    SongDescComponent,
    SoundComponent,
    TapeCaseComponent,
    TextureGraphicComponent(TextureGraphicComponent<'a>),
    TexturePatcherComponent(TexturePatcherComponent<'a>),
    UICarousel(UICarousel<'a>),
    UITextBox(UITextBox<'a>),
    UIWidgetGroupHUD(UIWidgetGroupHUD<'a>),
    UIWidgetGroupHUDAutodanceRecorder(UIWidgetGroupHUDAutodanceRecorder<'a>),
    UIWidgetGroupHUDLyrics(UIWidgetGroupHUDLyrics<'a>),
    UIWidgetGroupHUDPauseIcon(UIWidgetGroupHUDPauseIcon<'a>),
    Unknown77F7D66C(Unknown77F7D66C<'a>),
    UnknownA6E4EFBA(UnknownA6E4EFBA),
    Unknown2CB3C8E8(Unknown2CB3C8E8),
    UnknownA97634C7(UnknownA97634C7),
    Unknown8C76D717,
    ViewportUIComponent(ViewportUIComponent),
    AvatarDescComponent,
    SkinDescComponent,
    FxBankComponent,
    BezierTreeComponent,
    AFXPostProcessComponent,
}

impl Component<'_> {
    #[must_use]
    pub const fn to_id(&self) -> u32 {
        match self {
            // JD_AutoDanceComponent
            Component::AutodanceComponent => 0x67B8_BB77,
            // JD_BeatPulseComponent
            Component::BeatPulseComponent(_) => 0x7184_37A8,
            // JD_BlockFlowComponent
            Component::BlockFlowComponent => 0x8DA9_E375,
            // BoxInterpolatorComponent
            Component::BoxInterpolatorComponent(_) => 0xF513_60DA,
            // JD_CameraFeedComponent
            Component::CameraFeedComponent(_) => 0x499C_BAA4,
            // CameraGraphicComponent
            Component::CameraGraphicComponent(_) => 0xC760_4FA1,
            // ClearColorComponent
            Component::ClearColorComponent(_) => 0xAEBB_218B,
            // ConvertedTmlTape_Component
            Component::ConvertedTmlTapeComponent(_) => 0xCD07_BB76,
            // JD_CreditsComponent
            Component::CreditsComponent(_) => 0x342E_A4FC,
            // JD_FixedCameraComponent
            Component::FixedCameraComponent(_) => 0x3D5D_EBA2,
            // FXControllerComponent
            Component::FXControllerComponent(_) => 0x8D4F_FFB6,
            // JD_GoldMoveComponent
            Component::GoldMoveComponent => 0x5632_1EA5,
            // MasterTape
            Component::MasterTape => 0x677B_269B,
            // MaterialGraphicComponent
            Component::MaterialGraphicComponent(_) => 0x72B6_1FC5,
            // JD_Carousel
            Component::Carousel(_) => 0x27E4_80C0,
            // JD_PictoComponent
            Component::PictoComponent => 0xC316_BF34,
            // JD_PictoTimeline
            Component::PictoTimeline(_) => 0xFA24_128D,
            // PleoComponent
            Component::PleoComponent(_) => 0x1263_DAD9,
            // PleoTextureGraphicComponent
            Component::PleoTextureGraphicComponent(_) => 0x0579_E81B,
            // PropertyPatcher
            Component::PropertyPatcher => 0xF719_B524,
            // JD_RegistrationComponent
            Component::RegistrationComponent(_) => 0xE0A2_4B6D,
            // SingleInstanceMesh3DComponent
            Component::SingleInstanceMesh3DComponent(_) => 0x53E3_2AF7,
            // JD_SongDatabaseComponent
            Component::SongDatabaseComponent => 0x4055_79FB,
            // JD_SongDescComponent
            Component::SongDescComponent => 0xE07F_CC3F,
            // SoundComponent
            Component::SoundComponent => 0x7DD8_643C,
            // TapeCase_Component
            Component::TapeCaseComponent => 0x231F_27DE,
            // TextureGraphicComponent
            Component::TextureGraphicComponent(_) => 0x7B48_A9AE,
            // TexturePatcherComponent
            Component::TexturePatcherComponent(_) => 0x6F32_8BC1,
            // UICarousel
            Component::UICarousel(_) => 0x8782_FE60,
            // UITextBox
            Component::UITextBox(_) => 0xD10C_BEED,
            // JD_UIWidgetGroupHUD_Lyrics
            Component::UIWidgetGroupHUD(_) => 0x1528_D94A,
            // JD_UIWidgetGroupHUD_AutodanceRecorder
            Component::UIWidgetGroupHUDAutodanceRecorder(_) => 0x9F87_350C,
            // JD_UIWidgetGroupHUD_Lyrics
            Component::UIWidgetGroupHUDLyrics(_) => 0xF22C_9426,
            // JD_UIWidgetGroupHUD_PauseIcon
            Component::UIWidgetGroupHUDPauseIcon(_) => 0x4866_6BB2,
            Component::Unknown77F7D66C(_) => 0x77F7_D66C,
            Component::UnknownA6E4EFBA(_) => 0xA6E4_EFBA,
            Component::Unknown2CB3C8E8(_) => 0x2CB3_C8E8,
            Component::UnknownA97634C7(_) => 0xA976_34C7,
            Component::Unknown8C76D717 => 0x8C76_D717,
            // ViewportUIComponent
            Component::ViewportUIComponent(_) => 0x6990_834C,
            // JD_AvatarDescComponent
            Component::AvatarDescComponent => 0x1759_E29D,
            // JD_SkinDescComponent
            Component::SkinDescComponent => 0x84EA_AE82,
            // FxBankComponent
            Component::FxBankComponent => 0x966B_519D,
            // BezierTreeComponent
            Component::BezierTreeComponent => 0x3236_CF4C,
            // AFXPostProcessComponent
            Component::AFXPostProcessComponent => 0x2B34_9E69,
        }
    }

    pub fn material_graphic_component(&self) -> Result<&MaterialGraphicComponent, ParserError> {
        if let Self::MaterialGraphicComponent(mgc) = self {
            Ok(mgc)
        } else {
            Err(ParserError::custom(format!(
                "MaterialGraphicComponent not found in component data: {self:?}"
            )))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AaBb {
    pub min: (f32, f32),
    pub max: (f32, f32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeatPulseComponent<'a> {
    pub text: HipStr<'a>,
    pub loc_id: u32,
    pub model_name: &'static str,
    pub flag: HipStr<'a>,
    pub elements: Vec<UIWidgetElementDesc<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoxInterpolatorComponent {
    pub inner_box: AaBb,
    pub outer_box: AaBb,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CameraFeedComponent;

#[derive(Debug, Clone, PartialEq)]
pub struct CameraGraphicComponent<'a> {
    pub primitive_parameters: GFXPrimitiveParam,
    pub color_computer_tag_id: u32,
    pub render_in_target: u32,
    pub disable_light: u32,
    pub disable_shadow: u32,
    pub atlas_index: u32,
    pub custom_anchor: (f32, f32),
    pub sinus_amplitude: (f32, f32, f32),
    pub sinus_speed: f32,
    pub angle_x: f32,
    pub angle_y: f32,
    pub anchor: u32,
    pub old_anchor: u32,
    pub material: GFXMaterialSerializable<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Carousel<'a> {
    pub main_anchor: u32,
    pub validate_action: &'static str,
    pub carousel_data_id: HipStr<'a>,
    pub manage_carousel_history: u32,
    pub switch_speed: f32,
    pub shortcuts_config_default: HipStr<'a>,
    pub shortcuts_config_switch: HipStr<'a>,
    pub shortcuts_config_ps4: HipStr<'a>,
    pub shortcuts_config_xb1: HipStr<'a>,
    pub shortcuts_config_pc: HipStr<'a>,
    pub shortcuts_config_ggp: HipStr<'a>,
    pub shortcuts_config_prospero: Option<HipStr<'a>>,
    pub shortcuts_config_scarlett: Option<HipStr<'a>>,
    pub shortcuts_from_center_instead_from_left: u32,
    pub initial_behaviour: &'static str,
    pub sound_context: HipStr<'a>,
    pub behaviours: Vec<CarouselBehaviour<'a>>,
    pub anim_items_desc: CarouselAnimItemsDesc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarouselAnimItemsDesc {
    pub enable: u32,
    pub show_items_at_init: u32,
    pub enable_carousel_on_anim_ends: u32,
    pub check_items_visibility_on_anim_ends: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CarouselBehaviour<'a> {
    Navigation(CarouselBehaviourNavigation<'a>),
    GoToElement(CarouselBehaviourGoToElement<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CarouselBehaviourNavigation<'a> {
    pub key: &'static str,
    pub sound_context: HipStr<'a>,
    pub sound_notif_go_next: HipStr<'a>,
    pub sound_notif_go_prev: HipStr<'a>,
    pub stop_conditions: Vec<StopCondition>,
    pub decel_tape_label: &'static str,
    pub scroll_mode: u32,
    pub time_between_steps: f32,
    pub next_actions: Vec<&'static str>,
    pub prev_actions: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CarouselBehaviourGoToElement<'a> {
    pub key: &'static str,
    pub sound_context: HipStr<'a>,
    pub sound_notif_go_next: HipStr<'a>,
    pub sound_notif_go_prev: HipStr<'a>,
    pub stop_conditions: Vec<StopCondition>,
    pub decel_tape_label: &'static str,
    pub scroll_mode: u32,
    pub time_between_steps: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClearColorComponent {
    pub clear_color: Color,
    pub clear_front_light_color: Color,
    pub clear_back_light_color: Color,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvertedTmlTapeComponent<'a> {
    pub map_name: HipStr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreditsComponent<'a> {
    pub lines_number: u32,
    pub name_font_size: f32,
    pub title_font_size: f32,
    pub big_title_font_size: f32,
    pub very_big_title_font_size: f32,
    pub anim_duration: f32,
    pub lines_pos_offset: f32,
    pub min_anim_duration: Option<f32>,
    pub speed_steps: Option<f32>,
    pub bottom_spawn_y: Option<f32>,
    pub top_spawn_y: Option<f32>,
    pub credits_lines: Vec<HipStr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixedCameraComponent {
    pub remote: u32,
    pub offset: (f32, f32, f32),
    pub start_as_main_cam: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FXControllerComponent {
    pub allow_bus_mix_events: u32,
    pub allow_music_events: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GFXMaterialSerializable<'a> {
    pub atl_channel: u32,
    pub atl_path: SplitPath<'a>,
    pub shader_path: SplitPath<'a>,
    pub stencil_test: u32,
    pub alpha_test: u32,
    pub alpha_ref: u32,
    pub texture_set: GFXMaterialTexturePathSet<'a>,
    pub material_params: GFXMaterialSerializableParam,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GFXMaterialSerializableParam {
    pub reflector_factor: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GFXMaterialTexturePathSet<'a> {
    pub diffuse: SplitPath<'a>,
    pub back_light: SplitPath<'a>,
    pub normal: SplitPath<'a>,
    pub separate_alpha: SplitPath<'a>,
    pub diffuse_2: SplitPath<'a>,
    pub back_light_2: SplitPath<'a>,
    pub anim_impostor: SplitPath<'a>,
    pub diffuse_3: SplitPath<'a>,
    pub diffuse_4: SplitPath<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GFXPrimitiveParam {
    pub color_factor: Color,
    pub gfx_occlude_info: u32,
}

/// Data for textures
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialGraphicComponent<'a> {
    pub files: [SplitPath<'a>; 11],
    pub unk4: f32,
    pub unk9: u32,
    /// Unknown value, 6 for tga with coach, 1 for tga without
    pub anchor: i32,
    pub unk11: f32,
    pub unk12: f32,
    pub old_anchor: i32,
}

impl Default for MaterialGraphicComponent<'static> {
    fn default() -> Self {
        Self {
            files: Default::default(),
            unk4: 1.0,
            unk9: u32::MAX,
            anchor: 1,
            unk11: 0.0,
            unk12: 0.0,
            old_anchor: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PictoTimeline<'a> {
    pub text: HipStr<'a>,
    pub loc_id: u32,
    pub model_name: &'static str,
    pub flag: HipStr<'a>,
    pub relative_start_position_solo: (f32, f32, f32),
    pub relative_start_position_duo: (f32, f32, f32),
    pub relative_start_position_trio: (f32, f32, f32),
    pub relative_start_position_quatro: (f32, f32, f32),
    pub relative_start_position_sextet: (f32, f32, f32),
    pub shifting_position_solo: (f32, f32, f32),
    pub shifting_position_duo: (f32, f32, f32),
    pub shifting_position_trio: (f32, f32, f32),
    pub shifting_position_quatro: (f32, f32, f32),
    pub shifting_position_sextet: (f32, f32, f32),
    pub picto_track_offset: u32,
    pub picto_scale: (f32, f32),
}

/// The data for the main video player
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PleoComponent<'a> {
    /// The filename of the video to play
    pub video: SplitPath<'a>,
    /// Manifest filename of the video
    pub dash_mpd: SplitPath<'a>,
    pub channel_id: HipStr<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationComponent<'a> {
    pub tag: &'static str,
    pub user_data: HipStr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SingleInstanceMesh3DComponent<'a> {
    pub color_computer_tag_id: u32,
    pub render_in_target: u32,
    pub disable_light: u32,
    pub disable_shadow: u32,
    pub scale_z: f32,
    pub mesh_3d: SplitPath<'a>,
    pub skeleton_3d: SplitPath<'a>,
    pub animation_3d: SplitPath<'a>,
    pub animation_node: SplitPath<'a>,
    pub orientation: [Color; 4],
    pub primitive_parameters: GFXPrimitiveParam,
    pub material: GFXMaterialSerializable<'a>,
    pub animation_player_mode: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundComponent;

#[derive(Debug, Clone, PartialEq)]
pub struct StopCondition {
    pub waiting_time: f32,
    pub count_to_reach: u32,
    pub next_behaviour: &'static str,
    pub condition: u32,
    pub anim_state: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureGraphicComponent<'a> {
    pub primitive_parameters: GFXPrimitiveParam,
    pub color_computer_tag_id: u32,
    pub render_in_target: u32,
    pub disable_light: u32,
    pub disable_shadow: u32,
    pub sprite_index: u32,
    pub anchor: u32,
    pub material: GFXMaterialSerializable<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TexturePatcherComponent<'a> {
    pub unk1: SplitPath<'a>,
    pub unk2: SplitPath<'a>,
}

#[superstruct(
    variants(V16, V1718, V1922),
    variant_attributes(derive(Debug, PartialEq, Clone))
)]
#[derive(Debug, Clone, PartialEq)]
pub struct UICarousel<'a> {
    #[superstruct(only(V16))]
    pub acceleration: f32,
    #[superstruct(only(V16))]
    pub deceleration: f32,
    #[superstruct(only(V16))]
    pub min_speed: f32,
    #[superstruct(only(V16))]
    pub max_speed: f32,
    pub main_anchor: u32,
    #[superstruct(only(V16))]
    pub min_deceleration_start_ratio: f32,
    #[superstruct(only(V16))]
    pub max_deceleration_start_ratio: f32,
    pub validate_action: &'static str,
    pub carousel_data_id: HipStr<'a>,
    #[superstruct(only(V16))]
    pub time_between_step: f32,
    #[superstruct(only(V16))]
    pub sound_notif_go_next: HipStr<'a>,
    #[superstruct(only(V16))]
    pub sound_notif_go_prev: HipStr<'a>,
    #[superstruct(only(V16, V1718))]
    pub force_loop: u32,
    #[superstruct(only(V16))]
    pub focus_anims_on_disabled_items: u32,
    pub manage_carousel_history: u32,
    #[superstruct(only(V16, V1718))]
    pub min_nb_items_to_loop: u32,
    #[superstruct(only(V16))]
    pub auto_scroll: u32,
    #[superstruct(only(V16))]
    pub auto_scroll_pause_time: f32,
    #[superstruct(only(V16))]
    pub auto_scroll_max_speed_ratio: f32,
    #[superstruct(only(V1718, V1922))]
    pub initial_behaviour: &'static str,
    pub sound_context: HipStr<'a>,
    #[superstruct(only(V1718, V1922))]
    pub behaviours: Vec<CarouselBehaviour<'a>>,
    #[superstruct(only(V16))]
    pub mode: i32,
    #[superstruct(only(V16))]
    pub next_actions: Vec<&'static str>,
    #[superstruct(only(V16))]
    pub prev_actions: Vec<&'static str>,
    pub anim_items_desc: CarouselAnimItemsDesc,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UITextBox<'a> {
    pub style: u32,
    pub overriding_font_size: f32,
    pub offset: (f32, f32),
    pub scale: (f32, f32),
    pub alpha: f32,
    pub max_width: f32,
    pub max_height: f32,
    pub area: (f32, f32),
    pub raw_text: HipStr<'a>,
    pub use_lines_max_count: u32,
    pub lines_max_count: u32,
    pub loc_id: u32,
    pub auto_scroll_speed: f32,
    pub auto_scroll_speed_y: f32,
    pub auto_scroll_wait_time: f32,
    pub auto_scroll_wait_time_y: f32,
    pub auto_scroll_font_effect_name: HipStr<'a>,
    pub auto_scroll_reset_on_inactive: u32,
    pub scroll_once: u32,
    pub overriding_shadow_color: (f32, f32, f32, f32),
    pub overriding_shadow_offset: (f32, f32),
    pub overriding_line_spacing: f32,
    pub overriding_font_size_min: f32,
    pub ending_dots: u32,
    /// Not in 2019 and earlier
    pub colorize_icons: Option<u32>,
    pub overriding_anchor: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UIWidgetElementDesc<'a> {
    pub element_path: SplitPath<'a>,
    pub name: HipStr<'a>,
    pub flag: HipStr<'a>,
    pub parent_index: i32,
    pub bind_mode: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UIWidgetGroupHUD<'a> {
    pub text: HipStr<'a>,
    pub loc_id: u32,
    pub model_name: &'static str,
    pub flag: HipStr<'a>,
    pub elements: Vec<UIWidgetElementDesc<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UIWidgetGroupHUDAutodanceRecorder<'a> {
    pub text: HipStr<'a>,
    pub loc_id: u32,
    pub model_name: &'static str,
    pub flag: HipStr<'a>,
    pub icon_default_position: (f32, f32, f32),
    pub icon_relative_start_position_solo: (f32, f32, f32),
    pub icon_relative_start_position_duo: (f32, f32, f32),
    pub icon_relative_start_position_trio: (f32, f32, f32),
    pub icon_relative_start_position_quatro: (f32, f32, f32),
    pub icon_relative_start_position_sextet: (f32, f32, f32),
    pub icon_shifting_position_solo: (f32, f32, f32),
    pub icon_shifting_position_duo: (f32, f32, f32),
    pub icon_shifting_position_trio: (f32, f32, f32),
    pub icon_shifting_position_quatro: (f32, f32, f32),
    pub icon_shifting_position_sextet: (f32, f32, f32),
    pub elements: Vec<UIWidgetElementDesc<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UIWidgetGroupHUDLyrics<'a> {
    pub text: HipStr<'a>,
    pub loc_id: u32,
    pub model_name: &'static str,
    pub flag: HipStr<'a>,
    pub elements: Vec<UIWidgetElementDesc<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UIWidgetGroupHUDPauseIcon<'a> {
    pub text: HipStr<'a>,
    pub loc_id: u32,
    pub model_name: &'static str,
    pub flag: HipStr<'a>,
    pub elements: Vec<UIWidgetElementDesc<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unknown77F7D66C<'a> {
    pub map_name: HipStr<'a>,
    pub jd_version: u32,
    pub unk2: u32,
    pub unk3: Cow<'a, [u8]>,
    pub unk4: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownA6E4EFBA;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unknown2CB3C8E8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownA97634C7;

#[derive(Debug, Clone, PartialEq)]
pub struct ViewportUIComponent {
    pub active: u32,
    pub focale: f32,
    pub far_plane: f32,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub view_mask: u32,
}
