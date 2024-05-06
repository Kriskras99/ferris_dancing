use std::{
    fs::File,
    io::{BufWriter, Cursor, Seek, SeekFrom, Write},
};

use positioned_io::WriteAt as PWrite;

use super::{BinarySerialize, WriteAt, WriteError};
use crate::bytes::endian::Endianness;

impl BinarySerialize for u8 {
    fn serialize_at_with_ctx(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: &(),
    ) -> Result<(), WriteError> {
        writer.write_slice_at(position, &[*self])
    }
}

impl<const N: usize> BinarySerialize for [u8; N] {
    fn serialize_at_with_ctx(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: &(),
    ) -> Result<(), WriteError> {
        writer.write_slice_at(position, self.as_slice())
    }
}

impl<Endian: Endianness> BinarySerialize<Endian> for u16 {
    fn serialize_at_with_ctx(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: &Endian,
    ) -> Result<(), WriteError> {
        let mut bytes = self.to_ne_bytes();
        Endian::from_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
    }
}

// impl<Endian: Endianness> BinarySerialize<'_, Endian> for u24 {
//     fn serialize_at_with_ctx(
//         &self,
//         writer: &mut (impl WriteAt + ?Sized),
//         position: &mut u64,
//         _ctx: &Endian,
//     ) -> Result<(), WriteError> {
//         let mut bytes = self.to_ne_bytes();
//         Endian::from_native(&mut bytes);
//         writer.write_slice_at(position, &bytes)
//     }
// }

impl<Endian: Endianness> BinarySerialize<Endian> for u32 {
    fn serialize_at_with_ctx(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: &Endian,
    ) -> Result<(), WriteError> {
        let mut bytes = self.to_ne_bytes();
        Endian::from_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
    }
}

impl<Endian: Endianness> BinarySerialize<Endian> for u64 {
    fn serialize_at_with_ctx(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: &Endian,
    ) -> Result<(), WriteError> {
        let mut bytes = self.to_ne_bytes();
        Endian::from_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
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
