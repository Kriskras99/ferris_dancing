use std::{borrow::Cow, io::Cursor};

use dotstar_toolkit_utils::bytes::{
    primitives::{u32be, u64be},
    write::{BinarySerialize, WriteAt, WriteError},
};

use super::{Actor, Component, MaterialGraphicComponent, PleoComponent};

impl BinarySerialize for Actor<'_> {
    fn serialize_at(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError> {
        writer.write_at(position, &u32be::from(1))?; // unk0
        writer.write_at(position, &u32be::from(self.unk1))?;
        writer.write_at(position, &u32be::from(self.unk2))?;
        writer.write_at(position, &u32be::from(self.unk2_5))?;
        writer.write_at(position, &u64be::from(0))?; // unk3
        writer.write_at(position, &u32be::from(0))?; // unk3_5
        writer.write_at(position, &u64be::from(0x1_0000_0000))?; // unk4
        writer.write_at(position, &u32be::from(0))?; // unk5
        writer.write_at(position, &u64be::from(0))?; // unk6
        writer.write_at(position, &u64be::from(0xFFFF_FFFF))?; // unk7
        writer.write_at(position, &u32be::from(0))?; // unk8
        writer.write_at(position, &self.tpl)?;
        writer.write_at(position, &u32be::from(0))?; // unk9
        writer.write_at(position, &u32be::try_from(self.components.len())?)?;
        for component in &self.components {
            writer.write_at(position, &u32be::from(component.to_id()))?;
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
                component => todo!("{component:?}"),
            }
        }
        Ok(())
    }
}

/// Create an `Actor` in a newly allocated `Vec`
pub fn create_vec(actor: &Actor<'_>) -> Result<Vec<u8>, WriteError> {
    let mut vec = Vec::with_capacity(700);
    let mut cursor = Cursor::new(&mut vec);
    actor.serialize(&mut cursor)?;
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
        writer.write_at(position, &u32be::from(0x3F80_0000))?; // unk11
    }
    writer.write_at(position, &u32be::from(mgc.unk11_5))?;
    for _ in 0..2 {
        if is_pleo {
            writer.write_at(position, &u64be::from(0xFFFF_FFFF))?; // unk12
        } else {
            writer.write_at(position, &u64be::from(0x0))?; // unk12
        }
    }
    writer.write_at(position, &u32be::from(mgc.unk13))?;
    writer.write_at(position, &u64be::from(mgc.unk14))?;
    writer.write_at(position, &u64be::from(mgc.unk15))?;
    for item in mgc.files.iter().take(9) {
        writer.write_at(position, item)?;
        writer.write_at(position, &u32be::from(0))?;
    }
    writer.write_at(position, &u32be::from(0))?;
    for item in mgc.files.iter().skip(9) {
        writer.write_at(position, item)?;
        writer.write_at(position, &u32be::from(0))?;
    }
    for _ in 0..4 {
        writer.write_at(position, &u64be::from(0))?;
    }
    writer.write_at(position, &u32be::from(0))?;
    writer.write_at(position, &u32be::from(0x3F80_0000))?;
    writer.write_at(position, &u64be::from(0xFFFF_FFFF_FFFF_FFFF))?;
    for _ in 0..3 {
        writer.write_at(position, &u32be::from(0))?;
    }
    writer.write_at(position, &u32be::from(0x3F80_0000))?;
    writer.write_at(position, &u64be::from(0))?;
    writer.write_at(position, &u32be::from(mgc.unk26))?;
    if is_pleo {
        writer.write_at(position, &u32be::from(0))?;
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
    writer.write_at(position, &u32be::from(0))?;
    writer.write_at(position, &pleo_component.dash_mpd)?;
    writer.write_at(position, &u32be::from(0))?;
    writer.write_len_string_at::<u32be>(
        position,
        pleo_component.channel_id.as_ref().map_or("", Cow::as_ref),
    )?;

    Ok(())
}
