//! Contains the types that describe the usefull information in this filetype

use std::borrow::Cow;

use anyhow::{anyhow, Error};

use crate::utils::SplitPath;

#[derive(Debug, Clone)]
pub struct Actor<'a> {
    pub tpl: SplitPath<'a>,
    pub unk1: u32,
    pub unk2: u32,
    pub unk2_5: u32,
    pub templates: Vec<Template<'a>>,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateType {
    /// JD_AutoDanceComponent
    AutodanceComponent = 0x67b8_bb77,
    /// JD_BeatPulseComponent
    BeatPulseComponent = 0x7184_37a8,
    /// BoxInterpolatorComponent
    BoxInterpolatorComponent = 0xf513_60da,
    /// CameraGraphicComponent
    CameraGraphicComponent = 0xc760_4fa1,
    /// ClearColorComponent
    ClearColorComponent = 0xaebb_218b,
    /// ConvertedTmlTape_Component
    ConvertedTmlTapeComponent = 0xcd07_bb76,
    /// JD_CreditsComponent
    CreditsComponent = 0x342e_a4fc,
    /// JD_FixedCameraComponent
    FixedCameraComponent = 0x3d5d_eba2,
    /// FXControllerComponent
    FXControllerComponent = 0x8d4f_ffb6,
    /// MasterTape
    MasterTape = 0x677b_269b,
    /// MaterialGraphicComponent
    MaterialGraphicComponent = 0x72b6_1fc5,
    /// JD_Carousel
    MusicTrackComponent = 0x27e4_80c0,
    /// JD_PictoComponent
    PictoComponent = 0xc316_bf34,
    /// PleoComponent
    PleoComponent = 0x1263_dad9,
    /// PleoTextureGraphicComponent
    PleoTextureGraphicComponent = 0x0579_e81b,
    /// PropertyPatcher
    PropertyPatcher = 0xf719_b524,
    /// JD_RegistrationComponent
    RegistrationComponent = 0xe0a2_4b6d,
    /// SingleInstanceMesh3DComponent
    SingleInstanceMesh3DComponent = 0x53e3_2af7,
    /// JD_SongDatabaseComponent
    SongDatabaseComponent = 0x4055_79fb,
    /// JD_SongDescComponent
    SongDescComponent = 0xe07f_cc3f,
    /// SoundComponent
    SoundComponent = 0x7dd8_643c,
    /// TapeCase_Component
    TapeCaseComponent = 0x231f_27de,
    /// TextureGraphicComponent
    TextureGraphicComponent = 0x7b48_a9ae,
    /// UICarousel
    UICarousel = 0x8782_fe60,
    /// UITextBox
    UITextBox = 0xd10c_beed,
    /// JD_UIWidgetGroupHUD_AutodanceRecorder
    UIWdigetGroupHUDAutodanceRecorder = 0x9f87_350c,
    /// JD_UIWidgetGroupHUD_Lyrics
    UIWidgetGroupHUDLyrics = 0xf22c_9426,
    /// ViewportUIComponent
    ViewportUIComponent = 0x6990_834c,
    /// JD_AvatarDescComponent
    AvatarDescComponent = 0x1759_e29d,
    /// JD_SkinDescComponent
    SkinDescComponent = 0x84ea_ae82,
    /// FxBankComponent
    FxBankComponent = 0x966b_519d,
    /// BezierTreeComponent
    BezierTreeComponent = 0x3236_cf4c,
    /// AFXPostProcessComponent
    AFXPostProcessComponent = 0x2b34_9e69,
}

