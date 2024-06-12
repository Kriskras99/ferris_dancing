use std::collections::{hash_map::Entry, HashMap};

use dotstar_toolkit_utils::{
    bytes::{
        endian::{Endian, BE, LE},
        primitives::u32be,
        read::{BinaryDeserialize, ReadAtExt, ReadError},
    },
    testing::{test, test_eq, TestResult},
};

use super::{
    types::{Chunk, Fmt, Wav}, AdIn, Dsp, Mark, Strg
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
        test_eq(magic, Self::MAGIC)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        let platform = reader.read_at::<WavPlatform>(position)?;
        let codec = reader.read_at::<Codec>(position)?;

        let endian = match platform {
            WavPlatform::Wii | WavPlatform::WiiU | WavPlatform::PS3 | WavPlatform::X360 => BE,
            _ => LE,
        };

        let header_size = reader.read_at_with::<u32>(position, endian)?;
        let start_offset = reader.read_at_with::<u32>(position, endian)?;
        let number_of_chunks = reader.read_at_with::<u32>(position, endian)?;
        let unk2 = reader.read_at_with::<u32>(position, endian)?;

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

        // test_eq(start + u64::from(header_size), *position)?;

        // let fmt = chunks
        //     .get(&Fmt::MAGIC)
        //     .ok_or_else(|| ReadError::custom("No `fmt ` chunk!".into()))?;
        // let fmt = fmt.as_fmt()?;

        // match (platform, codec) {
        //     (_, Codec::PCM) => todo!("PCM16LE, interleave, interleave block size 2"),
        //     (WavPlatform::Windows, Codec::Adpc) => todo!("MSADPCM"),
        //     (WavPlatform::Wii | WavPlatform::WiiU, Codec::Adpc) => todo!("NGC_DSP"),
        //     (WavPlatform::DS3, Codec::Adpc) => todo!("NGC_DSP different chunk magics"),
        //     (WavPlatform::PS3, Codec::Mp3) => todo!("Mpeg"),
        //     (WavPlatform::X360, Codec::Xma2) => todo!("xma2"),
        //     (WavPlatform::Vita, Codec::At9) => todo!("atrac9"),
        //     (WavPlatform::Switch, Codec::Nx) => todo!("Opus"),
        //     _ => panic!("Unknown platform, codec combination! {platform:?}, {codec:?}"),
        // }

        Ok(Wav {
            unk1,
            unk2,
            chunks,
            platform,
            codec,
            header_size,
            start_offset,
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

impl BinaryDeserialize<'_> for Fmt {
    type Ctx = (u64, Endian);
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let (start, endian) = ctx;

        let offset = reader.read_at_with::<u32>(position, endian)?;
        let _size = reader.read_at_with::<u32>(position, endian)?;

        let mut new_position = start + u64::from(offset);
        let unk1 = reader.read_at_with::<u16>(&mut new_position, endian)?;
        let channel_count = reader.read_at_with::<u16>(&mut new_position, endian)?;
        let sample_rate = reader.read_at_with::<u32>(&mut new_position, endian)?;
        let unk2 = reader.read_at_with::<u32>(&mut new_position, endian)?;
        let block_align = reader.read_at_with::<u16>(&mut new_position, endian)?;
        let bits_per_sample = reader.read_at_with::<u16>(&mut new_position, endian)?;

        // if let TestResult::Err(_) =
        //     test_eq(start + u64::from(offset) + u64::from(size), new_position)
        // {
        //     println!("Warning: incomplete parsing!");
        // }

        Ok(Fmt {
            unk1,
            channel_count,
            sample_rate,
            unk2,
            block_align,
            bits_per_sample,
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

        let mut new_position = start + u64::from(offset);

        let num_of_samples = reader.read_at_with::<u32>(&mut new_position, endian)?;

        test_eq(start + u64::from(offset) + u64::from(size), new_position)?;

        Ok(AdIn { num_of_samples })
    }
}

impl BinaryDeserialize<'_> for Data {
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

        Ok(Data {
            position: start + u64::from(offset),
            size,
        })
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
            println!("unk1: 0x{unk1:x}, unk2: 0x{unk2:x}, string: {string}");
            StrOrRaw::String(string)
        } else {
            println!("unk1: 0x{unk1:x}, unk2: 0x{unk2:x}");
            StrOrRaw::Raw(reader.read_slice_at(&mut new_position, usize::try_from(size - 8)?)?)
        };

        if let TestResult::Err(_) = test_eq(new_position, start + u64::from(offset + size)) {
            println!("Warning! STRG broken!");
        }

        Ok(Self {
            unk1,
            unk2,
            data,
        })
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
        let loop_context_sample_history_1 = reader.read_at_with::<i16>(&mut new_position, endian)?;
        let loop_context_sample_history_2 = reader.read_at_with::<i16>(&mut new_position, endian)?;
        let reserved = reader.read_slice_at(&mut new_position, 22)?;

        let loop_flag = match loop_flag {
            0 => false,
            1 => true,
            _ => return Err(ReadError::custom(format!("Invalid loop_flag, value is {loop_flag}"))),
        };

        test_eq(format, 0)?;
        test_eq(gain, 0)?;
        test(reserved.iter().all(|b| *b == 0))?;

        test_eq(new_position, start + u64::from(offset + size))?;

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
