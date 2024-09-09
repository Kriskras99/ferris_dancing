use std::{borrow::Cow, collections::HashMap};

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{f32be, i32be, u32be},
        read::{BinaryDeserialize, ReadAtExt, ReadError},
    },
    test_any, test_eq,
};
use hipstr::HipStr;
use ownable::IntoOwned;
use serde::{Deserialize, Serialize};
use tracing::{instrument, trace};
use ubiart_toolkit_json_types::Empty;
use ubiart_toolkit_shared_types::{errors::ParserError, Color};

use super::json;
use crate::utils::{Game, SplitPath, UniqueGameId};

pub fn parse<'a>(data: &'a [u8], ugi: UniqueGameId) -> Result<Tape<'a>, ParserError> {
    let tape = match ugi.game {
        Game::JustDance2022
        | Game::JustDance2021
        | Game::JustDance2020
        | Game::JustDance2019
        | Game::JustDance2018
        | Game::JustDance2017
        | Game::JustDance2016
        | Game::JustDanceChina => json::parse(data, false)?,
        Game::JustDance2015 => Tape::deserialize_with(data, ugi)?,
        _ => todo!(),
    };
    Ok(tape)
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Tape<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub clips: Vec<Clip<'a>>,
    pub tape_clock: u32,
    pub tape_bar_count: u32,
    pub free_resources_after_play: u32,
    #[serde(borrow)]
    pub map_name: HipStr<'a>,
    /// Not present in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub soundwich_event: Option<HipStr<'a>>,
}

impl Tape<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("Tape");
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

        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 0)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_any!(unk5, [0x0, 0x1])?;
        if unk5 == 1 {
            let _unk6 = reader.read_at_with::<Unknown9BC67FC4>(position, ugi)?;
        }
        let tape_clock = reader.read_at::<u32be>(position)?;
        let tape_bar_count = reader.read_at::<u32be>(position)?;
        let free_resources_after_play = reader.read_at::<u32be>(position)?;
        let map_name = reader.read_len_string_at::<u32be>(position)?.into();

        Ok(Self {
            class: None,
            clips,
            tape_clock,
            tape_bar_count,
            free_resources_after_play,
            map_name,
            soundwich_event: None,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
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
    #[serde(borrow, rename = "Unknown59FCC733Clip")]
    Unknown59FCC733(Unknown59FCC733Clip<'a>),
    #[serde(borrow, rename = "UnknownCBB7C029Clip")]
    UnknownCBB7C029(UnknownCBB7C029Clip<'a>),
    #[serde(borrow, rename = "Unknown5C944B01Clip")]
    Unknown5C944B01(Unknown5C944B01Clip<'a>),
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
            0x59FC_C733 => {
                Clip::Unknown59FCC733(reader.read_at_with::<Unknown59FCC733Clip>(position, ugi)?)
            }
            0xCBB7_C029 => {
                Clip::UnknownCBB7C029(reader.read_at_with::<UnknownCBB7C029Clip>(position, ugi)?)
            }
            0x5C94_4B01 => {
                Clip::Unknown5C944B01(reader.read_at_with::<Unknown5C944B01Clip>(position, ugi)?)
            }
            _ => return Err(ReadError::custom(format!("Unknown magic: 0x{magic:08x}"))),
        };
        Ok(clip)
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ActorEnableClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub actor_enable: u32,
}

impl<'a> BinaryDeserialize<'a> for ActorEnableClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        _reader: &'a (impl ReadAtExt + ?Sized),
        _position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AlphaClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for AlphaClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x8607_D582)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x2C)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let curve = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            curve,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CameraFeedClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub capture_type: u32,
    pub record_beat: u32,
    pub feed_type: u32,
}

impl<'a> BinaryDeserialize<'a> for CameraFeedClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        _reader: &'a (impl ReadAtExt + ?Sized),
        _position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ColorClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_red: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_green: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_blue: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for ColorClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xF61B_3A75)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x34)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let curve_red = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_green = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_blue = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            curve_red,
            curve_green,
            curve_blue,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityDancerClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(borrow)]
    pub dancer_country_code: HipStr<'a>,
    pub dancer_avatar_id: u32,
    #[serde(borrow)]
    pub dancer_name: HipStr<'a>,
}

