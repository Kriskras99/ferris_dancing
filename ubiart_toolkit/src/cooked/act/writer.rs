use std::borrow::Cow;

use dotstar_toolkit_utils::bytes::{
    endian::BigEndian,
    write::{BinarySerialize, BinarySerializeExt as _, WriteAt, WriteError},
};

use super::{Actor, Component, MaterialGraphicComponent, PleoComponent};

impl BinarySerialize for Actor<'_> {
    fn serialize_at_with_ctx(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: &(),
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx(position, &1u32, &BigEndian)?; // unk0
        writer.write_at_with_ctx(position, &self.unk1, &BigEndian)?;
        writer.write_at_with_ctx(position, &self.unk2, &BigEndian)?;
        writer.write_at_with_ctx(position, &self.unk2_5, &BigEndian)?;
        writer.write_at_with_ctx(position, &0u64, &BigEndian)?; // unk3
        writer.write_at_with_ctx(position, &0u32, &BigEndian)?; // unk3_5
        writer.write_at_with_ctx(position, &0x1_0000_0000u64, &BigEndian)?; // unk4
        writer.write_at_with_ctx(position, &0u32, &BigEndian)?; // unk5
        writer.write_at_with_ctx(position, &0u64, &BigEndian)?; // unk6
        writer.write_at_with_ctx(position, &0xFFFF_FFFFu64, &BigEndian)?; // unk7
        writer.write_at_with_ctx(position, &0u32, &BigEndian)?; // unk8
        writer.write_at(position, &self.tpl)?;
        writer.write_at_with_ctx(position, &0u32, &BigEndian)?; // unk9
        writer.write_at_with_ctx(position, &u32::try_from(self.components.len())?, &BigEndian)?;
        for component in &self.components {
            writer.write_at_with_ctx(position, &component.to_id(), &BigEndian)?;
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
                    write_material_graphic_component(writer, position, mgc, false)?;
                }
                Component::PleoComponent(pc) => write_pleo_component(writer, position, pc)?,
                Component::PleoTextureGraphicComponent(mgc) => {
                    write_material_graphic_component(writer, position, mgc, true)?;
                }
                component => return Err(WriteError::custom(format!("TODO: {component:?}"))),
            }
        }
        Ok(())
    }
}

/// Create an `Actor` in a newly allocated `Vec`
pub fn create_vec(actor: &Actor<'_>) -> Result<Vec<u8>, WriteError> {
    let mut vec = Vec::with_capacity(700);
    actor.serialize(&mut vec)?;
    vec.shrink_to_fit();
    Ok(vec)
}

/// Write the `MaterialGraphicComponent` part of the actor to the writer
fn write_material_graphic_component(
    writer: &mut (impl WriteAt + ?Sized),
    position: &mut u64,
    mgc: &MaterialGraphicComponent,
    is_pleo: bool,
) -> Result<(), WriteError> {
    for _ in 0..3 {
        writer.write_at_with_ctx(position, &0x3F80_0000u32, &BigEndian)?; // unk11
    }
    writer.write_at_with_ctx(position, &mgc.unk11_5, &BigEndian)?;
    for _ in 0..2 {
        if is_pleo {
            writer.write_at_with_ctx(position, &0xFFFF_FFFFu64, &BigEndian)?; // unk12
        } else {
            writer.write_at_with_ctx(position, &0x0u64, &BigEndian)?; // unk12
        }
    }
    writer.write_at_with_ctx(position, &mgc.unk13, &BigEndian)?;
    writer.write_at_with_ctx(position, &mgc.unk14, &BigEndian)?;
    writer.write_at_with_ctx(position, &mgc.unk15, &BigEndian)?;
    for item in mgc.files.iter().take(9) {
        writer.write_at(position, item)?;
    }
    writer.write_at_with_ctx(position, &0u32, &BigEndian)?;
    for item in mgc.files.iter().skip(9) {
        writer.write_at(position, item)?;
    }
    for _ in 0..4 {
        writer.write_at_with_ctx(position, &0u64, &BigEndian)?;
    }
    writer.write_at_with_ctx(position, &0u32, &BigEndian)?;
    writer.write_at_with_ctx(position, &0x3F80_0000u32, &BigEndian)?;
    writer.write_at_with_ctx(position, &0xFFFF_FFFF_FFFF_FFFFu64, &BigEndian)?;
    for _ in 0..3 {
        writer.write_at_with_ctx(position, &0u32, &BigEndian)?;
    }
    writer.write_at_with_ctx(position, &0x3F80_0000u32, &BigEndian)?;
    writer.write_at_with_ctx(position, &0u64, &BigEndian)?;
    writer.write_at_with_ctx(position, &mgc.unk26, &BigEndian)?;
    if is_pleo {
        writer.write_at_with_ctx(position, &0u32, &BigEndian)?;
    }
    Ok(())
}

/// Write the `PleoComponent` part of the actor to the writer
fn write_pleo_component(
    writer: &mut (impl WriteAt + ?Sized),
    position: &mut u64,
    pleo_component: &PleoComponent,
) -> Result<(), WriteError> {
    writer.write_at(position, &pleo_component.video)?;
    writer.write_at(position, &pleo_component.dash_mpd)?;
    writer.write_len_string_at::<u32be>(
        position,
        pleo_component.channel_id.as_ref().map_or("", Cow::as_ref),
    )?;

    Ok(())
}
