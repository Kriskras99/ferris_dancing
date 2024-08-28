use dotstar_toolkit_utils::{
    bytes::{
        primitives::{i16le, u16le, u32be, u32le},
        write::{BinarySerialize, WriteAt, WriteError},
    },
    test_eq,
};

use super::{AdIn, Chunk, Codec, Data, Fmt, Wav, WavPlatform};
use crate::cooked::wav::Dsp;

pub struct Writer;

impl Writer {
    pub fn create_pcm(
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        fmt: Fmt,
        samples: &[i16],
    ) -> Result<(), WriteError> {
        let data: *const u8 = samples.as_ptr().cast();
        let len = samples.len() * 2;
        let data = unsafe { std::slice::from_raw_parts(data, len) };
        Self::create(
            writer,
            position,
            Codec::PCM,
            &[Chunk::Fmt(fmt), Chunk::Data(Data { data: data.into() })],
        )
    }

    pub fn create_opus(
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        fmt: Fmt,
        adin: AdIn,
        nx_opus: &[u8],
    ) -> Result<(), WriteError> {
        Self::create(
            writer,
            position,
            Codec::Nx,
            &[
                Chunk::Fmt(fmt),
                Chunk::AdIn(adin),
                Chunk::Data(Data {
                    data: nx_opus.into(),
                }),
            ],
        )
    }

    fn create(
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        codec: Codec,
        chunks: &[Chunk],
    ) -> Result<(), WriteError> {
        let original_position = *position;
        let chunk_len = u32::try_from(chunks.len())?;
        let header_size = 32u32; // header size without the chunks
        let chunk_header_size = 12 * chunk_len; // size of the chunks without their data

        // write the header
        writer.write_at::<u32be>(position, Wav::MAGIC)?;
        writer.write_at::<u32le>(position, 0xB)?;
        writer.write_at::<WavPlatform>(position, WavPlatform::Switch)?;
        writer.write_at::<Codec>(position, codec)?;
        let total_header_size_pos = *position;
        writer.write_at::<u32le>(position, 0)?; // header size
        let data_start_pos = *position;
        writer.write_at::<u32le>(position, 0)?; // data start offset
        writer.write_at::<u32le>(position, chunk_len)?; // number of chunks
        writer.write_at::<u32le>(position, if codec == Codec::PCM { 0 } else { 3 })?; // unk2

        let mut chunk_data_start =
            original_position + u64::from(header_size) + u64::from(chunk_header_size);
        let mut relative_chunk_data_start = u32::try_from(chunk_data_start - original_position)?;
        for chunk in chunks {
            match chunk {
                Chunk::Fmt(fmt) => {
                    writer.write_at::<u32be>(position, Fmt::MAGIC)?;
                    writer.write_at::<u32le>(position, relative_chunk_data_start)?;
                    writer.write_at::<u32le>(position, Fmt::NORMAL_SIZE)?;
                    let mut chunk_data_start_copy = chunk_data_start;
                    writer.write_at::<&Fmt>(&mut chunk_data_start_copy, fmt)?;
                    chunk_data_start += u64::from(Fmt::NORMAL_SIZE);
                    test_eq!(
                        chunk_data_start,
                        chunk_data_start_copy,
                        "Wrote more than expected size"
                    )?;
                    relative_chunk_data_start += Fmt::NORMAL_SIZE;
                }
                Chunk::AdIn(adin) => {
                    writer.write_at::<u32be>(position, AdIn::MAGIC)?;
                    writer.write_at::<u32le>(position, relative_chunk_data_start)?;
                    writer.write_at::<u32le>(position, AdIn::SIZE)?;
                    let mut chunk_data_start_copy = chunk_data_start;
                    writer.write_at::<&AdIn>(&mut chunk_data_start_copy, adin)?;
                    chunk_data_start += u64::from(AdIn::SIZE);
                    test_eq!(
                        chunk_data_start,
                        chunk_data_start_copy,
                        "Wrote more than expected size"
                    )?;
                    relative_chunk_data_start += AdIn::SIZE;
                }
                Chunk::Mark(_) => unimplemented!(),
                Chunk::Strg(_) => unimplemented!(),
                Chunk::DspL(dsp) | Chunk::DspR(dsp) => {
                    writer.write_at::<u32be>(position, chunk.magic())?;
                    writer.write_at::<u32le>(position, relative_chunk_data_start)?;
                    writer.write_at::<u32le>(position, Dsp::SIZE)?;
                    let mut chunk_data_start_copy = chunk_data_start;
                    writer.write_at::<&Dsp>(&mut chunk_data_start_copy, dsp)?;
                    chunk_data_start += u64::from(Dsp::SIZE);
                    test_eq!(
                        chunk_data_start,
                        chunk_data_start_copy,
                        "Wrote more than expected size"
                    )?;
                    relative_chunk_data_start += Dsp::SIZE;
                }
                Chunk::Data(_) | Chunk::DatS(_) | Chunk::DatL(_) | Chunk::DatR(_) => {}
            }
        }

        let data_start = relative_chunk_data_start.next_multiple_of(8);
        let mut relative_data_offset = data_start;
        let mut data_offset = original_position + u64::from(relative_data_offset);
        for chunk in chunks {
            match chunk {
                Chunk::Data(data) | Chunk::DatS(data) | Chunk::DatL(data) | Chunk::DatR(data) => {
                    writer.write_at::<u32be>(position, chunk.magic())?;
                    writer.write_at::<u32le>(position, relative_data_offset)?;
                    writer.write_at::<u32le>(position, u32::try_from(data.data.len())?)?;
                    writer.write_slice_at(&mut data_offset, &data.data)?;
                    relative_data_offset += u32::try_from(data.data.len())?;
                }
                _ => {}
            }
        }

        *position = total_header_size_pos;
        writer.write_at::<u32le>(position, relative_chunk_data_start)?; // header size
        *position = data_start_pos;
        writer.write_at::<u32le>(position, data_start)?; // data start offset

        Ok(())
    }
}

