use std::collections::{hash_map::Entry, HashMap};

use dotstar_toolkit_utils::bytes::{
    endian::{Endian, BE, LE},
    primitives::u32be,
    read::{BinaryDeserialize, ReadAtExt, ReadError},
};
use test_eq::{test_any, test_eq};
use tracing::debug;

use super::{
    types::{Chunk, Fmt, Wav},
    AdIn, Dsp, Mark, Strg,
};
use crate::cooked::wav::{
    types::{Codec, WavPlatform},
    Data, StrOrRaw,
};

impl<'de> BinaryDeserialize<'de> for Wav<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let start = *position;
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, Self::MAGIC)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_any!(unk1, [0x0A00_0000, 0x0B00_0000, 0x0A, 0x09])?;
        let platform = reader.read_at::<WavPlatform>(position)?;
        let codec = reader.read_at::<Codec>(position)?;

        let endian = match platform {
            WavPlatform::Wii | WavPlatform::WiiU | WavPlatform::PS3 | WavPlatform::X360 => BE,
            _ => LE,
        };

        let header_size = reader.read_at_with::<u32>(position, endian)?;
        let data_start_offset = reader.read_at_with::<u32>(position, endian)?;
        let number_of_chunks = reader.read_at_with::<u32>(position, endian)?;
        let unk2 = reader.read_at_with::<u32>(position, endian)?;
        test_any!(unk2, [0, 3])?;

        let mut chunks = HashMap::with_capacity(usize::try_from(number_of_chunks)?);
        for _ in 0..number_of_chunks {
            let chunk = reader.read_at_with::<Chunk>(position, (start, endian))?;
            match chunks.entry(chunk.magic()) {
                Entry::Occupied(entry) => {
                    let key = entry.key();
                    let old = entry.get();
                    return Err(ReadError::custom(format!(
                        "Two chunks with same magic: {key:x}, {old:?}, {chunk:?}"
                    )));
                }
                Entry::Vacant(entry) => {
                    entry.insert(chunk);
                }
            }
        }

        Ok(Wav {
            unk1,
            unk2,
            platform,
            codec,
            header_size,
            data_start_offset,
            chunks,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for Chunk<'de> {
    type Ctx = (u64, Endian);
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let (start, endian) = ctx;
        let magic = reader.read_at::<u32be>(position)?;
        match magic {
            Fmt::MAGIC => Ok(Chunk::Fmt(
                reader.read_at_with::<Fmt>(position, (start, endian))?,
            )),
            AdIn::MAGIC => Ok(Chunk::AdIn(
                reader.read_at_with::<AdIn>(position, (start, endian))?,
            )),
            Data::MAGIC => Ok(Chunk::Data(
                reader.read_at_with::<Data>(position, (start, endian))?,
            )),
            Data::MAGIC_STEREO => Ok(Chunk::DatS(
                reader.read_at_with::<Data>(position, (start, endian))?,
            )),
            Data::MAGIC_LEFT => Ok(Chunk::DatL(
                reader.read_at_with::<Data>(position, (start, endian))?,
            )),
            Data::MAGIC_RIGHT => Ok(Chunk::DatR(
                reader.read_at_with::<Data>(position, (start, endian))?,
            )),
            Mark::MAGIC => Ok(Chunk::Mark(
                reader.read_at_with::<Mark>(position, (start, endian))?,
            )),
            Strg::MAGIC => Ok(Chunk::Strg(
                reader.read_at_with::<Strg>(position, (start, endian))?,
            )),
            Dsp::MAGIC_LEFT => Ok(Chunk::DspL(
                reader.read_at_with::<Dsp>(position, (start, endian))?,
            )),
            Dsp::MAGIC_RIGHT => Ok(Chunk::DspR(
                reader.read_at_with::<Dsp>(position, (start, endian))?,
            )),
            _ => Err(ReadError::custom(format!("Unkown magic!: 0x{magic:x}"))),
        }
    }
}

impl<'de> BinaryDeserialize<'de> for Fmt<'de> {
    type Ctx = (u64, Endian);
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let (start, endian) = ctx;

        let offset = reader.read_at_with::<u32>(position, endian)?;
        let size = reader.read_at_with::<u32>(position, endian)?;

        test_any!(size, [Self::NORMAL_SIZE, 18, 40, 50])?;

        let mut new_position = start + u64::from(offset);
        let unk1 = reader.read_at_with::<u16>(&mut new_position, endian)?;
        let channel_count = reader.read_at_with::<u16>(&mut new_position, endian)?;
        let sample_rate = reader.read_at_with::<u32>(&mut new_position, endian)?;
        let unk2 = reader.read_at_with::<u32>(&mut new_position, endian)?;
        let block_align = reader.read_at_with::<u16>(&mut new_position, endian)?;
        let bits_per_sample = reader.read_at_with::<u16>(&mut new_position, endian)?;

        let unk3 = match size {
            Self::NORMAL_SIZE => None,
            _ => Some(reader.read_slice_at(
                &mut new_position,
                usize::try_from(size - Self::NORMAL_SIZE)?,
            )?),
        };

        Ok(Self {
            unk1,
            channel_count,
            sample_rate,
            total_samples_hz: unk2,
            block_align,
            bits_per_sample,
            unk3,
        })
    }
}

