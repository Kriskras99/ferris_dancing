use dotstar_toolkit_utils::bytes::{
    primitives::u32be,
    write::{BinarySerialize, WriteAt, WriteError},
};

use crate::loc8::types::Loc8;

/// Creates a .loc8 file in a newly allocated `Vec`
pub fn create_vec(loc8: Loc8) -> Result<Vec<u8>, WriteError> {
    let mut vec = Vec::with_capacity(2_000_000);
    vec.write_at::<Loc8>(&mut 0, loc8)?;
    vec.shrink_to_fit();
    Ok(vec)
}

impl BinarySerialize for Loc8<'_> {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        loc8: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        let language = loc8.language;
        let strings = loc8.strings;

        writer.write_at::<u32be>(position, 1)?;
        writer.write_at::<u32be>(position, u32::from(language))?;
        let size = u32::try_from(strings.len())?;
        writer.write_at::<u32be>(position, size)?;

        for (locale_id, string) in strings {
            writer.write_at::<u32be>(position, u32::from(locale_id))?;
            writer.write_len_string_at::<u32be>(position, &string)?;
        }

        writer.write_at::<u32be>(position, 0)?; // unk2
        writer.write_slice_at(position, &Loc8::FOOTERS[0])?;
        Ok(())
    }
}
