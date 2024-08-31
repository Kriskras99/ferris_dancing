#![allow(
    non_camel_case_types,
    reason = "The type aliases need to look like primary types"
)]

use super::{
    endian::{self, Endian, BE, LE},
    len::Len,
    read::{BinaryDeserialize, ReadAtExt, ReadError},
    write::{BinarySerialize, WriteAt, WriteError},
};

impl BinaryDeserialize<'_> for u8 {
    type Ctx = ();
    type Output = Self;

    #[inline]
    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        Ok(reader.read_slice_at(position, 1)?[0])
    }
}

impl BinarySerialize for u8 {
    type Ctx = ();
    type Input = Self;

    #[inline]
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_slice_at(position, &[input])
    }
}

impl Len<'_> for u8 {}

pub enum u16be {}
impl BinaryDeserialize<'_> for u16be {
    type Ctx = ();
    type Output = u16;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<u16>(position, BE)
    }
}
impl BinarySerialize for u16be {
    type Ctx = ();
    type Input = u16;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<u16>(position, input, BE)
    }
}
impl Len<'_> for u16be {}
pub enum u16le {}
impl BinaryDeserialize<'_> for u16le {
    type Ctx = ();
    type Output = u16;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<u16>(position, LE)
    }
}
impl BinarySerialize for u16le {
    type Ctx = ();
    type Input = u16;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<u16>(position, input, LE)
    }
}
impl Len<'_> for u16le {}
impl BinaryDeserialize<'_> for u16 {
    type Ctx = Endian;
    type Output = Self;

    #[inline]
    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut bytes = reader.read_at::<[u8; 2]>(position)?;
        ctx.to_native(&mut bytes);
        Ok(Self::from_ne_bytes(bytes))
    }
}

impl BinarySerialize for u16 {
    type Ctx = Endian;
    type Input = Self;

    #[inline]
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        let mut bytes = input.to_ne_bytes();
        ctx.from_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
    }
}
pub enum i16be {}
impl BinaryDeserialize<'_> for i16be {
    type Ctx = ();
    type Output = i16;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<i16>(position, BE)
    }
}
impl BinarySerialize for i16be {
    type Ctx = ();
    type Input = i16;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<i16>(position, input, BE)
    }
}
impl Len<'_> for i16be {}
pub enum i16le {}
impl BinaryDeserialize<'_> for i16le {
    type Ctx = ();
    type Output = i16;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<i16>(position, LE)
    }
}
impl BinarySerialize for i16le {
    type Ctx = ();
    type Input = i16;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<i16>(position, input, LE)
    }
}
impl Len<'_> for i16le {}
impl BinaryDeserialize<'_> for i16 {
    type Ctx = Endian;
    type Output = Self;

    #[inline]
    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut bytes = reader.read_at::<[u8; 2]>(position)?;
        ctx.to_native(&mut bytes);
        Ok(Self::from_ne_bytes(bytes))
    }
}

impl BinarySerialize for i16 {
    type Ctx = Endian;
    type Input = Self;

    #[inline]
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        let mut bytes = input.to_ne_bytes();
        ctx.from_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
    }
}

pub enum u24be {}
impl BinaryDeserialize<'_> for u24be {
    type Ctx = ();
    type Output = u32;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut bytes = reader.read_at::<[u8; 3]>(position)?;
        BE.to_native(&mut bytes);
        let bytes = endian::pad(bytes);
        Ok(Self::Output::from_ne_bytes(bytes))
    }
}
impl BinarySerialize for u24be {
    type Ctx = ();
    type Input = u32;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        let mut bytes = input.to_ne_bytes();
        BE.from_native(&mut bytes);
        let bytes: [u8; 3] = endian::unpad(bytes);
        writer.write_slice_at(position, &bytes)
    }
}
impl Len<'_> for u24be {}
pub enum u24le {}
impl BinaryDeserialize<'_> for u24le {
    type Ctx = ();
    type Output = u32;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut bytes = reader.read_at::<[u8; 3]>(position)?;
        LE.to_native(&mut bytes);
        let bytes = endian::pad(bytes);
        Ok(Self::Output::from_ne_bytes(bytes))
    }
}
impl BinarySerialize for u24le {
    type Ctx = ();
    type Input = u32;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        let mut bytes = input.to_ne_bytes();
        LE.from_native(&mut bytes);
        let bytes: [u8; 3] = endian::unpad(bytes);
        writer.write_slice_at(position, &bytes)
    }
}
impl Len<'_> for u24le {}