impl<'a> BinaryDeserialize<'a> for CommunityDancerClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x0F95_B841)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x34)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let dancer_country_code = reader.read_len_string_at::<u32be>(position)?.into();
        let dancer_avatar_id = reader.read_at::<u32be>(position)?;
        let dancer_name = reader.read_len_string_at::<u32be>(position)?.into();

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            dancer_country_code,
            dancer_avatar_id,
            dancer_name,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FXClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub fx_name: StringOrId<'a>,
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
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x0F19_B038)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x34)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let fx_name_stringid = reader.read_at::<u32be>(position)?;
        let kill_particles_on_end = reader.read_at::<u32be>(position)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            fx_name: StringOrId::Id(fx_name_stringid),
            kill_particles_on_end,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GameplayEventClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub event_type: u32,
    #[serde(borrow)]
    pub custom_param: HipStr<'a>,
}

impl<'a> BinaryDeserialize<'a> for GameplayEventClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xCE73_233E)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x38)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let event_type = reader.read_at::<u32be>(position)?;
        let custom_param = reader.read_len_string_at::<u32be>(position)?.into();
        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            event_type,
            custom_param,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GoldEffectClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
        _ctx: Self::Ctx,
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

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct HideUserInterfaceClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u32>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub event_type: u32,
    #[serde(borrow)]
    pub custom_param: HipStr<'a>,
}

impl<'a> BinaryDeserialize<'a> for HideUserInterfaceClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x52E0_6A9A)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x38)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let event_type = reader.read_at::<u32be>(position)?;
        let custom_param = reader.read_len_string_at::<u32be>(position)?.into();

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            event_type,
            custom_param,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct KaraokeClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub pitch: f32,
    #[serde(borrow)]
    pub lyrics: HipStr<'a>,
    pub is_end_of_line: u8,
    pub content_type: u32,
    pub start_time_tolerance: u32,
    pub end_time_tolerance: u32,
    pub semitone_tolerance: f32,
}

impl<'a> BinaryDeserialize<'a> for KaraokeClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x6855_2A41)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x50)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let pitch = reader.read_at::<f32be>(position)?;
        let lyrics = reader.read_len_string_at::<u32be>(position)?.into();
        let is_end_of_line = reader.read_at::<u32be>(position)?;
        let content_type = reader.read_at::<u32be>(position)?;
        let start_time_tolerance = reader.read_at::<u32be>(position)?;
        let end_time_tolerance = reader.read_at::<u32be>(position)?;
        let semitone_tolerance = reader.read_at::<f32be>(position)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            pitch,
            lyrics,
            is_end_of_line: u8::try_from(is_end_of_line)?,
            content_type,
            start_time_tolerance,
            end_time_tolerance,
            semitone_tolerance,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicDiffuseAlphaClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_a: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicDiffuseAlphaClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xE684_12CA)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x34)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let layer_idx = reader.read_at::<u32be>(position)?;
        let uv_modifier_idx = reader.read_at::<u32be>(position)?;
        let curve_a = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            layer_idx,
            uv_modifier_idx,
            curve_a,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicDiffuseColorClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_r: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_g: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_b: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicDiffuseColorClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xC6FE_D58E)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x3C)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let layer_idx = reader.read_at::<u32be>(position)?;
        let uv_modifier_idx = reader.read_at::<u32be>(position)?;
        let curve_r = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_g = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_b = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            layer_idx,
            uv_modifier_idx,
            curve_r,
            curve_g,
            curve_b,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicEnableLayerClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
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
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x4D30_9320)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x34)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let layer_idx = reader.read_at::<u32be>(position)?;
        let uv_modifier_idx = reader.read_at::<u32be>(position)?;
        let layer_enabled = reader.read_at::<u32be>(position)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            layer_idx,
            uv_modifier_idx,
            layer_enabled: u8::try_from(layer_enabled)?,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVAnimRotationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_anim_angle: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_y: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicUVAnimRotationClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        _reader: &'a (impl ReadAtExt + ?Sized),
        _position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVRotationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_angle: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_y: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicUVRotationClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xDD4A_9D55)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x3C)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let layer_idx = reader.read_at::<u32be>(position)?;
        let uv_modifier_idx = reader.read_at::<u32be>(position)?;
        let curve_angle = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_pivot_x = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_pivot_y = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            layer_idx,
            uv_modifier_idx,
            curve_angle,
            curve_pivot_x,
            curve_pivot_y,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVScaleClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_scale_u: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_scale_v: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_pivot_y: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicUVScaleClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x511F_C7A5)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x40)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let layer_idx = reader.read_at::<u32be>(position)?;
        let uv_modifier_idx = reader.read_at::<u32be>(position)?;
        let curve_scale_u = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_scale_v = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_pivot_x = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_pivot_y = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            layer_idx,
            uv_modifier_idx,
            curve_scale_u,
            curve_scale_v,
            curve_pivot_x,
            curve_pivot_y,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVScrollClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_scroll_u: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_scroll_v: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicUVScrollClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x57E2_6726)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x38)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let layer_idx = reader.read_at::<u32be>(position)?;
        let uv_modifier_idx = reader.read_at::<u32be>(position)?;
        let curve_scroll_u = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_scroll_v = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            layer_idx,
            uv_modifier_idx,
            curve_scroll_u,
            curve_scroll_v,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialGraphicUVTranslationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub layer_idx: u32,
    #[serde(rename = "UVModifierIdx")]
    pub uv_modifier_idx: u32,
    #[serde(borrow)]
    pub curve_u: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_v: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for MaterialGraphicUVTranslationClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xC411_5B2E)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x38)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let layer_idx = reader.read_at::<u32be>(position)?;
        let uv_modifier_idx = reader.read_at::<u32be>(position)?;
        let curve_u = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_v = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            layer_idx,
            uv_modifier_idx,
            curve_u,
            curve_v,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MotionClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(borrow)]
    pub classifier_path: HipStr<'a>,
    pub gold_move: u8,
    pub coach_id: u8,
    pub move_type: u8,
    pub color: Color,
    #[serde(borrow, default)]
    pub motion_platform_specifics: HashMap<HipStr<'a>, MotionPlatformSpecific<'a>>,
}

