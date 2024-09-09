//! Contains the types that describe the usefull information in this filetype

use std::borrow::Cow;

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
    BeatPulseComponent,
    BoxInterpolatorComponent(BoxInterpolatorComponent),
    CameraGraphicComponent,
    Carousel(Carousel<'a>),
    ClearColorComponent(ClearColorComponent),
    ConvertedTmlTapeComponent(ConvertedTmlTapeComponent<'a>),
    CreditsComponent(CreditsComponent<'a>),
    FixedCameraComponent(FixedCameraComponent),
    FXControllerComponent(FXControllerComponent),
    MasterTape,
    MaterialGraphicComponent(MaterialGraphicComponent<'a>),
    PictoComponent,
    PleoComponent(PleoComponent<'a>),
    PleoTextureGraphicComponent(MaterialGraphicComponent<'a>),
    PropertyPatcher,
    RegistrationComponent,
    SingleInstanceMesh3DComponent,
    SongDatabaseComponent,
    SongDescComponent,
    SoundComponent,
    TapeCaseComponent,
    TextureGraphicComponent,
    UICarousel,
    UITextBox(UITextBox<'a>),
    UIWdigetGroupHUDAutodanceRecorder,
    UIWidgetGroupHUDLyrics,
    ViewportUIComponent,
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
            Component::BeatPulseComponent => 0x7184_37A8,
            // BoxInterpolatorComponent
            Component::BoxInterpolatorComponent(_) => 0xF513_60DA,
            // CameraGraphicComponent
            Component::CameraGraphicComponent => 0xC760_4FA1,
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
            // MasterTape
            Component::MasterTape => 0x677B_269B,
            // MaterialGraphicComponent
            Component::MaterialGraphicComponent(_) => 0x72B6_1FC5,
            // JD_Carousel
            Component::Carousel(_) => 0x27E4_80C0,
            // JD_PictoComponent
            Component::PictoComponent => 0xC316_BF34,
            // PleoComponent
            Component::PleoComponent(_) => 0x1263_DAD9,
            // PleoTextureGraphicComponent
            Component::PleoTextureGraphicComponent(_) => 0x0579_E81B,
            // PropertyPatcher
            Component::PropertyPatcher => 0xF719_B524,
            // JD_RegistrationComponent
            Component::RegistrationComponent => 0xE0A2_4B6D,
            // SingleInstanceMesh3DComponent
            Component::SingleInstanceMesh3DComponent => 0x53E3_2AF7,
            // JD_SongDatabaseComponent
            Component::SongDatabaseComponent => 0x4055_79FB,
            // JD_SongDescComponent
            Component::SongDescComponent => 0xE07F_CC3F,
            // SoundComponent
            Component::SoundComponent => 0x7DD8_643C,
            // TapeCase_Component
            Component::TapeCaseComponent => 0x231F_27DE,
            // TextureGraphicComponent
            Component::TextureGraphicComponent => 0x7B48_A9AE,
            // UICarousel
            Component::UICarousel => 0x8782_FE60,
            // UITextBox
            Component::UITextBox(_) => 0xD10C_BEED,
            // JD_UIWidgetGroupHUD_AutodanceRecorder
            Component::UIWdigetGroupHUDAutodanceRecorder => 0x9F87_350C,
            // JD_UIWidgetGroupHUD_Lyrics
            Component::UIWidgetGroupHUDLyrics => 0xF22C_9426,
            // ViewportUIComponent
            Component::ViewportUIComponent => 0x6990_834C,
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

#[derive(Debug, Clone, PartialEq)]
pub struct BoxInterpolatorComponent {
    pub inner_box: AaBb,
    pub outer_box: AaBb,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Carousel<'a> {
    pub validate_action: Cow<'a, str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClearColorComponent {
    pub clear_color: Color,
    pub clear_front_light_color: Color,
    pub clear_back_light_color: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConvertedTmlTapeComponent<'a> {
    pub map_name: Cow<'a, str>,
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
    pub credits_lines: Vec<Cow<'a, str>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixedCameraComponent {
    pub remote: u32,
    pub offset: (f32, f32, f32),
    pub start_as_main_cam: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FXControllerComponent {
    pub allow_bus_mix_events: u32,
    pub allow_music_events: u32,
}

/// The data for the main video player
#[derive(Debug, Clone, PartialEq)]
pub struct PleoComponent<'a> {
    /// The filename of the video to play
    pub video: SplitPath<'a>,
    /// Manifest filename of the video
    pub dash_mpd: SplitPath<'a>,
    pub channel_id: Option<Cow<'a, str>>,
}

/// Data for textures
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialGraphicComponent<'a> {
    pub files: [SplitPath<'a>; 11],
    pub unk11_5: u32,
    pub unk13: u32,
    /// Unknown value, 6 for tga with coach, 1 for tga without
    pub unk14: u64,
    pub unk15: u64,
    pub unk26: u32,
}

impl Default for MaterialGraphicComponent<'static> {
    fn default() -> Self {
        Self {
            files: Default::default(),
            unk11_5: 0x3F80_0000,
            unk13: u32::MAX,
            unk14: 1,
            unk15: Default::default(),
            unk26: 1,
        }
    }
}

/// The data for the main video player
#[derive(Debug, Clone, PartialEq)]
pub struct UITextBox<'a> {
    pub string1: Option<Cow<'a, str>>,
    pub string2: Option<Cow<'a, str>>,
}
