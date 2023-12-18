//! Contains the types that describe the usefull information in this filetype

use std::borrow::Cow;

use crate::utils::{errors::ParserError, SplitPath};

#[derive(Debug, Clone)]
pub struct Actor<'a> {
    pub tpl: SplitPath<'a>,
    pub unk1: u32,
    pub unk2: u32,
    pub unk2_5: u32,
    pub components: Vec<Component<'a>>,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentType {
    /// JD_AutoDanceComponent
    AutodanceComponent = 0x67B8_BB77,
    /// JD_BeatPulseComponent
    BeatPulseComponent = 0x7184_37A8,
    /// BoxInterpolatorComponent
    BoxInterpolatorComponent = 0xF513_60DA,
    /// CameraGraphicComponent
    CameraGraphicComponent = 0xC760_4FA1,
    /// ClearColorComponent
    ClearColorComponent = 0xAEBB_218B,
    /// ConvertedTmlTape_Component
    ConvertedTmlTapeComponent = 0xCD07_BB76,
    /// JD_CreditsComponent
    CreditsComponent = 0x342E_A4FC,
    /// JD_FixedCameraComponent
    FixedCameraComponent = 0x3D5D_EBA2,
    /// FXControllerComponent
    FXControllerComponent = 0x8D4F_FFB6,
    /// MasterTape
    MasterTape = 0x677B_269B,
    /// MaterialGraphicComponent
    MaterialGraphicComponent = 0x72B6_1FC5,
    /// JD_Carousel
    MusicTrackComponent = 0x27E4_80C0,
    /// JD_PictoComponent
    PictoComponent = 0xC316_BF34,
    /// PleoComponent
    PleoComponent = 0x1263_DAD9,
    /// PleoTextureGraphicComponent
    PleoTextureGraphicComponent = 0x0579_E81B,
    /// PropertyPatcher
    PropertyPatcher = 0xF719_B524,
    /// JD_RegistrationComponent
    RegistrationComponent = 0xE0A2_4B6D,
    /// SingleInstanceMesh3DComponent
    SingleInstanceMesh3DComponent = 0x53E3_2AF7,
    /// JD_SongDatabaseComponent
    SongDatabaseComponent = 0x4055_79FB,
    /// JD_SongDescComponent
    SongDescComponent = 0xE07F_CC3F,
    /// SoundComponent
    SoundComponent = 0x7DD8_643C,
    /// TapeCase_Component
    TapeCaseComponent = 0x231F_27DE,
    /// TextureGraphicComponent
    TextureGraphicComponent = 0x7B48_A9AE,
    /// UICarousel
    UICarousel = 0x8782_FE60,
    /// UITextBox
    UITextBox = 0xD10C_BEED,
    /// JD_UIWidgetGroupHUD_AutodanceRecorder
    UIWdigetGroupHUDAutodanceRecorder = 0x9F87_350C,
    /// JD_UIWidgetGroupHUD_Lyrics
    UIWidgetGroupHUDLyrics = 0xF22C_9426,
    /// ViewportUIComponent
    ViewportUIComponent = 0x6990_834C,
    /// JD_AvatarDescComponent
    AvatarDescComponent = 0x1759_E29D,
    /// JD_SkinDescComponent
    SkinDescComponent = 0x84EA_AE82,
    /// FxBankComponent
    FxBankComponent = 0x966B_519D,
    /// BezierTreeComponent
    BezierTreeComponent = 0x3236_CF4C,
    /// AFXPostProcessComponent
    AFXPostProcessComponent = 0x2B34_9E69,
}

impl TryFrom<u32> for ComponentType {
    type Error = ParserError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0579_E81B => Ok(Self::PleoTextureGraphicComponent),
            0x1263_DAD9 => Ok(Self::PleoComponent),
            0x1759_E29D => Ok(Self::AvatarDescComponent),
            0x231F_27DE => Ok(Self::TapeCaseComponent),
            0x27E4_80C0 => Ok(Self::MusicTrackComponent),
            0x2B34_9E69 => Ok(Self::AFXPostProcessComponent),
            0x3236_CF4C => Ok(Self::BezierTreeComponent),
            0x342E_A4FC => Ok(Self::CreditsComponent),
            0x3D5D_EBA2 => Ok(Self::FixedCameraComponent),
            0x4055_79FB => Ok(Self::SongDatabaseComponent),
            0x53E3_2AF7 => Ok(Self::SingleInstanceMesh3DComponent),
            0x677B_269B => Ok(Self::MasterTape),
            0x67B8_BB77 => Ok(Self::AutodanceComponent),
            0x6990_834C => Ok(Self::ViewportUIComponent),
            0x7184_37A8 => Ok(Self::BeatPulseComponent),
            0x72B6_1FC5 => Ok(Self::MaterialGraphicComponent),
            0x7B48_A9AE => Ok(Self::TextureGraphicComponent),
            0x7DD8_643C => Ok(Self::SoundComponent),
            0x84EA_AE82 => Ok(Self::SkinDescComponent),
            0x8782_FE60 => Ok(Self::UICarousel),
            0x8D4F_FFB6 => Ok(Self::FXControllerComponent),
            0x966B_519D => Ok(Self::FxBankComponent),
            0x9F87_350C => Ok(Self::UIWdigetGroupHUDAutodanceRecorder),
            0xAEBB_218B => Ok(Self::ClearColorComponent),
            0xC316_BF34 => Ok(Self::PictoComponent),
            0xC760_4FA1 => Ok(Self::CameraGraphicComponent),
            0xCD07_BB76 => Ok(Self::ConvertedTmlTapeComponent),
            0xD10C_BEED => Ok(Self::UITextBox),
            0xE07F_CC3F => Ok(Self::SongDescComponent),
            0xE0A2_4B6D => Ok(Self::RegistrationComponent),
            0xF22C_9426 => Ok(Self::UIWidgetGroupHUDLyrics),
            0xF513_60DA => Ok(Self::BoxInterpolatorComponent),
            0xF719_B524 => Ok(Self::PropertyPatcher),
            _ => Err(ParserError::custom(format!(
                "Found unexpected component type value: 0x{value:x}!"
            ))),
        }
    }
}

impl From<ComponentType> for u32 {
    // Type is repr(u32) thus 'as' is always safe
    #[allow(clippy::as_conversions)]
    fn from(value: ComponentType) -> Self {
        value as Self
    }
}

#[derive(Debug, Clone)]
pub struct Component<'a> {
    pub the_type: ComponentType,
    pub data: ComponentData<'a>,
}

/// Contains the component specific data
#[derive(Debug, Clone)]
pub enum ComponentData<'a> {
    /// Describes the main video player of a song
    PleoComponent(PleoComponent<'a>),
    /// Describes a 2d sprite
    MaterialGraphicComponent(Box<MaterialGraphicComponent<'a>>),
    CreditsComponent(CreditsComponent<'a>),
    UITextBox(UITextBox<'a>),
    None,
}

impl ComponentData<'_> {
    /// Convert this component data to a `MaterialGraphicComponent`.
    pub fn material_graphics_component(
        &self,
    ) -> Result<&MaterialGraphicComponent<'_>, ParserError> {
        if let ComponentData::MaterialGraphicComponent(mgc) = self {
            Ok(mgc)
        } else {
            Err(ParserError::custom(format!(
                "MaterialGraphicComponent not found in component data: {self:?}"
            )))
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
            unk11_5: 0x3F80_0000,
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
