use dotstar_toolkit_utils::bytes::{
    primitives::{u16be, u32be, u64be},
    write::{BinarySerialize, WriteAt, WriteError},
};

use super::Png;
use crate::cooked::xtx::Xtx;

impl BinarySerialize for Png {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        png: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u64be>(position, 0x9_5445_5800)?;
        writer.write_at::<u32be>(position, 0x2C)?;
        writer.write_at::<u32be>(position, png.unk2)?;
        writer.write_at::<u16be>(position, png.width)?;
        writer.write_at::<u16be>(position, png.height)?;
        writer.write_at::<u16be>(position, 0x1)?;
        writer.write_at::<u16be>(position, png.unk5)?;
        writer.write_at::<u32be>(position, png.unk2)?;
        writer.write_at::<u32be>(position, 0x0)?;
        writer.write_at::<u32be>(position, png.unk8)?;
        writer.write_at::<u32be>(position, png.unk9)?;
        writer.write_at::<u16be>(position, png.unk10)?;
        writer.write_at::<u16be>(position, 0x0)?;
        writer.write_at::<Xtx>(position, png.texture.xtx()?)?;
        Ok(())
    }
}

/// Create the cooked PNG file in a newly allocated `Vec`
pub fn create_vec(png: Png) -> Result<Vec<u8>, WriteError> {
    let mut vec = Vec::new();
    vec.write_at::<Png>(&mut 0, png)?;
    vec.shrink_to_fit();
    Ok(vec)
}