impl<'a> BinaryDeserialize<'a> for MotionClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    #[instrument(skip(reader, position, ugi))]
    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x9553_84A1)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x6C)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let classifier_path = reader.read_at::<SplitPath>(position)?;
        let gold_move = reader.read_at::<u32be>(position)?;
        let coach_id = reader.read_at::<u32be>(position)?;
        let move_type = reader.read_at::<u32be>(position)?;
        // let unk2 = reader.read_at::<u32be>(position)?;
        // test_eq!(unk2, 0)?;
        let color = reader.read_at::<Color>(position)?;
        let n_motion_platform_specifics = reader.read_at::<u32be>(position)?;
        let mut motion_platform_specifics =
            HashMap::with_capacity(usize::try_from(n_motion_platform_specifics)?);
        for _ in 0..n_motion_platform_specifics {
            let platform = reader.read_at::<u32be>(position)?;
            let platform = match platform {
                0x1 => Cow::Borrowed("X360"),
                0x3 => Cow::Borrowed("ORBIS"),
                0xA => Cow::Borrowed("DURANGO"),
                _ => {
                    return Err(ReadError::custom(format!(
                        "Unknown platform: 0x{platform:x}, position: {position}"
                    )))
                }
            };
            let motion_platform_specific =
                reader.read_at_with::<MotionPlatformSpecific>(position, ugi)?;
            motion_platform_specifics.insert(platform.into(), motion_platform_specific);
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

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MotionPlatformSpecific<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0xC)?;
        let score_scale = reader.read_at::<f32be>(position)?;
        let scoring_mode = reader.read_at::<u32be>(position)?;
        let score_smoothing = reader.read_at::<u32be>(position)?;
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

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PictogramClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(borrow)]
    pub picto_path: HipStr<'a>,
    /// Only in nx2017-nx2018, only has non-empty values in nx2018
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub montage_path: Option<HipStr<'a>>,
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
        _ctx: Self::Ctx,
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

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProportionClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_y: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for ProportionClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x5477_75BC)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x30)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let curve_x = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_y = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            curve_x,
            curve_y,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Proportion3DClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_y: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_z: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for Proportion3DClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        _reader: &'a (impl ReadAtExt + ?Sized),
        _position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RotationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_y: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_z: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for RotationClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x7A9C_58B3)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x34)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let curve_x = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_y = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_z = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            curve_x,
            curve_y,
            curve_z,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SizeClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_y: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for SizeClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x52B8_9D18)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x30)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let curve_x = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_y = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            curve_x,
            curve_y,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SlotClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub bpm: f32,
    #[serde(borrow)]
    pub signature: HipStr<'a>,
    #[serde(borrow)]
    pub guid: HipStr<'a>,
}