impl BinaryDeserialize<'_> for AdIn {
    type Ctx = (u64, Endian);
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let (start, endian) = ctx;

        let offset = reader.read_at_with::<u32>(position, endian)?;
        let size = reader.read_at_with::<u32>(position, endian)?;
        test_eq!(size, Self::SIZE)?;

        let mut new_position = start + u64::from(offset);

        let num_of_samples = reader.read_at_with::<u32>(&mut new_position, endian)?;

        Ok(Self { num_of_samples })
    }
}

impl<'de> BinaryDeserialize<'de> for Data<'de> {
    type Ctx = (u64, Endian);
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let (start, endian) = ctx;

        let offset = reader.read_at_with::<u32>(position, endian)?;
        let size = reader.read_at_with::<u32>(position, endian)?;

        let mut position = start + u64::from(offset);
        let data = reader.read_slice_at(&mut position, usize::try_from(size)?)?;

        Ok(Self { data })
    }
}

impl BinaryDeserialize<'_> for Mark {
    type Ctx = (u64, Endian);
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let (start, endian) = ctx;

        let offset = reader.read_at_with::<u32>(position, endian)?;
        let size = reader.read_at_with::<u32>(position, endian)?;

        // println!("Mark:");
        // let data = reader.read_slice_at(&mut (start + u64::from(offset)), usize::try_from(size)?)?;
        // rhexdump!(data, start + u64::from(offset));

        Ok(Self {
            position: start + u64::from(offset),
            size,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for Strg<'de> {
    type Ctx = (u64, Endian);
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let (start, endian) = ctx;

        let offset = reader.read_at_with::<u32>(position, endian)?;
        let size = reader.read_at_with::<u32>(position, endian)?;

        let mut new_position = start + u64::from(offset);
        let unk1 = reader.read_at_with::<u32>(&mut new_position, endian)?;
        let unk2 = reader.read_at_with::<u32>(&mut new_position, endian)?;

        let data = if let Ok(string) = reader.read_null_terminated_string_at(&mut new_position) {
            // println!("unk1: 0x{unk1:x}, unk2: 0x{unk2:x}, string: {string}");
            StrOrRaw::String(string)
        } else {
            // println!("unk1: 0x{unk1:x}, unk2: 0x{unk2:x}");
            StrOrRaw::Raw(reader.read_slice_at(&mut new_position, usize::try_from(size - 8)?)?)
        };

        if test_eq!(new_position, start + u64::from(offset + size)).is_err() {
            debug!("STRG broken!");
        }

        Ok(Self { unk1, unk2, data })
    }
}

impl BinaryDeserialize<'_> for Dsp {
    type Ctx = (u64, Endian);
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let (start, endian) = ctx;

        let offset = reader.read_at_with::<u32>(position, endian)?;
        let size = reader.read_at_with::<u32>(position, endian)?;

        let mut new_position = start + u64::from(offset);

        let sample_count = reader.read_at_with::<u32>(&mut new_position, endian)?;
        let nibble_count = reader.read_at_with::<u32>(&mut new_position, endian)?;
        let sample_rate = reader.read_at_with::<u32>(&mut new_position, endian)?;
        let loop_flag = reader.read_at_with::<u16>(&mut new_position, endian)?;
        let format = reader.read_at_with::<u16>(&mut new_position, endian)?;
        let loop_start_offset = reader.read_at_with::<u32>(&mut new_position, endian)?;
        let loop_end_offset = reader.read_at_with::<u32>(&mut new_position, endian)?;
        let current_address = reader.read_at_with::<u32>(&mut new_position, endian)?;
        let coefficients = reader.read_at_with::<[i16; 0x10]>(&mut new_position, endian)?;
        let gain = reader.read_at_with::<u16>(&mut new_position, endian)?;
        let initial_predictor_scale = reader.read_at_with::<u16>(&mut new_position, endian)?;
        let initial_sample_history_1 = reader.read_at_with::<i16>(&mut new_position, endian)?;
        let initial_sample_history_2 = reader.read_at_with::<i16>(&mut new_position, endian)?;
        let loop_context_predictor_scale = reader.read_at_with::<u16>(&mut new_position, endian)?;
        let loop_context_sample_history_1 =
            reader.read_at_with::<i16>(&mut new_position, endian)?;
        let loop_context_sample_history_2 =
            reader.read_at_with::<i16>(&mut new_position, endian)?;
        let reserved = reader.read_slice_at(&mut new_position, 22)?;

        let loop_flag = match loop_flag {
            0 => false,
            1 => true,
            _ => {
                return Err(ReadError::custom(format!(
                    "Invalid loop_flag, value is {loop_flag}"
                )))
            }
        };

        test_eq!(format, 0)?;
        test_eq!(gain, 0)?;
        test_eq!(reserved.iter().all(|b| *b == 0), true)?;

        test_eq!(new_position, start + u64::from(offset + size))?;

        Ok(Self {
            coefficients,
            sample_count,
            nibble_count,
            sample_rate,
            loop_flag,
            loop_start_offset,
            loop_end_offset,
            current_address,
            gain,
            initial_predictor_scale,
            initial_sample_history_1,
            initial_sample_history_2,
            loop_context_predictor_scale,
            loop_context_sample_history_1,
            loop_context_sample_history_2,
        })
    }
}
