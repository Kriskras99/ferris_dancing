use dotstar_toolkit_utils::bytes::{
    primitives::{f32be, i32be, u32be, u64be},
    write::{BinarySerialize, BinarySerializeExt, WriteAt, WriteError},
};

use super::{Actor, Component, MaterialGraphicComponent, PleoComponent};
use crate::utils::SplitPath;

impl BinarySerialize for Actor<'_> {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<(), WriteError> {
        writer.write_at::<u32be>(position, 1)?; // unk0
        writer.write_at::<f32be>(position, input.unk1)?;
        writer.write_at::<f32be>(position, input.unk2)?;
        writer.write_at::<f32be>(position, input.unk2_5)?;
        writer.write_at::<u64be>(position, 0)?; // unk3
        writer.write_at::<u32be>(position, 0)?; // unk3_5
        writer.write_at::<u64be>(position, 0x1_0000_0000)?; // unk4
        writer.write_at::<u32be>(position, 0)?; // unk5
        writer.write_at::<u64be>(position, 0)?; // unk6
        writer.write_at::<u64be>(position, 0xFFFF_FFFF)?; // unk7
        writer.write_at::<u32be>(position, 0)?; // unk8
        writer.write_at::<SplitPath>(position, input.lua)?;
        writer.write_at::<u32be>(position, 0)?; // unk9
        writer.write_at::<u32be>(position, u32::try_from(input.components.len())?)?;
        for component in input.components {
            writer.write_at::<u32be>(position, component.to_id())?;
            match component {
                Component::AutodanceComponent
                | Component::MasterTape
                | Component::PictoComponent
                | Component::SongDatabaseComponent
                | Component::SongDescComponent
                | Component::TapeCaseComponent
                | Component::AvatarDescComponent
                | Component::SkinDescComponent => {}
                Component::MaterialGraphicComponent(mgc) => {
                    writer.write_at_with_ctx::<MaterialGraphicComponent>(position, mgc, false)?;
                }
                Component::PleoComponent(pc) => writer.write_at::<PleoComponent>(position, pc)?,
                Component::PleoTextureGraphicComponent(mgc) => {
                    writer.write_at_with_ctx::<MaterialGraphicComponent>(position, mgc, true)?;
                }
                component => return Err(WriteError::custom(format!("TODO: {component:?}"))),
            }
        }
        Ok(())
    }
}

/// Create an `Actor` in a newly allocated `Vec`
pub fn create_vec(actor: Actor<'_>) -> Result<Vec<u8>, WriteError> {
    let mut vec = Vec::with_capacity(700);
    Actor::serialize(actor, &mut vec)?;
    vec.shrink_to_fit();
    Ok(vec)
}

impl BinarySerialize for MaterialGraphicComponent<'_> {
    type Ctx = bool;
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        is_pleo: Self::Ctx,
    ) -> Result<(), WriteError> {
        for _ in 0..3 {
            writer.write_at::<u32be>(position, 0x3F80_0000)?; // unk11
        }
        writer.write_at::<f32be>(position, input.unk4)?;
        for _ in 0..2 {
            if is_pleo {
                writer.write_at::<u64be>(position, 0xFFFF_FFFF)?; // unk12
            } else {
                writer.write_at::<u64be>(position, 0x0)?; // unk12
            }
        }
        writer.write_at::<u32be>(position, input.unk9)?;
        writer.write_at::<u32be>(position, 0)?;
        writer.write_at::<i32be>(position, input.anchor)?;
        writer.write_at::<f32be>(position, input.unk11)?;
        writer.write_at::<f32be>(position, input.unk12)?;

        for (index, item) in input.files.into_iter().enumerate() {
            if index == 9 {
                writer.write_at::<u32be>(position, 0)?;
            }
            writer.write_at::<SplitPath>(position, item)?;
        }

        for _ in 0..4 {
            writer.write_at::<u64be>(position, 0)?;
        }
        writer.write_at::<u32be>(position, 0)?;
        writer.write_at::<u32be>(position, 0x3F80_0000)?;
        writer.write_at::<u64be>(position, 0xFFFF_FFFF_FFFF_FFFF)?;
        for _ in 0..3 {
            writer.write_at::<u32be>(position, 0)?;
        }
        writer.write_at::<u32be>(position, 0x3F80_0000)?;
        writer.write_at::<u64be>(position, 0)?;
        writer.write_at::<i32be>(position, input.old_anchor)?;
        if is_pleo {
            writer.write_at::<u32be>(position, 0)?;
        }
        Ok(())
    }
}

impl BinarySerialize for PleoComponent<'_> {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<SplitPath>(position, input.video)?;
        writer.write_at::<SplitPath>(position, input.dash_mpd)?;
        writer.write_len_string_at::<u32be>(position, input.channel_id.as_str())?;

        Ok(())
    }
}
