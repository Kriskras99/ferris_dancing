use std::collections::HashMap;

use dotstar_toolkit_utils::bytes::{
    primitives::{f32be, i32be, u32be},
    read::{BinaryDeserialize, ReadAtExt, ReadError},
};
use hipstr::HipStr;
use test_eq::{test_any, test_eq};
use tracing::{instrument, trace};
use ubiart_toolkit_shared_types::Color;

use super::{
    AlphaClip, BezierCurveFloat, BezierCurveFloatConstant, BezierCurveFloatLinear,
    BezierCurveFloatMulti, BezierCurveFloatValue, Clip, ColorClip, CommunityDancerClip, FXClip,
    GameplayEventClip, GoldEffectClip, HideUserInterfaceClip, KaraokeClip, KeyFloat,
    MaterialGraphicDiffuseAlphaClip, MaterialGraphicDiffuseColorClip,
    MaterialGraphicEnableLayerClip, MaterialGraphicUVRotationClip, MaterialGraphicUVScaleClip,
    MaterialGraphicUVScrollClip, MaterialGraphicUVTranslationClip, MotionClip,
    MotionPlatformSpecific, PictogramClip, ProportionClip, RotationClip, SizeClip, SlotClip,
    SoundSetClip, SpawnActorClip, StringOrId, Tape, TapeLauncherClip, TapeReferenceClip,
    TargetActor, TextClip, TranslationClip, Unknown59FCC733Clip, Unknown5C944B01Clip,
    UnknownCBB7C029Clip,
};
use crate::{
    shared_json_types::Empty,
    utils::{Game, SplitPath, UniqueGameId},
};

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
        let map_name = reader.read_len_string_at::<u32be>(position)?;

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
                return Err(ReadError::custom(
                    "ActorEnable is not used in binary formats".into(),
                ))
            }
            0x8607_D582 => Clip::Alpha(reader.read_at_with::<AlphaClip>(position, ugi)?),
            0x98A9_6A60 => {
                return Err(ReadError::custom(
                    "CameraFeed is not used in binary formats".into(),
                ))
            }
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
            0x2E01_F676 => {
                return Err(ReadError::custom(
                    "MaterialGraphicUVAnimRotation is not used in binary formats".into(),
                ))
            }
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
                return Err(ReadError::custom(
                    "Proportion3D is not used in binary formats".into(),
                ))
            }
            0x7A9C_58B3 => Clip::Rotation(reader.read_at_with::<RotationClip>(position, ugi)?),
            0x52B8_9D18 => Clip::Size(reader.read_at_with::<SizeClip>(position, ugi)?),
            0x896A_96B0 => Clip::Slot(reader.read_at_with::<SlotClip>(position, ugi)?),
            0xA247_B5D3 => Clip::SpawnActor(reader.read_at_with::<SpawnActorClip>(position, ugi)?),
            0x2D8C_885B => Clip::SoundSet(reader.read_at_with::<SoundSetClip>(position, ugi)?),
            0x9FF5_7F95 => {
                return Err(ReadError::custom(
                    "SoundWich is not used in binary formats".into(),
                ))
            }
            0x7B8F_9D7B => {
                return Err(ReadError::custom(
                    "SoundWichWithId is not used in binary formats".into(),
                ))
            }
            0x115F_128D => {
                Clip::TapeLauncher(reader.read_at_with::<TapeLauncherClip>(position, ugi)?)
            }
            0x0E1E_8158 => {
                Clip::TapeReference(reader.read_at_with::<TapeReferenceClip>(position, ugi)?)
            }
            0xE5B3_34C8 => Clip::Text(reader.read_at_with::<TextClip>(position, ugi)?),
            0x9B85_16EB => {
                return Err(ReadError::custom(
                    "TextAreaSize is not used in binary formats".into(),
                ))
            }
            0x36A3_12DC => {
                Clip::Translation(reader.read_at_with::<TranslationClip>(position, ugi)?)
            }
            0x101F_9D2B => {
                return Err(ReadError::custom(
                    "Vibration is not used in binary formats".into(),
                ))
            }
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
        let dancer_country_code = reader.read_len_string_at::<u32be>(position)?;
        let dancer_avatar_id = reader.read_at::<u32be>(position)?;
        let dancer_name = reader.read_len_string_at::<u32be>(position)?;

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
        let custom_param = reader.read_len_string_at::<u32be>(position)?;
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
        let custom_param = reader.read_len_string_at::<u32be>(position)?;

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
        let lyrics = reader.read_len_string_at::<u32be>(position)?;
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
                0x1 => HipStr::borrowed("X360"),
                0x3 => HipStr::borrowed("ORBIS"),
                0xA => HipStr::borrowed("DURANGO"),
                _ => {
                    return Err(ReadError::custom(format!(
                        "Unknown platform: 0x{platform:x}, position: {position}"
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
        let signature = reader.read_len_string_at::<u32be>(position)?;
        let guid = reader.read_len_string_at::<u32be>(position)?;

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
        let actor_name = reader.read_len_string_at::<u32be>(position)?;
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
        let string = reader.read_len_string_at::<u32be>(position)?;

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

struct Unknown9BC67FC4;

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

struct ActorPaths;
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
            let actor_path = reader.read_len_string_at::<u32be>(position)?;
            actor_paths.push(actor_path);
            let unk4 = reader.read_at::<u32be>(position)?;
            test_eq!(unk4, 0x0)?;
        }
        Ok(actor_paths)
    }
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
            let scene = reader.read_len_string_at::<u32be>(position)?;
            scenes.push(scene);
            let unk3 = reader.read_at::<u32be>(position)?;
            test_any!(unk3, [0x0, 0x1], "Padding is wrong at {position}")?;
        }
        let path = reader.read_len_string_at::<u32be>(position)?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_any!(unk4, [0x0, 0x1], "Padding is wrong at {position}")?;

        Ok(Self { path, scenes })
    }
}
