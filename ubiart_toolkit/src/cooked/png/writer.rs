use std::io::{Cursor, Write};

use byteorder::{BigEndian, WriteBytesExt};

use super::Png;
use crate::{cooked::xtx, utils::errors::WriterError};

/// Create the cooked PNG file in a newly allocated `Vec`
pub fn create<W: Write>(mut src: W, png: &Png) -> Result<(), WriterError> {
    src.write_u64::<BigEndian>(0x9_5445_5800)?;
    src.write_u32::<BigEndian>(0x2C)?;
    src.write_u32::<BigEndian>(png.unk2)?;
    src.write_u16::<BigEndian>(png.width)?;
    src.write_u16::<BigEndian>(png.height)?;
    src.write_u16::<BigEndian>(0x1)?;
    src.write_u16::<BigEndian>(png.unk5)?;
    src.write_u32::<BigEndian>(png.unk2)?;
    src.write_u32::<BigEndian>(0x0)?;
    src.write_u32::<BigEndian>(png.unk8)?;
    src.write_u32::<BigEndian>(png.unk9)?;
    src.write_u16::<BigEndian>(png.unk10)?;
    src.write_u16::<BigEndian>(0x0)?;
    xtx::create(src, &png.xtx)?;
    Ok(())
}

/// Create the cooked PNG file in a newly allocated `Vec`
pub fn create_vec(png: &Png) -> Result<Vec<u8>, WriterError> {
    // Calculate required capacity: png header + xtx header/footer + per image(header size + data size)
    // We can't use a static capacity as image sizes range from 66KB to 2MB
    let capacity = 44
        + 76
        + png
            .xtx
            .images
            .iter()
            .map(|i| 452 + i.data.iter().map(Vec::len).sum::<usize>())
            .sum::<usize>();
    let mut vec = Vec::with_capacity(capacity);
    let cursor = Cursor::new(&mut vec);
    create(cursor, png)?;
    // Just in case our calculation is off
    vec.shrink_to_fit();
    Ok(vec)
}