impl<'a> BinaryDeserialize<'a> for SlotClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x896A_96B0)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x34)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let bpm = reader.read_at::<f32be>(position)?;
        let signature = reader.read_len_string_at::<u32be>(position)?.into();
        let guid = reader.read_len_string_at::<u32be>(position)?.into();

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            bpm,
            signature,
            guid,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SpawnActorClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(borrow)]
    pub actor_path: HipStr<'a>,
    #[serde(borrow)]
    pub actor_name: HipStr<'a>,
    pub spawn_position: (f32, f32, f32),
    #[serde(borrow)]
    pub parent_actor: HipStr<'a>,
}

impl<'a> BinaryDeserialize<'a> for SpawnActorClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xA247_B5D3)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x70)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let actor_path = reader.read_at::<SplitPath>(position)?.to_string().into();
        let actor_name = reader.read_len_string_at::<u32be>(position)?.into();
        let spawn_x = reader.read_at::<f32be>(position)?;
        let spawn_y = reader.read_at::<f32be>(position)?;
        let spawn_z = reader.read_at::<f32be>(position)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0x24)?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_eq!(unk3, 0)?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 0)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_path,
            actor_name,
            spawn_position: (spawn_x, spawn_y, spawn_z),
            parent_actor: HipStr::new(),
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SoundSetClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(borrow)]
    pub sound_set_path: HipStr<'a>,
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

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SoundwichClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    /// Not present in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub soundwich_event: Option<HipStr<'a>>,
}

impl<'a> BinaryDeserialize<'a> for SoundwichClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        _reader: &'a (impl ReadAtExt + ?Sized),
        _position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SoundwichClipWithId<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    /// Not present in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub soundwich_event: Option<HipStr<'a>>,
    pub soundwich_id: i32,
}

impl<'a> BinaryDeserialize<'a> for SoundwichClipWithId<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        _reader: &'a (impl ReadAtExt + ?Sized),
        _position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[allow(clippy::module_name_repetitions, reason = "Name is enforced by UbiArt")]
#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TapeLauncherClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub action: u32,
    /// Not in WiiU 2016
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tape_choice: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub tape_labels: Vec<HipStr<'a>>,
}

impl<'a> BinaryDeserialize<'a> for TapeLauncherClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    #[instrument(skip(reader, ctx))]
    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x115F_128D)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x34)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let action = reader.read_at::<u32be>(position)?;
        let unk2 = reader.read_at::<f32be>(position)?;
        trace!("Unk2: {unk2}");

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            action,
            tape_choice: None,
            tape_labels: Vec::new(),
        })
    }
}

#[allow(
    clippy::module_name_repetitions,
    reason = "Name is required by the engine"
)]
#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TapeReferenceClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(borrow)]
    pub path: HipStr<'a>,
    #[serde(rename = "Loop")]
    pub loop_it: u32,
}

impl<'a> BinaryDeserialize<'a> for TapeReferenceClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    #[instrument(skip(reader, position, _ctx))]
    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x0E1E_8158)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        trace!("Unk1: 0x{unk1:x}");
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let path = reader.read_at::<SplitPath>(position)?;
        let loop_it = reader.read_at::<u32be>(position)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        trace!("Unk2: 0x{unk2:x}");
        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            path: path.to_string().into(),
            loop_it,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TextClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
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
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xE5B3_34C8)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x2C)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let localization_key = reader.read_at::<u32be>(position)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            localization_key,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TextAreaSizeClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_max_width: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_max_height: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_area_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_area_y: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for TextAreaSizeClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        _reader: &'a (impl ReadAtExt + ?Sized),
        _position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TranslationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_indices: Vec<u8>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_x: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_y: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_z: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for TranslationClip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x36A3_12DC)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x34)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let curve_x = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_y = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_z = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            actor_indices: Vec::new(),
            target_actors,
            curve_x,
            curve_y,
            curve_z,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct VibrationClip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(borrow)]
    pub vibration_file_path: HipStr<'a>,
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
        _reader: &'a (impl ReadAtExt + ?Sized),
        _position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        todo!()
    }
}

