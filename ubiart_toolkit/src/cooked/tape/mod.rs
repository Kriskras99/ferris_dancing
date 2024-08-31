use std::{borrow::Cow, collections::HashMap};

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{f32be, i32be, u32be, u64be},
        read::{BinaryDeserialize, ReadAtExt, ReadError},
    },
    test_eq,
};
use serde::{Deserialize, Serialize};
use tracing::{instrument, trace};
use ubiart_toolkit_shared_types::{errors::ParserError, Color};

use crate::utils::{Game, SplitPath, UniqueGameId};

pub fn parse(data: &[u8], ugi: UniqueGameId) -> Result<Tape<'_>, ParserError> {
    let tape = match ugi.game {
        Game::JustDance2022
        | Game::JustDance2021
        | Game::JustDance2020
        | Game::JustDance2019
        | Game::JustDance2018
        | Game::JustDance2017
        | Game::JustDance2016
        | Game::JustDanceChina => crate::cooked::json::parse(&data, false)?,
        Game::JustDance2015 => Tape::deserialize_with(data, ugi)?,
        _ => todo!(),
    };
    Ok(tape)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Tape<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clips: Vec<Clip<'a>>,
    pub tape_clock: u32,
    pub tape_bar_count: u32,
    pub free_resources_after_play: u32,
    pub map_name: Cow<'a, str>,
    /// Not present in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub soundwich_event: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_paths: Vec<Cow<'a, str>>,
}

impl Tape<'_> {
    pub const CLASS: &'static str = "Tape";
}

