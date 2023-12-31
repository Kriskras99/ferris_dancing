use std::{
    borrow::Cow,
    io::{Cursor, Seek, Write},
};

use byteorder::{BigEndian, WriteBytesExt};

use super::{Actor, ComponentData, MaterialGraphicComponent, PleoComponent};
use crate::utils::{bytes::WriteBytesExtUbiArt, errors::WriterError};

/// Write the `Actor` to the writer
pub fn create<W: Write + Seek>(mut writer: W, actor: &Actor) -> Result<(), WriterError> {
    writer.write_u32::<BigEndian>(1)?;
    writer.write_u32::<BigEndian>(actor.unk1)?;
    writer.write_u32::<BigEndian>(actor.unk2)?;
    writer.write_u32::<BigEndian>(actor.unk2_5)?;
    writer.write_u64::<BigEndian>(0)?;
    writer.write_u32::<BigEndian>(0)?;
    writer.write_u64::<BigEndian>(0x1_0000_0000)?;
    writer.write_u32::<BigEndian>(0)?;
    writer.write_u64::<BigEndian>(0)?;
    writer.write_u64::<BigEndian>(0xFFFF_FFFF)?;
    writer.write_u32::<BigEndian>(0)?;
    writer.write_path::<BigEndian>(&actor.tpl)?;
    writer.write_u32::<BigEndian>(0)?;
    writer.write_u32::<BigEndian>(0)?;
    writer.write_u32::<BigEndian>(u32::try_from(actor.components.len())?)?;
    for template in &actor.components {
        writer.write_u32::<BigEndian>(u32::from(template.the_type))?;
        match &template.data {
            ComponentData::None => {}
            ComponentData::MaterialGraphicComponent(mgc) => {
                write_material_graphic_component(&mut writer, mgc, false)?;
            }
            ComponentData::PleoComponent(pc) => {
                write_pleo_component(&mut writer, pc)?;
            }
            _ => todo!("{:?}", template.data),
        }
    }
    Ok(())
}

/// Create an `Actor` in a newly allocated `Vec`
pub fn create_vec(actor: &Actor) -> Result<Vec<u8>, WriterError> {
    let mut vec = Vec::with_capacity(700);
    let cursor = Cursor::new(&mut vec);
    create(cursor, actor)?;
    vec.shrink_to_fit();
    Ok(vec)
}

/// Write the `MaterialGraphicComponent` part of the actor to the writer
fn write_material_graphic_component<W: Write + Seek>(
    writer: &mut W,
    mgc: &MaterialGraphicComponent,
    is_pleo: bool,
) -> Result<(), WriterError> {
    for _ in 0..3 {
        writer.write_u32::<BigEndian>(0x3F80_0000)?;
    }
    writer.write_u32::<BigEndian>(mgc.unk11_5)?;
    for _ in 0..2 {
        if is_pleo {
            writer.write_u64::<BigEndian>(0xFFFF_FFFF)?;
        } else {
            writer.write_u64::<BigEndian>(0x0)?;
        }
    }
    writer.write_u32::<BigEndian>(mgc.unk13)?;
    writer.write_u64::<BigEndian>(mgc.unk14)?;
    writer.write_u64::<BigEndian>(mgc.unk15)?;
    for item in mgc.files.iter().take(9) {
        writer.write_path::<BigEndian>(item)?;
        writer.write_u32::<BigEndian>(0)?;
    }
    writer.write_u32::<BigEndian>(0)?;
    for item in mgc.files.iter().skip(9) {
        writer.write_path::<BigEndian>(item)?;
        writer.write_u32::<BigEndian>(0)?;
    }
    for _ in 0..4 {
        writer.write_u64::<BigEndian>(0)?;
    }
    writer.write_u32::<BigEndian>(0)?;
    writer.write_u32::<BigEndian>(0x3F80_0000)?;
    writer.write_u64::<BigEndian>(0xFFFF_FFFF_FFFF_FFFF)?;
    for _ in 0..3 {
        writer.write_u32::<BigEndian>(0)?;
    }
    writer.write_u32::<BigEndian>(0x3F80_0000)?;
    writer.write_u64::<BigEndian>(0)?;
    writer.write_u32::<BigEndian>(mgc.unk26)?;
    if is_pleo {
        writer.write_u32::<BigEndian>(0)?;
    }
    Ok(())
}

/// Write the `PleoComponent` part of the actor to the writer
fn write_pleo_component<W: Write + Seek>(
    writer: &mut W,
    pleo_component: &PleoComponent,
) -> Result<(), WriterError> {
    writer.write_path::<BigEndian>(&pleo_component.video)?;
    writer.write_u32::<BigEndian>(0)?;
    writer.write_path::<BigEndian>(&pleo_component.dash_mpd)?;
    writer.write_u32::<BigEndian>(0)?;
    writer.write_string::<BigEndian>(pleo_component.channel_id.as_ref().map_or("", Cow::as_ref))?;

    Ok(())
}