impl VibrationClip<'_> {
    #[must_use]
    pub fn to_owned(self) -> VibrationClip<'static> {
        let class = None;
        let vibration_file_path = self.vibration_file_path.into_owned();
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

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Unknown59FCC733Clip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    #[serde(borrow)]
    pub curve_one: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_two: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_three: BezierCurveFloat<'a>,
    #[serde(borrow)]
    pub curve_four: BezierCurveFloat<'a>,
}

impl<'a> BinaryDeserialize<'a> for Unknown59FCC733Clip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x59FC_C733)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x38)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let curve_one = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_two = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_three = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;
        let curve_four = reader.read_at_with::<BezierCurveFloat>(position, ctx)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            target_actors,
            curve_one,
            curve_two,
            curve_three,
            curve_four,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnknownCBB7C029Clip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub target_actors: Vec<TargetActor<'a>>,
    pub unk2_stringid: u32,
    pub unk3: u32,
    pub unk4: f32,
    pub unk5: u32,
}

impl<'a> BinaryDeserialize<'a> for UnknownCBB7C029Clip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xCBB7_C029)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x3C)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let target_actors = reader
            .read_len_type_at_with::<u32be, TargetActor>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let unk2_stringid = reader.read_at::<u32be>(position)?;
        let unk3 = reader.read_at::<u32be>(position)?;
        let unk4 = reader.read_at::<f32be>(position)?;
        let unk5 = reader.read_at::<u32be>(position)?;

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            target_actors,
            unk2_stringid,
            unk3,
            unk4,
            unk5,
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Unknown5C944B01Clip<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub track_id: u32,
    pub is_active: u8,
    pub start_time: i32,
    pub duration: u32,
    pub string: HipStr<'a>,
}

impl<'a> BinaryDeserialize<'a> for Unknown5C944B01Clip<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x5C94_4B01)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x24)?;
        let id = reader.read_at::<u32be>(position)?;
        let track_id = reader.read_at::<u32be>(position)?;
        let is_active = reader.read_at::<u32be>(position)?;
        let start_time = reader.read_at::<i32be>(position)?;
        let duration = reader.read_at::<u32be>(position)?;
        let string = reader.read_len_string_at::<u32be>(position)?.into();

        Ok(Self {
            class: None,
            id,
            track_id,
            is_active: u8::try_from(is_active)?,
            start_time,
            duration,
            string,
        })
    }
}

pub struct Unknown9BC67FC4;

