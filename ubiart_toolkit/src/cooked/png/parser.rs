//! Contains the parser implementation

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{u16be, u32be, u64be},
        read::{ReadAtExt, ReadError},
    },
    testing::{test_any, test_eq},
};

use super::Png;
use crate::{
    cooked::{png::Texture, xtx::Xtx},
    utils::{Platform, UniqueGameId},
};

pub fn parse(reader: &(impl ReadAtExt + ?Sized), ugi: UniqueGameId) -> Result<Png, ReadError> {
    parse_at(reader, &mut 0, ugi)
}
pub fn parse_at(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
    ugi: UniqueGameId,
) -> Result<Png, ReadError> {
    let start_position = *position;

    let magic = reader.read_at::<u64be>(position)?.into();
    test_eq(&magic, &0x9_5445_5800u64)?;

    let header_size = reader.read_at::<u32be>(position)?.into();
    test_eq(&header_size, &0x2Cu32)?;

    // For XTX textures with one mipmap, this is the data size + 0x80
    let unk2 = reader.read_at::<u32be>(position)?.into();

    let width = reader.read_at::<u16be>(position)?.into();
    let height = reader.read_at::<u16be>(position)?.into();

    let unk4 = reader.read_at::<u16be>(position)?.into();
    test_eq(&unk4, &0x0001)?;

    let unk5 = reader.read_at::<u16be>(position)?.into();
    test_any(&unk5, &[0x1800, 0x1801, 0x2000, 0x2002])?;

    let unk6 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk2, &unk6)?;

    let unk7 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk7, &0x0u32)?;

    let unk8 = reader.read_at::<u32be>(position)?.into();
    // largest values are all montage
    let unk9 = reader.read_at::<u32be>(position)?.into();

    let unk10 = reader.read_at::<u16be>(position)?.into();
    // montage is always 0x0202
    test_any(&unk10, &[0x0202, 0x0])?;

    // Always zero for just dance 2022
    let _unk11 = reader.read_at::<u16be>(position)?;

    test_eq(&(start_position + u64::from(header_size)), position)?;

    let texture = match ugi.platform {
        Platform::Nx => {
            let xtx = reader.read_at::<Xtx>(position)?;

            if xtx.images.len() > 1 {
                println!("Multiple XTX images!");
            }
            Texture::Xtx(xtx)
        }
        Platform::WiiU => Texture::Gtx(reader.read_at(position)?),
        _ => todo!(),
    };

    Ok(Png {
        width,
        height,
        unk2,
        unk5,
        unk8,
        unk9,
        unk10,
        texture,
    })
}