pub enum u32be {}
impl BinaryDeserialize<'_> for u32be {
    type Ctx = ();
    type Output = u32;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<u32>(position, BE)
    }
}
impl BinarySerialize for u32be {
    type Ctx = ();
    type Input = u32;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<u32>(position, input, BE)
    }
}
impl Len<'_> for u32be {}
pub enum u32le {}
impl BinaryDeserialize<'_> for u32le {
    type Ctx = ();
    type Output = u32;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<u32>(position, LE)
    }
}
impl BinarySerialize for u32le {
    type Ctx = ();
    type Input = u32;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<u32>(position, input, LE)
    }
}
impl Len<'_> for u32le {}
impl BinaryDeserialize<'_> for u32 {
    type Ctx = Endian;
    type Output = Self;

    #[inline]
    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut bytes = reader.read_at::<[u8; 4]>(position)?;
        ctx.to_native(&mut bytes);
        Ok(Self::from_ne_bytes(bytes))
    }
}

impl BinarySerialize for u32 {
    type Ctx = Endian;
    type Input = Self;

    #[inline]
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        let mut bytes = input.to_ne_bytes();
        ctx.from_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
    }
}
pub enum i32be {}
impl BinaryDeserialize<'_> for i32be {
    type Ctx = ();
    type Output = i32;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<i32>(position, BE)
    }
}
impl BinarySerialize for i32be {
    type Ctx = ();
    type Input = i32;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<i32>(position, input, BE)
    }
}
impl Len<'_> for i32be {}
pub enum i32le {}
impl BinaryDeserialize<'_> for i32le {
    type Ctx = ();
    type Output = i32;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<i32>(position, LE)
    }
}
impl BinarySerialize for i32le {
    type Ctx = ();
    type Input = i32;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<i32>(position, input, LE)
    }
}
impl Len<'_> for i32le {}
impl BinaryDeserialize<'_> for i32 {
    type Ctx = Endian;
    type Output = Self;

    #[inline]
    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut bytes = reader.read_at::<[u8; 4]>(position)?;
        ctx.to_native(&mut bytes);
        Ok(Self::from_ne_bytes(bytes))
    }
}

impl BinarySerialize for i32 {
    type Ctx = Endian;
    type Input = Self;

    #[inline]
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        let mut bytes = input.to_ne_bytes();
        ctx.from_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
    }
}

pub enum u64be {}
impl BinaryDeserialize<'_> for u64be {
    type Ctx = ();
    type Output = u64;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<u64>(position, BE)
    }
}
impl BinarySerialize for u64be {
    type Ctx = ();
    type Input = u64;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<u64>(position, input, BE)
    }
}
impl Len<'_> for u64be {}
pub enum u64le {}
impl BinaryDeserialize<'_> for u64le {
    type Ctx = ();
    type Output = u64;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<u64>(position, LE)
    }
}
impl BinarySerialize for u64le {
    type Ctx = ();
    type Input = u64;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<u64>(position, input, LE)
    }
}
impl Len<'_> for u64le {}
impl BinaryDeserialize<'_> for u64 {
    type Ctx = Endian;
    type Output = Self;

    #[inline]
    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut bytes = reader.read_at::<[u8; 8]>(position)?;
        ctx.to_native(&mut bytes);
        Ok(Self::from_ne_bytes(bytes))
    }
}

impl BinarySerialize for u64 {
    type Ctx = Endian;
    type Input = Self;

    #[inline]
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        let mut bytes = input.to_ne_bytes();
        ctx.from_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
    }
}

pub enum f32be {}
impl BinaryDeserialize<'_> for f32be {
    type Ctx = ();
    type Output = f32;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<f32>(position, BE)
    }
}
impl BinarySerialize for f32be {
    type Ctx = ();
    type Input = f32;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<f32>(position, input, BE)
    }
}
pub enum f32le {}
impl BinaryDeserialize<'_> for f32le {
    type Ctx = ();
    type Output = f32;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        reader.read_at_with::<f32>(position, LE)
    }
}
impl BinarySerialize for f32le {
    type Ctx = ();
    type Input = f32;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx::<f32>(position, input, LE)
    }
}
impl BinaryDeserialize<'_> for f32 {
    type Ctx = Endian;
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut bytes = reader.read_at::<[u8; 4]>(position)?;
        ctx.to_native(&mut bytes);
        Ok(Self::from_ne_bytes(bytes))
    }
}

impl BinarySerialize for f32 {
    type Ctx = Endian;
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        let mut bytes = input.to_ne_bytes();
        ctx.to_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
    }
}