impl BinaryDeserialize<'_> for Unknown9BC67FC4 {
    type Ctx = UniqueGameId;
    type Output = Self;

    #[instrument(skip(reader, _ctx))]
    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x9BC6_7FC4)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x3C)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        trace!("Unk2: {unk2}");
        let unk3 = reader.read_at::<u32be>(position)?;
        test_any!(unk3, [0x100, 0x0400])?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_any!(unk4, [0x100, 0x0400])?;
        let unk5 = reader.read_at::<f32be>(position)?;
        test_any!(unk5, [100.0, 500.0])?;
        let unk6 = reader.read_at::<f32be>(position)?;
        test_eq!(unk6, 0.70)?;
        let unk7 = reader.read_at::<f32be>(position)?;
        test_eq!(unk7, 0.90)?;
        let unk8 = reader.read_at::<f32be>(position)?;
        test_eq!(unk8, 0.75)?;
        let unk9 = reader.read_at::<f32be>(position)?;
        test_eq!(unk9, 90.0)?;
        let unk10 = reader.read_at::<f32be>(position)?;
        test_eq!(unk10, 900.0)?;
        let unk11 = reader.read_at::<u32be>(position)?;
        test_eq!(unk11, 0x0A)?;
        let unk12 = reader.read_at::<u32be>(position)?;
        test_eq!(unk12, 0x07)?;
        let unk13 = reader.read_at::<f32be>(position)?;
        test_any!(unk13, [0.80, 0.90])?;
        let unk14 = reader.read_at::<u32be>(position)?;
        test_eq!(unk14, 0x0)?;
        let unk15 = reader.read_at::<u32be>(position)?;
        test_any!(unk15, [0xA, 0x75_6C00])?;

        Ok(Self)
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloat<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "Curve",
        deserialize_with = "deser_bezier_curve_float_value"
    )]
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
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x4)?;
        let value = reader.read_at_with::<BezierCurveFloatValue>(position, ctx)?;
        Ok(Self { class: None, value })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BezierCurveFloatValue<'a> {
    #[serde(borrow)]
    Empty(Empty<'a>),
    #[serde(borrow, rename = "BezierCurveFloatConstant")]
    Constant(BezierCurveFloatConstant<'a>),
    #[serde(borrow, rename = "BezierCurveFloatLinear")]
    Linear(BezierCurveFloatLinear<'a>),
    #[serde(borrow, rename = "BezierCurveFloatMulti")]
    Multi(BezierCurveFloatMulti<'a>),
}

impl Default for BezierCurveFloatValue<'_> {
    fn default() -> Self {
        BezierCurveFloatValue::Empty(Empty::default())
    }
}

impl<'a> BinaryDeserialize<'a> for BezierCurveFloatValue<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut temp_position = *position;
        let magic = reader.read_at::<u32be>(&mut temp_position)?;
        match magic {
            0xE2BC_4FB2 => Ok(Self::Multi(
                reader.read_at_with::<BezierCurveFloatMulti>(position, ctx)?,
            )),
            0x4DE6_D871 => Ok(Self::Linear(
                reader.read_at_with::<BezierCurveFloatLinear>(position, ctx)?,
            )),
            0xB791_4191 => Ok(Self::Constant(
                reader.read_at_with::<BezierCurveFloatConstant>(position, ctx)?,
            )),
            0xFFFF_FFFF => {
                *position = temp_position;
                Ok(BezierCurveFloatValue::Empty(Empty::default()))
            }
            _ => Err(ReadError::custom(format!("Unknown magic: 0x{magic:08x}"))),
        }
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatConstant<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub value: f32,
}

impl<'a> BinaryDeserialize<'a> for BezierCurveFloatConstant<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xB791_4191)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x8)?;
        let value = reader.read_at::<f32be>(position)?;
        Ok(Self { class: None, value })
    }
}

impl Default for BezierCurveFloatConstant<'static> {
    fn default() -> Self {
        Self {
            class: Some(HipStr::borrowed("BezierCurveFloatConstant")),
            value: Default::default(),
        }
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatLinear<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub value_left: (f32, f32),
    pub normal_left_out: (f32, f32),
    pub value_right: (f32, f32),
    pub normal_right_in: (f32, f32),
}

impl<'a> BinaryDeserialize<'a> for BezierCurveFloatLinear<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x4DE6_D871)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x24)?;
        let value_left_left = reader.read_at::<f32be>(position)?;
        let value_left_right = reader.read_at::<f32be>(position)?;
        let normal_left_out_left = reader.read_at::<f32be>(position)?;
        let normal_left_out_right = reader.read_at::<f32be>(position)?;
        let value_right_left = reader.read_at::<f32be>(position)?;
        let value_right_right = reader.read_at::<f32be>(position)?;
        let normal_right_in_left = reader.read_at::<f32be>(position)?;
        let normal_right_in_right = reader.read_at::<f32be>(position)?;

        Ok(Self {
            class: None,
            value_left: (value_left_left, value_left_right),
            normal_left_out: (normal_left_out_left, normal_left_out_right),
            value_right: (value_right_left, value_right_right),
            normal_right_in: (normal_right_in_left, normal_right_in_right),
        })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BezierCurveFloatMulti<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
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
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xE2BC_4FB2)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x14)?;
        let keys = reader
            .read_len_type_at_with::<u32be, KeyFloat>(position, ctx)?
            .collect::<Result<_, _>>()?;
        Ok(Self { class: None, keys })
    }
}

#[derive(IntoOwned, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct KeyFloat<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub value: (f32, f32),
    pub normal_in: (f32, f32),
    pub normal_out: (f32, f32),
}

