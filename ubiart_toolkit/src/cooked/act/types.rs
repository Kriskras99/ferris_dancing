//! Contains the types that describe the usefull information in this filetype

use std::borrow::Cow;

use crate::utils::{errors::ParserError, SplitPath};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Actor<'a> {
    pub tpl: SplitPath<'a>,
    pub unk1: u32,
    pub unk2: u32,
    pub unk2_5: u32,
    pub components: Vec<Component<'a>>,
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Actor<'a> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Actor {
            tpl: u.arbitrary()?,
            unk1: *u.choose(&[
                0x0,
                0x3D23_D70A,
                0x3DCC_CCCD,
                0x3F66_6C4C,
                0x3F80_0000,
                0x4000_0000,
            ])?,
            unk2: *u.choose(&[
                0x3F00_0000,
                0x3F80_0000,
                0x4240_0000,
                0x4320_0000,
                0x4420_0000,
                0x4422_8000,
            ])?,
            unk2_5: *u.choose(&[
                0x3F00_0000,
                0x3F80_0000,
                0x4120_0000,
                0x4240_0000,
                0x4320_0000,
            ])?,
            components: u.arbitrary()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Component<'a> {
    AutodanceComponent,
    BeatPulseComponent,
    BoxInterpolatorComponent,
    CameraGraphicComponent,
    Carousel,
    ClearColorComponent,
    ConvertedTmlTapeComponent,
    CreditsComponent(CreditsComponent<'a>),
    FixedCameraComponent,
    FXControllerComponent,
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
            Component::BoxInterpolatorComponent => 0xF513_60DA,
            // CameraGraphicComponent
            Component::CameraGraphicComponent => 0xC760_4FA1,
            // ClearColorComponent
            Component::ClearColorComponent => 0xAEBB_218B,
            // ConvertedTmlTape_Component
            Component::ConvertedTmlTapeComponent => 0xCD07_BB76,
            // JD_CreditsComponent
            Component::CreditsComponent(_) => 0x342E_A4FC,
            // JD_FixedCameraComponent
            Component::FixedCameraComponent => 0x3D5D_EBA2,
            // FXControllerComponent
            Component::FXControllerComponent => 0x8D4F_FFB6,
            // MasterTape
            Component::MasterTape => 0x677B_269B,
            // MaterialGraphicComponent
            Component::MaterialGraphicComponent(_) => 0x72B6_1FC5,
            // JD_Carousel
            Component::Carousel => 0x27E4_80C0,
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

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Component<'a> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let id = *u.choose(&[
            0x67B8_BB77u32,
            0x677B_269B,
            0xC316_BF34,
            0x4055_79FB,
            0xE07F_CC3F,
            0x231F_27DE,
            0x1759_E29D,
            0x84EA_AE82,
            0x72B6_1FC5,
            0x1263_DAD9,
            0x0579_E81B,
        ])?;
        let component = match id {
            0x67B8_BB77 => Component::AutodanceComponent,
            0x677B_269B => Component::MasterTape,
            0xC316_BF34 => Component::PictoComponent,
            0x4055_79FB => Component::SongDatabaseComponent,
            0xE07F_CC3F => Component::SongDescComponent,
            0x231F_27DE => Component::TapeCaseComponent,
            0x1759_E29D => Component::AvatarDescComponent,
            0x84EA_AE82 => Component::SkinDescComponent,
            0x72B6_1FC5 => Component::MaterialGraphicComponent(u.arbitrary()?),
            0x1263_DAD9 => Component::PleoComponent(u.arbitrary()?),
            0x0579_E81B => Component::PleoTextureGraphicComponent(u.arbitrary()?),
            _ => unreachable!(),
        };
        Ok(component)
    }
}

/// The data for the main video player
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct CreditsComponent<'a> {
    pub lines: Vec<Cow<'a, str>>,
}

/// The data for the main video player
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct PleoComponent<'a> {
    /// The filename of the video to play
    pub video: SplitPath<'a>,
    /// Manifest filename of the video
    pub dash_mpd: SplitPath<'a>,
    pub channel_id: Option<Cow<'a, str>>,
}

/// Data for textures
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialGraphicComponent<'a> {
    pub files: [SplitPath<'a>; 11],
    pub unk11_5: u32,
    pub unk13: u32,
    /// Unknown value, 6 for tga with coach, 1 for tga without
    pub unk14: u64,
    pub unk15: u64,
    pub unk26: u32,
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for MaterialGraphicComponent<'a> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self {
            files: u.arbitrary()?,
            unk11_5: *u.choose(&[0x3F80_0000u32, 0x0])?,
            unk13: *u.choose(&[0xFFFF_FFFFu32, 0x1])?,
            unk14: *u.choose(&[0x1u64, 0x2, 0x3, 0x6, 0x9])?,
            unk15: *u.choose(&[
                0x0u64,
                0x3E2E_147B,
                0xC080_0000,
                0x3E99_999A_BDCC_CCCD,
                0xBDE1_47AE_3E61_47AE,
            ])?,
            unk26: *u.choose(&[0x1, 0x2, 0x3, 0x6, 0x9])?,
        })
    }
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct UITextBox<'a> {
    pub string1: Option<Cow<'a, str>>,
    pub string2: Option<Cow<'a, str>>,
}