impl<'a> BinaryDeserialize<'a> for Tape<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    #[instrument(skip(reader, position, ugi))]
    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        if ugi.game != Game::JustDance2015 {
            return Err(ReadError::custom("Unsupported".into()));
        }
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 1)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        trace!("Unk2: 0x{unk2:x}");
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x9E84_5460, "Wrong magic for Tape")?;
        let unk3 = reader.read_at::<u32be>(position)?;
        trace!("Unk3: 0x{unk3:x}");
        let clips = reader
            .read_len_type_at_with::<u32be, Clip>(position, ugi)?
            .collect::<Result<Vec<_>, _>>()?;

        let unk4 = reader.read_at::<u64be>(position)?;
        test_eq!(unk4, 0)?;
        let tape_clock = reader.read_at::<u32be>(position)?;
        let tape_bar_count = reader.read_at::<u32be>(position)?;
        let free_resources_after_play = reader.read_at::<u32be>(position)?;
        let map_name = reader.read_len_string_at::<u32be>(position)?;

        Ok(Self {
            class: None,
            clips,
            tape_clock,
            tape_bar_count,
            free_resources_after_play,
            map_name,
            soundwich_event: None,
            actor_paths: Vec::new(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "__class")]
pub enum Clip<'a> {
    #[serde(borrow, rename = "ActorEnableClip")]
    ActorEnable(ActorEnableClip<'a>),
    #[serde(borrow, rename = "AlphaClip")]
    Alpha(AlphaClip<'a>),
    #[serde(borrow, rename = "CameraFeedClip")]
    CameraFeed(CameraFeedClip<'a>),
    #[serde(borrow, rename = "ColorClip")]
    Color(ColorClip<'a>),
    #[serde(borrow, rename = "CommunityDancerClip")]
    CommunityDancer(CommunityDancerClip<'a>),
    #[serde(borrow, rename = "FXClip")]
    FX(FXClip<'a>),
    #[serde(borrow, rename = "GameplayEventClip")]
    GameplayEvent(GameplayEventClip<'a>),
    #[serde(borrow, rename = "GoldEffectClip")]
    GoldEffect(GoldEffectClip<'a>),
    #[serde(borrow, rename = "HideUserInterfaceClip")]
    HideUserInterface(HideUserInterfaceClip<'a>),
    #[serde(borrow, rename = "KaraokeClip")]
    Karaoke(KaraokeClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicAlphaThresholdClip")]
    MaterialGraphicAlphaThreshold(MaterialGraphicDiffuseAlphaClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicDiffuseAlphaClip")]
    MaterialGraphicDiffuseAlpha(MaterialGraphicDiffuseAlphaClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicDiffuseColorClip")]
    MaterialGraphicDiffuseColor(MaterialGraphicDiffuseColorClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicEnableLayerClip")]
    MaterialGraphicEnableLayer(MaterialGraphicEnableLayerClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicUVAnimRotationClip")]
    MaterialGraphicUVAnimRotation(MaterialGraphicUVAnimRotationClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicUVRotationClip")]
    MaterialGraphicUVRotation(MaterialGraphicUVRotationClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicUVScaleClip")]
    MaterialGraphicUVScale(MaterialGraphicUVScaleClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicUVScrollClip")]
    MaterialGraphicUVScroll(MaterialGraphicUVScrollClip<'a>),
    #[serde(borrow, rename = "MaterialGraphicUVTranslationClip")]
    MaterialGraphicUVTranslation(MaterialGraphicUVTranslationClip<'a>),
    #[serde(borrow, rename = "MotionClip")]
    Motion(MotionClip<'a>),
    #[serde(borrow, rename = "PictogramClip")]
    Pictogram(PictogramClip<'a>),
    #[serde(borrow, rename = "ProportionClip")]
    Proportion(ProportionClip<'a>),
    #[serde(borrow, rename = "Proportion3DClip")]
    Proportion3D(Proportion3DClip<'a>),
    #[serde(borrow, rename = "RotationClip")]
    Rotation(RotationClip<'a>),
    #[serde(borrow, rename = "SizeClip")]
    Size(SizeClip<'a>),
    #[serde(borrow, rename = "SlotClip")]
    Slot(SlotClip<'a>),
    #[serde(borrow, rename = "SpawnActorClip")]
    SpawnActor(SpawnActorClip<'a>),
    #[serde(borrow, rename = "SoundSetClip")]
    SoundSet(SoundSetClip<'a>),
    #[serde(borrow, rename = "SoundwichClip")]
    Soundwich(SoundwichClip<'a>),
    #[serde(borrow, rename = "SoundwichClipWithId")]
    SoundwichWithId(SoundwichClipWithId<'a>),
    #[serde(borrow, rename = "TapeLauncherClip")]
    TapeLauncher(TapeLauncherClip<'a>),
    #[serde(borrow, rename = "TapeReferenceClip")]
    TapeReference(TapeReferenceClip<'a>),
    #[serde(borrow, rename = "TextClip")]
    Text(TextClip<'a>),
    #[serde(borrow, rename = "TextAreaSizeClip")]
    TextAreaSize(TextAreaSizeClip<'a>),
    #[serde(borrow, rename = "TranslationClip")]
    Translation(TranslationClip<'a>),
    #[serde(borrow, rename = "VibrationClip")]
    Vibration(VibrationClip<'a>),
}

impl<'a> BinaryDeserialize<'a> for Clip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut temp_position = *position;
        let magic = reader.read_at::<u32be>(&mut temp_position)?;
        let clip = match magic {
            0xD128_885D => {
                Clip::ActorEnable(reader.read_at_with::<ActorEnableClip>(position, ugi)?)
            }
            0x8607_D582 => Clip::Alpha(reader.read_at_with::<AlphaClip>(position, ugi)?),
            0x98A9_6A60 => Clip::CameraFeed(reader.read_at_with::<CameraFeedClip>(position, ugi)?),
            0xF61B_3A75 => Clip::Color(reader.read_at_with::<ColorClip>(position, ugi)?),
            0x0F95_B841 => {
                Clip::CommunityDancer(reader.read_at_with::<CommunityDancerClip>(position, ugi)?)
            }
            0x0F19_B038 => Clip::FX(reader.read_at_with::<FXClip>(position, ugi)?),
            0xCE73_233E => {
                Clip::GameplayEvent(reader.read_at_with::<GameplayEventClip>(position, ugi)?)
            }
            0xFD69_B110 => Clip::GoldEffect(reader.read_at_with::<GoldEffectClip>(position, ugi)?),
            0x52E0_6A9A => Clip::HideUserInterface(
                reader.read_at_with::<HideUserInterfaceClip>(position, ugi)?,
            ),
            0x6855_2A41 => Clip::Karaoke(reader.read_at_with::<KaraokeClip>(position, ugi)?),
            0xC236_72A1 => Clip::MaterialGraphicAlphaThreshold(
                reader.read_at_with::<MaterialGraphicDiffuseAlphaClip>(position, ugi)?,
            ),
            0xE684_12CA => Clip::MaterialGraphicDiffuseAlpha(
                reader.read_at_with::<MaterialGraphicDiffuseAlphaClip>(position, ugi)?,
            ),
            0xC6FE_D58E => Clip::MaterialGraphicDiffuseColor(
                reader.read_at_with::<MaterialGraphicDiffuseColorClip>(position, ugi)?,
            ),
            0x4D30_9320 => Clip::MaterialGraphicEnableLayer(
                reader.read_at_with::<MaterialGraphicEnableLayerClip>(position, ugi)?,
            ),
            0x2E01_F676 => Clip::MaterialGraphicUVAnimRotation(
                reader.read_at_with::<MaterialGraphicUVAnimRotationClip>(position, ugi)?,
            ),
            0xDD4A_9D55 => Clip::MaterialGraphicUVRotation(
                reader.read_at_with::<MaterialGraphicUVRotationClip>(position, ugi)?,
            ),
            0x511F_C7A5 => Clip::MaterialGraphicUVScale(
                reader.read_at_with::<MaterialGraphicUVScaleClip>(position, ugi)?,
            ),
            0x57E2_6726 => Clip::MaterialGraphicUVScroll(
                reader.read_at_with::<MaterialGraphicUVScrollClip>(position, ugi)?,
            ),
            0xC411_5B2E => Clip::MaterialGraphicUVTranslation(
                reader.read_at_with::<MaterialGraphicUVTranslationClip>(position, ugi)?,
            ),
            0x9553_84A1 => Clip::Motion(reader.read_at_with::<MotionClip>(position, ugi)?),
            0x52EC_8962 => Clip::Pictogram(reader.read_at_with::<PictogramClip>(position, ugi)?),
            0x5477_75BC => Clip::Proportion(reader.read_at_with::<ProportionClip>(position, ugi)?),
            0x1F11_BC9A => {
                Clip::Proportion3D(reader.read_at_with::<Proportion3DClip>(position, ugi)?)
            }
            0x7A9C_58B3 => Clip::Rotation(reader.read_at_with::<RotationClip>(position, ugi)?),
            0x52B8_9D18 => Clip::Size(reader.read_at_with::<SizeClip>(position, ugi)?),
            0x896A_96B0 => Clip::Slot(reader.read_at_with::<SlotClip>(position, ugi)?),
            0xA247_B5D3 => Clip::SpawnActor(reader.read_at_with::<SpawnActorClip>(position, ugi)?),
            0x2D8C_885B => Clip::SoundSet(reader.read_at_with::<SoundSetClip>(position, ugi)?),
            0x9FF5_7F95 => Clip::Soundwich(reader.read_at_with::<SoundwichClip>(position, ugi)?),
            0x7B8F_9D7B => {
                Clip::SoundwichWithId(reader.read_at_with::<SoundwichClipWithId>(position, ugi)?)
            }
            0x115F_128D => {
                Clip::TapeLauncher(reader.read_at_with::<TapeLauncherClip>(position, ugi)?)
            }
            0x0E1E_8158 => {
                Clip::TapeReference(reader.read_at_with::<TapeReferenceClip>(position, ugi)?)
            }
            0xE5B3_34C8 => Clip::Text(reader.read_at_with::<TextClip>(position, ugi)?),
            0x9B85_16EB => {
                Clip::TextAreaSize(reader.read_at_with::<TextAreaSizeClip>(position, ugi)?)
            }
            0x36A3_12DC => {
                Clip::Translation(reader.read_at_with::<TranslationClip>(position, ugi)?)
            }
            0x101F_9D2B => Clip::Vibration(reader.read_at_with::<VibrationClip>(position, ugi)?),
            _ => return Err(ReadError::custom(format!("Unknown magic: 0x{magic:08x}"))),
        };
        Ok(clip)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ActorEnableClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub actor_enable: u32,
}

impl<'a> BinaryDeserialize<'a> for ActorEnableClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AlphaClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for AlphaClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CameraFeedClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub capture_type: u32,
    pub record_beat: u32,
    pub feed_type: u32,
}

impl<'a> BinaryDeserialize<'a> for CameraFeedClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ColorClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_red: Curve<'a>,
    pub curve_green: Curve<'a>,
    pub curve_blue: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for ColorClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityDancerClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub dancer_country_code: Cow<'a, str>,
    pub dancer_avatar_id: u32,
    pub dancer_name: Cow<'a, str>,
}

impl<'a> BinaryDeserialize<'a> for CommunityDancerClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FXClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub fx_name: Cow<'a, str>,
    pub kill_particles_on_end: u32,
}

impl<'a> BinaryDeserialize<'a> for FXClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GameplayEventClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub event_type: u32,
    pub custom_param: Cow<'a, str>,
}

impl<'a> BinaryDeserialize<'a> for GameplayEventClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GoldEffectClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default)]
    pub effect_type: u8,
}

impl<'a> BinaryDeserialize<'a> for GoldEffectClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xFD69_B110)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x1C)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let effect_type = reader.read_at::<u32be>(position)?;
        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            effect_type: u8::try_from(effect_type)?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct HideUserInterfaceClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u32>,
    pub event_type: u32,
    pub custom_param: Cow<'a, str>,
}

impl<'a> BinaryDeserialize<'a> for HideUserInterfaceClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    #[instrument(skip(reader, position, _ctx))]
    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x52E0_6A9A)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        trace!("Unk1: 0x{unk1:x}");
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let actor_indices = reader
            .read_len_type_at::<u32be, u32be>(position)?
            .collect::<Result<_, _>>()?;
        let event_type = reader.read_at::<u32be>(position)?;
        let custom_param = reader.read_len_string_at::<u32be>(position)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices,
            event_type,
            custom_param,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct KaraokeClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub pitch: f32,
    pub lyrics: Cow<'a, str>,
    pub is_end_of_line: u8,
    pub content_type: u8,
    pub start_time_tolerance: u8,
    pub end_time_tolerance: u8,
    pub semitone_tolerance: f32,
}

impl<'a> BinaryDeserialize<'a> for KaraokeClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicDiffuseAlphaClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_a: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicDiffuseAlphaClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicDiffuseColorClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_r: Curve<'a>,
    pub curve_g: Curve<'a>,
    pub curve_b: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicDiffuseColorClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicEnableLayerClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u8,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u8,
    pub layer_enabled: u8,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicEnableLayerClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVAnimRotationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_anim_angle: Curve<'a>,
    pub curve_pivot_x: Curve<'a>,
    pub curve_pivot_y: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicUVAnimRotationClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVRotationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_angle: Curve<'a>,
    pub curve_pivot_x: Curve<'a>,
    pub curve_pivot_y: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicUVRotationClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVScaleClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_scale_u: Curve<'a>,
    pub curve_scale_v: Curve<'a>,
    pub curve_pivot_x: Curve<'a>,
    pub curve_pivot_y: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicUVScaleClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVScrollClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_scroll_u: Curve<'a>,
    pub curve_scroll_v: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicUVScrollClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVTranslationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    pub curve_u: Curve<'a>,
    pub curve_v: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicUVTranslationClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MotionClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub classifier_path: Cow<'a, str>,
    pub gold_move: u8,
    pub coach_id: u8,
    pub move_type: u8,
    pub color: Color,
    #[serde(default)]
    pub motion_platform_specifics: HashMap<Cow<'a, str>, MotionPlatformSpecific<'a>>,
}

impl<'a> BinaryDeserialize<'a> for MotionClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x9553_84A1)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let classifier_path = reader.read_at::<SplitPath>(position)?;
        let gold_move = reader.read_at::<u32be>(position)?;
        let coach_id = reader.read_at::<u32be>(position)?;
        let move_type = reader.read_at::<u32be>(position)?;
        let color = reader.read_at::<Color>(position)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0x3F80_0000)?;
        let mut motion_platform_specifics = HashMap::new();
        for _ in 0..reader.read_at::<u32be>(position)? {
            let platform = reader.read_at::<u32be>(position)?;
            let platform = match platform {
                0x1 => Cow::Borrowed("X360"),
                0x3 => Cow::Borrowed("ORBIS"),
                0xA => Cow::Borrowed("DURANGO"),
                _ => {
                    return Err(ReadError::custom(format!(
                        "Unknown platform: 0x{platform:x}"
                    )))
                }
            };
            let motion_platform_specific =
                reader.read_at_with::<MotionPlatformSpecific>(position, ugi)?;
            motion_platform_specifics.insert(platform, motion_platform_specific);
        }
        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            classifier_path: classifier_path.to_string().into(),
            gold_move: u8::try_from(gold_move)?,
            coach_id: u8::try_from(coach_id)?,
            move_type: u8::try_from(move_type)?,
            color,
            motion_platform_specifics,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MotionPlatformSpecific<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub score_scale: f32,
    pub score_smoothing: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoring_mode: Option<u32>,
    /// Not used in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low_threshold: Option<f32>,
    /// Not used in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_threshold: Option<f32>,
}

impl<'a> BinaryDeserialize<'a> for MotionPlatformSpecific<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let score_scale = reader.read_at::<f32be>(position)?;
        let score_smoothing = reader.read_at::<u32be>(position)?;
        let scoring_mode = reader.read_at::<u32be>(position)?;
        Ok(Self {
            class: None,
            score_scale,
            score_smoothing,
            scoring_mode: Some(scoring_mode),
            low_threshold: None,
            high_threshold: None,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PictogramClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub picto_path: Cow<'a, str>,
    /// Only in nx2017-nx2018, only has non-empty values in nx2018
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub montage_path: Option<Cow<'a, str>>,
    /// Only in nx2017-nx2018
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atl_index: Option<u32>,
    pub coach_count: u32,
}

impl<'a> BinaryDeserialize<'a> for PictogramClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x52EC_8962)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x38)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let picto_path = reader.read_at::<SplitPath>(position)?;
        let coach_count = reader.read_at::<u32be>(position)?;
        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            picto_path: picto_path.to_string().into(),
            montage_path: None,
            atl_index: None,
            coach_count,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProportionClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_x: Curve<'a>,
    pub curve_y: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for ProportionClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Proportion3DClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_x: Curve<'a>,
    pub curve_y: Curve<'a>,
    pub curve_z: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for Proportion3DClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RotationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_x: Curve<'a>,
    pub curve_y: Curve<'a>,
    pub curve_z: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for RotationClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SizeClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_x: Curve<'a>,
    pub curve_y: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for SizeClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SlotClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub bpm: f32,
    pub signature: Cow<'a, str>,
    pub guid: Cow<'a, str>,
}

impl<'a> BinaryDeserialize<'a> for SlotClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SpawnActorClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub actor_path: Cow<'a, str>,
    pub actor_name: Cow<'a, str>,
    pub spawn_position: (f32, f32, f32),
    pub parent_actor: Cow<'a, str>,
}

impl<'a> BinaryDeserialize<'a> for SpawnActorClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SoundSetClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub sound_set_path: Cow<'a, str>,
    pub sound_channel: i32,
    #[serde(default)]
    pub start_offset: u32,
    pub stops_on_end: u32,
    pub accounted_for_duration: u32,
}

impl<'a> BinaryDeserialize<'a> for SoundSetClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    #[instrument(skip(reader, position, _ctx))]
    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x2D8C_885B)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        trace!("Unk1: 0x{unk1:x}");
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let sound_set_path = reader.read_at::<SplitPath>(position)?;
        let sound_channel = reader.read_at::<i32be>(position)?;
        let stops_on_end = reader.read_at::<u32be>(position)?;
        let accounted_for_duration = reader.read_at::<u32be>(position)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            sound_set_path: sound_set_path.to_string().into(),
            sound_channel,
            start_offset: 0,
            stops_on_end,
            accounted_for_duration,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SoundwichClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    /// Not present in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub soundwich_event: Option<Cow<'a, str>>,
}

impl<'a> BinaryDeserialize<'a> for SoundwichClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SoundwichClipWithId<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    /// Not present in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub soundwich_event: Option<Cow<'a, str>>,
    pub soundwich_id: i32,
}

impl<'a> BinaryDeserialize<'a> for SoundwichClipWithId<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[allow(clippy::module_name_repetitions, reason = "Name is enforced by UbiArt")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TapeLauncherClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub action: u32,
    /// Not in WiiU 2016
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tape_choice: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tape_labels: Vec<Cow<'a, str>>,
}

impl<'a> BinaryDeserialize<'a> for TapeLauncherClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[allow(
    clippy::module_name_repetitions,
    reason = "Name is required by the engine"
)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TapeReferenceClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub path: Cow<'a, str>,
    #[serde(rename = "Loop")]
    pub loop_it: u32,
}

impl<'a> BinaryDeserialize<'a> for TapeReferenceClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TextClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub localization_key: u32,
}

impl<'a> BinaryDeserialize<'a> for TextClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TextAreaSizeClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_max_width: Curve<'a>,
    pub curve_max_height: Curve<'a>,
    pub curve_area_x: Curve<'a>,
    pub curve_area_y: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for TextAreaSizeClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TranslationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    pub curve_x: Curve<'a>,
    pub curve_y: Curve<'a>,
    pub curve_z: Curve<'a>,
}

impl<'a> BinaryDeserialize<'a> for TranslationClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct VibrationClip<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub vibration_file_path: Cow<'a, str>,
    #[serde(rename = "Loop")]
    pub loop_it: u8,
    pub device_side: u8,
    pub player_id: i8,
    pub context: u32,
    pub start_time_offset: f32,
    pub modulation: f32,
}

impl<'a> BinaryDeserialize<'a> for VibrationClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

impl VibrationClip<'_> {
    #[must_use]
    pub fn to_owned(self) -> VibrationClip<'static> {
        let class = None;
        let vibration_file_path = Cow::Owned(self.vibration_file_path.into_owned());
        VibrationClip {
            class,
            id: self.id,
            track_id: self.track_id,
            is_active: self.is_active,
            start_time: self.start_time,
            duration: self.duration,
            vibration_file_path,
            loop_it: self.loop_it,
            device_side: self.device_side,
            player_id: self.player_id,
            context: self.context,
            start_time_offset: self.start_time_offset,
            modulation: self.modulation,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "__class")]
pub enum Curve<'a> {
    #[serde(borrow)]
    BezierCurveFloat(BezierCurveFloat<'a>),
}

impl<'a> BinaryDeserialize<'a> for Curve<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

impl Default for Curve<'static> {
    fn default() -> Self {
        Self::BezierCurveFloat(BezierCurveFloat::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloat<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "Curve", deserialize_with = "deser_bezier_curve_float_value")]
    pub value: BezierCurveFloatValue<'a>,
}

impl<'a> BinaryDeserialize<'a> for BezierCurveFloat<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(tag = "__class")]
pub enum BezierCurveFloatValue<'a> {
    #[default]
    Empty,
    #[serde(borrow, rename = "BezierCurveFloatConstant")]
    Constant(BezierCurveFloatConstant<'a>),
    #[serde(borrow, rename = "BezierCurveFloatLinear")]
    Linear(BezierCurveFloatLinear<'a>),
    #[serde(borrow, rename = "BezierCurveFloatMulti")]
    Multi(BezierCurveFloatMulti<'a>),
}

impl<'a> BinaryDeserialize<'a> for BezierCurveFloatValue<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatConstant<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub value: f64,
}

impl<'a> BinaryDeserialize<'a> for BezierCurveFloatConstant<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

impl Default for BezierCurveFloatConstant<'static> {
    fn default() -> Self {
        Self {
            class: Some("BezierCurveFloatConstant"),
            value: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatLinear<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub value_left: (f64, f64),
    pub normal_left_out: (f64, f64),
    pub value_right: (f64, f64),
    pub normal_right_in: (f64, f64),
}

impl<'a> BinaryDeserialize<'a> for BezierCurveFloatLinear<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatMulti<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub keys: Vec<KeyFloat<'a>>,
}

impl<'a> BinaryDeserialize<'a> for BezierCurveFloatMulti<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct KeyFloat<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub value: (f64, f64),
    pub normal_in: (f64, f64),
    pub normal_out: (f64, f64),
}

impl<'a> BinaryDeserialize<'a> for KeyFloat<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

/// Deserialize a [`BezierCurveFloatValue`] which is weirdly formattend in the JSON files
fn deser_bezier_curve_float_value<'de, D>(deser: D) -> Result<BezierCurveFloatValue<'de>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let bcfv: Result<BezierCurveFloatValue, D::Error> = Deserialize::deserialize(deser);
    match bcfv {
        Ok(something) => Ok(something),
        Err(err) => {
            if err.to_string().as_str() == "missing field `__class`" {
                Ok(BezierCurveFloatValue::Empty)
            } else {
                Err(err)
            }
        }
    }
}