impl<'a> BinaryDeserialize<'a> for KeyFloat<'a> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'a (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x18)?;
        let value_left = reader.read_at::<f32be>(position)?;
        let value_right = reader.read_at::<f32be>(position)?;
        let normal_in_left = reader.read_at::<f32be>(position)?;
        let normal_in_right = reader.read_at::<f32be>(position)?;
        let normal_out_left = reader.read_at::<f32be>(position)?;
        let normal_out_right = reader.read_at::<f32be>(position)?;

        Ok(Self {
            class: None,
            value: (value_left, value_right),
            normal_in: (normal_in_left, normal_in_right),
            normal_out: (normal_out_left, normal_out_right),
        })
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
                Ok(BezierCurveFloatValue::Empty(Empty::default()))
            } else {
                Err(err)
            }
        }
    }
}

pub struct ActorPaths;
impl<'de> BinaryDeserialize<'de> for ActorPaths {
    type Ctx = UniqueGameId;
    type Output = Vec<HipStr<'de>>;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let n_actors = reader.read_at::<u32be>(position)?;
        let mut actor_paths = Vec::with_capacity(usize::try_from(n_actors)?);
        for _ in 0..n_actors {
            let unk2 = reader.read_at::<u32be>(position)?;
            test_eq!(unk2, 0x24)?;
            let unk3 = reader.read_at::<u32be>(position)?;
            test_eq!(unk3, 0x0)?;
            let actor_path = reader.read_len_string_at::<u32be>(position)?.into();
            actor_paths.push(actor_path);
            let unk4 = reader.read_at::<u32be>(position)?;
            test_eq!(unk4, 0x0)?;
        }
        Ok(actor_paths)
    }
}

#[derive(Debug, IntoOwned, Serialize, Deserialize)]
pub struct TargetActor<'a> {
    #[serde(borrow)]
    pub path: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub scenes: Vec<HipStr<'a>>,
}

impl<'de> BinaryDeserialize<'de> for TargetActor<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x24)?;
        let n_scenes = reader.read_at::<u32be>(position)?;
        let mut scenes = Vec::with_capacity(usize::try_from(n_scenes)?);
        for _ in 0..n_scenes {
            let unk2 = reader.read_at::<u32be>(position)?;
            test_eq!(unk2, 0x10, "unk2 is wrong at {position}")?;
            let scene = reader.read_len_string_at::<u32be>(position)?.into();
            scenes.push(scene);
            let unk3 = reader.read_at::<u32be>(position)?;
            test_any!(unk3, [0x0, 0x1], "Padding is wrong at {position}")?;
        }
        let path = reader.read_len_string_at::<u32be>(position)?.into();
        let unk4 = reader.read_at::<u32be>(position)?;
        test_any!(unk4, [0x0, 0x1], "Padding is wrong at {position}")?;

        Ok(Self { path, scenes })
    }
}

#[derive(IntoOwned, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrId<'a> {
    #[serde(borrow)]
    String(HipStr<'a>),
    Id(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_JSON: &str = r#"{"__class": "BezierCurveFloat","Curve": {}}"#;
    #[test]
    fn test_empty_bezier_curve_float_value() {
        let curve: BezierCurveFloat = serde_json::from_str(EMPTY_JSON).unwrap();
        assert!(matches!(curve.value, BezierCurveFloatValue::Empty(_)));
    }

    const CONSTANT_JSON: &str = r#"{"__class":"BezierCurveFloat","Curve":{"__class":"BezierCurveFloatConstant","Value":0}}"#;
    #[test]
    fn test_constant_bezier_curve_float_value() {
        let curve: BezierCurveFloat = serde_json::from_str(CONSTANT_JSON).unwrap();
        assert!(matches!(curve.value, BezierCurveFloatValue::Constant(_)));
    }

    const MIXED_JSON: &str = r#"[{"__class": "BezierCurveFloat","Curve": {}},{"__class":"BezierCurveFloat","Curve":{"__class":"BezierCurveFloatConstant","Value":0}}]"#;
    #[test]
    fn test_mix_bezier_curve_float_value() {
        let _: Vec<BezierCurveFloat> = serde_json::from_str(MIXED_JSON).unwrap();
    }
}