impl BinarySerialize for WavPlatform {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32be>(position, u32::from(input))
    }
}

impl BinarySerialize for Codec {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32be>(position, u32::from(input))
    }
}

impl BinarySerialize for Fmt<'_> {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<&Fmt>(position, &input)
    }
}

impl BinarySerialize for &Fmt<'_> {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        fmt: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u16le>(position, fmt.unk1)?;
        writer.write_at::<u16le>(position, fmt.channel_count)?;
        writer.write_at::<u32le>(position, fmt.sample_rate)?;
        writer.write_at::<u32le>(position, fmt.unk2)?;
        writer.write_at::<u16le>(position, fmt.block_align)?;
        writer.write_at::<u16le>(position, fmt.bits_per_sample)?;
        Ok(())
    }
}

impl BinarySerialize for Dsp {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<&Self>(position, &input)
    }
}

impl BinarySerialize for &Dsp {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32le>(position, input.sample_count)?;
        writer.write_at::<u32le>(position, input.nibble_count)?;
        writer.write_at::<u32le>(position, input.sample_rate)?;
        writer.write_at::<u16le>(position, u16::from(input.loop_flag))?;
        writer.write_at::<u16le>(position, 0)?; // format
        writer.write_at::<u32le>(position, input.loop_start_offset)?;
        writer.write_at::<u32le>(position, input.loop_end_offset)?;
        writer.write_at::<u32le>(position, input.current_address)?;
        writer.write_at::<[i16le; 0x10]>(position, input.coefficients)?;
        writer.write_at::<u16le>(position, input.gain)?;
        writer.write_at::<u16le>(position, input.initial_predictor_scale)?;
        writer.write_at::<i16le>(position, input.initial_sample_history_1)?;
        writer.write_at::<i16le>(position, input.initial_sample_history_2)?;
        writer.write_at::<u16le>(position, input.loop_context_predictor_scale)?;
        writer.write_at::<i16le>(position, input.loop_context_sample_history_1)?;
        writer.write_at::<i16le>(position, input.loop_context_sample_history_2)?;
        writer.write_slice_at(position, &[0; 22])?;

        Ok(())
    }
}

impl BinarySerialize for AdIn {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<&Self>(position, &input)
    }
}

impl BinarySerialize for &AdIn {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32le>(position, input.num_of_samples)
    }
}
