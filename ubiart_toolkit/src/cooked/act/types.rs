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
    Autodance = 0x67b8_bb77,
    BeatPulseComponent = 0x7184_37a8,
    BoxInterpolatorComponent = 0xf513_60da,
    CameraGraphicComponent = 0xc760_4fa1,
    ClearColor = 0xaebb_218b,
    ConvertedTmlTape = 0xcd07_bb76,
    CreditsComponent = 0x342e_a4fc,
    FixedCameraComponent = 0x3d5d_eba2,
    FXController = 0x8d4f_ffb6,
    MasterTape = 0x677b_269b,
    MaterialGraphicComponent = 0x72b6_1fc5,
    MusicTrackComponent = 0x27e4_80c0,
    PictoComponent = 0xc316_bf34,
    PleoComponent = 0x1263_dad9,
    PleoTextureGraphicComponent = 0x0579_e81b,
    PropertyPatcher = 0xf719_b524,
    RegistrationComponent = 0xe0a2_4b6d,
    SingleInstanceMesh3D = 0x53e3_2af7,
    SongDatabase = 0x4055_79fb,
    SongDesc = 0xe07f_cc3f,
    SoundComponent = 0x7dd8_643c,
    TapeCase = 0x231f_27de,
    TextureGraphicComponent = 0x7b48_a9ae,
    UICarousel = 0x8782_fe60,
    UITextBox = 0xd10c_beed,
    UIWdigetGroupHUDAutodanceRecorder = 0x9f87_350c,
    UIWidgetGroupHUDLyrics = 0xf22c_9426,
    ViewportUI = 0x6990_834c,
    Unknown1 = 0x1759_e29d,
    Unknown2 = 0x84ea_ae82,
    Unknown3 = 0x966b_519d,
    Unknown4 = 0x3236_cf4c,
    Unknown5 = 0x2b34_9e69,
    Unknown6 = 0xaa55_b6bd,
}

impl TryFrom<u32> for TemplateType {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0579_e81b => Ok(Self::PleoTextureGraphicComponent),
            0x1263_dad9 => Ok(Self::PleoComponent),
            0x1759_e29d => Ok(Self::Unknown1),
            0x231f_27de => Ok(Self::TapeCase),
            0x27e4_80c0 => Ok(Self::MusicTrackComponent),
            0x2b34_9e69 => Ok(Self::Unknown5),
            0x3236_cf4c => Ok(Self::Unknown4),
            0x342e_a4fc => Ok(Self::CreditsComponent),
            0x3d5d_eba2 => Ok(Self::FixedCameraComponent),
            0x4055_79fb => Ok(Self::SongDatabase),
            0x53e3_2af7 => Ok(Self::SingleInstanceMesh3D),
            0x677b_269b => Ok(Self::MasterTape),
            0x67b8_bb77 => Ok(Self::Autodance),
            0x6990_834c => Ok(Self::ViewportUI),
            0x7184_37a8 => Ok(Self::BeatPulseComponent),
            0x72b6_1fc5 => Ok(Self::MaterialGraphicComponent),
            0x7b48_a9ae => Ok(Self::TextureGraphicComponent),
            0x7dd8_643c => Ok(Self::SoundComponent),
            0x84ea_ae82 => Ok(Self::Unknown2),
            0x8782_fe60 => Ok(Self::UICarousel),
            0x8d4f_ffb6 => Ok(Self::FXController),
            0x966b_519d => Ok(Self::Unknown3),
            0x9f87_350c => Ok(Self::UIWdigetGroupHUDAutodanceRecorder),
            0xaa55_b6bd => Ok(Self::Unknown6),
            0xaebb_218b => Ok(Self::ClearColor),
            0xc316_bf34 => Ok(Self::PictoComponent),
            0xc760_4fa1 => Ok(Self::CameraGraphicComponent),
            0xcd07_bb76 => Ok(Self::ConvertedTmlTape),
            0xd10c_beed => Ok(Self::UITextBox),
            0xe07f_cc3f => Ok(Self::SongDesc),
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