impl TryFrom<u32> for TemplateType {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0579_e81b => Ok(Self::PleoTextureGraphicComponent),
            0x1263_dad9 => Ok(Self::PleoComponent),
            0x1759_e29d => Ok(Self::AvatarDescComponent),
            0x231f_27de => Ok(Self::TapeCaseComponent),
            0x27e4_80c0 => Ok(Self::MusicTrackComponent),
            0x2b34_9e69 => Ok(Self::AFXPostProcessComponent),
            0x3236_cf4c => Ok(Self::BezierTreeComponent),
            0x342e_a4fc => Ok(Self::CreditsComponent),
            0x3d5d_eba2 => Ok(Self::FixedCameraComponent),
            0x4055_79fb => Ok(Self::SongDatabaseComponent),
            0x53e3_2af7 => Ok(Self::SingleInstanceMesh3DComponent),
            0x677b_269b => Ok(Self::MasterTape),
            0x67b8_bb77 => Ok(Self::AutodanceComponent),
            0x6990_834c => Ok(Self::ViewportUIComponent),
            0x7184_37a8 => Ok(Self::BeatPulseComponent),
            0x72b6_1fc5 => Ok(Self::MaterialGraphicComponent),
            0x7b48_a9ae => Ok(Self::TextureGraphicComponent),
            0x7dd8_643c => Ok(Self::SoundComponent),
            0x84ea_ae82 => Ok(Self::SkinDescComponent),
            0x8782_fe60 => Ok(Self::UICarousel),
            0x8d4f_ffb6 => Ok(Self::FXControllerComponent),
            0x966b_519d => Ok(Self::FxBankComponent),
            0x9f87_350c => Ok(Self::UIWdigetGroupHUDAutodanceRecorder),
            0xaebb_218b => Ok(Self::ClearColorComponent),
            0xc316_bf34 => Ok(Self::PictoComponent),
            0xc760_4fa1 => Ok(Self::CameraGraphicComponent),
            0xcd07_bb76 => Ok(Self::ConvertedTmlTapeComponent),
            0xd10c_beed => Ok(Self::UITextBox),
            0xe07f_cc3f => Ok(Self::SongDescComponent),
            0xe0a2_4b6d => Ok(Self::RegistrationComponent),
            0xf22c_9426 => Ok(Self::UIWidgetGroupHUDLyrics),
            0xf513_60da => Ok(Self::BoxInterpolatorComponent),
            0xf719_b524 => Ok(Self::PropertyPatcher),
            _ => Err(anyhow!(
                "Found unexpected template type value: 0x{value:x}!"
            )),
        }
    }
}

impl From<TemplateType> for u32 {
    // Type is repr(u32) thus 'as' is always safe
    #[allow(clippy::as_conversions)]
    fn from(value: TemplateType) -> Self {
        value as Self
    }
}

#[derive(Debug, Clone)]
pub struct Template<'a> {
    pub the_type: TemplateType,
    pub data: TemplateData<'a>,
}

/// Contains the template specific data
#[derive(Debug, Clone)]
pub enum TemplateData<'a> {
    /// Describes the main video player of a song
    PleoComponent(PleoComponent<'a>),
    /// Describes a 2d sprite
    MaterialGraphicComponent(Box<MaterialGraphicComponent<'a>>),
    CreditsComponent(CreditsComponent<'a>),
    UITextBox(UITextBox<'a>),
    None,
}

impl TemplateData<'_> {
    /// Convert this template data to a `MaterialGraphicComponent`.
    ///
    /// # Errors
    /// Will error if this template data is not a `MaterialGraphicComponent`.
    pub fn material_graphics_component(&self) -> Result<&MaterialGraphicComponent<'_>, Error> {
        if let TemplateData::MaterialGraphicComponent(mgc) = self {
            Ok(mgc)
        } else {
            Err(anyhow!(
                "MaterialGraphicComponent not found in template data: {self:?}"
            ))
        }
    }
}

/// The data for the main video player
#[derive(Debug, Clone)]
pub struct CreditsComponent<'a> {
    pub lines: Vec<Cow<'a, str>>,
}

/// The data for the main video player
#[derive(Debug, Clone)]
pub struct PleoComponent<'a> {
    /// The filename of the video to play
    pub video: SplitPath<'a>,
    /// Manifest filename of the video
    pub dash_mpd: SplitPath<'a>,
    pub channel_id: Option<Cow<'a, str>>,
}

/// Data for textures
#[derive(Debug, Clone)]
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
            unk11_5: 0x3f80_0000,
            unk13: u32::MAX,
            unk14: 1,
            unk15: Default::default(),
            unk26: 1,
        }
    }
}

/// The data for the main video player
#[derive(Debug, Clone)]
pub struct UITextBox<'a> {
    pub string1: Option<Cow<'a, str>>,
    pub string2: Option<Cow<'a, str>>,
}
