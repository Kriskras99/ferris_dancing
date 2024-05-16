use std::{
    fs::File,
    io::{BufWriter, Cursor, Seek, SeekFrom, Write},
};

use positioned_io::WriteAt as PWrite;

use super::{BinarySerialize, WriteAt, WriteError};

impl<const N: usize, T> BinarySerialize for [T; N]
where
    T: BinarySerialize,
    T::Ctx: Copy,
{
    type Input = [T::Input; N];
    type Ctx = T::Ctx;
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        ctx: T::Ctx,
    ) -> Result<(), WriteError> {
        for ty in input {
            writer.write_at_with_ctx::<T>(position, ty, ctx)?;
        }
        Ok(())
    }
}

impl WriteAt for Vec<u8> {
    fn write_slice_at(&mut self, position: &mut u64, buf: &[u8]) -> Result<(), WriteError> {
        let position_usize = usize::try_from(*position)?;
        let end = position_usize
            .checked_add(buf.len())
            .ok_or_else(WriteError::int_under_overflow)?;
        if end >= self.len() {
            self.resize(end, 0);
        }
        self[position_usize..end].copy_from_slice(buf);
        *position = u64::try_from(end)?;
        Ok(())
    }
}

// How to make this generic??
impl WriteAt for Cursor<&mut Vec<u8>> {
    fn write_slice_at(&mut self, position: &mut u64, ty: &[u8]) -> Result<(), WriteError> {
        self.seek(SeekFrom::Start(*position))?;
        self.write_all(ty)?;
        *position = self.position();
        Ok(())
    }
}

impl WriteAt for File {
    fn write_slice_at(&mut self, position: &mut u64, ty: &[u8]) -> Result<(), WriteError> {
        self.write_all_at(*position, ty)?;
        *position = position
            .checked_add(u64::try_from(ty.len())?)
            .ok_or_else(WriteError::int_under_overflow)?;
        Ok(())
    }
}

impl<T: Write + Seek> WriteAt for BufWriter<T> {
    fn write_slice_at(&mut self, position: &mut u64, ty: &[u8]) -> Result<(), WriteError> {
        self.seek(SeekFrom::Start(*position))?;
        self.write_all(ty)?;
        *position = position
            .checked_add(u64::try_from(ty.len())?)
            .ok_or_else(WriteError::int_under_overflow)?;
        Ok(())
    }
}

impl<T: WriteAt + ?Sized> WriteAt for &mut T {
    fn write_slice_at(&mut self, position: &mut u64, buf: &[u8]) -> Result<(), WriteError> {
        (*self).write_slice_at(position, buf)
    }
}
