use dotstar_toolkit_utils::bytes::read::{ReadAtExt, ReadError};
use itertools::Itertools;

/// Decoder for Nintendo GameCube ADPCM audio format
///
/// Produces [`i16`] samples. When decoding a stereo sound,
/// the two channels are interleaved per sample.
pub struct Decoder<'a, R: ReadAtExt + ?Sized> {
    left_reader: (&'a R, u64),
    right_reader: Option<(&'a R, u64)>,
    left_state: DspState,
    right_state: Option<DspState>,
    frames_remaing: u32,
    buffer: Vec<i16>,
}

impl<'a, R: ReadAtExt + ?Sized> Decoder<'a, R> {
    /// Decode mono sound
    ///
    /// `total_frames` is the total amount of frames in the channel
    pub fn mono(reader: &'a R, position: u64, state: DspState, total_frames: u32) -> Self {
        Self {
            left_reader: (reader, position),
            right_reader: None,
            left_state: state,
            right_state: None,
            frames_remaing: total_frames,
            buffer: Vec::with_capacity(14),
        }
    }

    /// Decode stereo sound where each channel has their own buffer
    ///
    /// `total_frames` is the total amount of frames in one channel
    pub fn stereo(
        left_reader: &'a R,
        left_position: u64,
        left_state: DspState,
        right_reader: &'a R,
        right_position: u64,
        right_state: DspState,
        total_frames: u32,
    ) -> Self {
        Self {
            left_reader: (left_reader, left_position),
            right_reader: Some((right_reader, right_position)),
            left_state,
            right_state: Some(right_state),
            frames_remaing: total_frames,
            buffer: Vec::with_capacity(28),
        }
    }

    /// Decode stereo sound interleaved per frame
    ///
    /// `total_frames` is the total amount of frames for both channels.
    ///
    /// # Panics
    /// Will panic if `total_frames` is not a multiple of 2
    pub fn interleaved_stereo(
        reader: &'a R,
        position: u64,
        left_state: DspState,
        right_state: DspState,
        total_frames: u32,
    ) -> Self {
        assert_eq!(
            total_frames % 2,
            0,
            "Total frames needs to be a multiple of 2"
        );
        Self {
            left_reader: (reader, position),
            right_reader: None,
            left_state,
            right_state: Some(right_state),
            frames_remaing: total_frames,
            buffer: Vec::with_capacity(28),
        }
    }
}

impl<R: ReadAtExt + ?Sized> Iterator for Decoder<'_, R> {
    type Item = Result<i16, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        let (left_reader, left_position) = &mut self.left_reader;
        if self.buffer.is_empty() && self.frames_remaing != 0 {
            match &mut (self.right_reader, self.right_state.as_mut()) {
                (None, None) => {
                    // mono
                    let frame = left_reader.read_at::<[u8; 8]>(left_position);
                    let Ok(frame) = frame else {
                        return Some(frame.map(|_| 0i16));
                    };
                    let samples = self.left_state.decode_frame(frame);
                    self.buffer.extend_from_slice(&samples);
                    self.frames_remaing -= 1;
                }
                (None, Some(right_state)) => {
                    // stereo interleaved per frame

                    // read the frame for the left channel
                    let left_frame = left_reader.read_at::<[u8; 8]>(left_position);
                    let Ok(left_frame) = left_frame else {
                        return Some(left_frame.map(|_| 0i16));
                    };
                    // decode the frame for the left channel
                    let left_samples = self.left_state.decode_frame(left_frame);

                    // read the frame for the right channel
                    let right_frame = left_reader.read_at::<[u8; 8]>(left_position);
                    let Ok(right_frame) = right_frame else {
                        return Some(right_frame.map(|_| 0i16));
                    };
                    // decode the frame for the right channel
                    let right_samples = right_state.decode_frame(right_frame);

                    // interleave the samples
                    self.buffer.extend(
                        left_samples
                            .into_iter()
                            .rev()
                            .interleave(right_samples.into_iter().rev()),
                    );
                    self.frames_remaing -= 2;
                }
                (Some((right_reader, right_position)), Some(right_state)) => {
                    // stereo not interleaved
                    let frame = left_reader.read_at::<[u8; 8]>(left_position);
                    let Ok(frame) = frame else {
                        return Some(frame.map(|_| 0i16));
                    };
                    let left_samples = self.left_state.decode_frame(frame);
                    let frame = right_reader.read_at::<[u8; 8]>(right_position);
                    let Ok(frame) = frame else {
                        return Some(frame.map(|_| 0i16));
                    };
                    let right_samples = right_state.decode_frame(frame);
                    self.buffer.extend(
                        left_samples
                            .into_iter()
                            .rev()
                            .interleave(right_samples.into_iter().rev()),
                    );
                    self.frames_remaing -= 1;
                }
                (Some(_), None) => unreachable!(),
            }
        };
        self.buffer.pop().map(Ok)
    }
}

/// State of the DSP encoder of a single channel
pub struct DspState {
    /// The initial history
    pub hist1: i16,
    /// The initial history 2
    pub hist2: i16,
    /// Coefficients for the audio
    pub coefficients: [i16; 16],
}

impl DspState {
    /// Decode a single frame of ADPCM data.
    ///
    /// Note: the frames need to be parsed sequentially as the hist1 and hist2 values
    /// are updated very frame.
    pub fn decode_frame(&mut self, frame: [u8; 8]) -> [i16; 14] {
        let header = frame[0];

        let scale = 1i32 << (header & 0xF);
        let coef_index = usize::from((header >> 4) & 0xF);
        let coef1 = i32::from(self.coefficients[coef_index * 2]);
        let coef2 = i32::from(self.coefficients[coef_index * 2 + 1]);

        let mut out = [0; 14];
        let mut i = 0;

        // 7 data bytes per frame
        for byte in &frame[1..] {
            let byte = *byte;
            // 2 samples per byte
            for s in 0..2 {
                let mut sample = if s == 0 {
                    get_high_nibble(byte)
                } else {
                    get_low_nibble(byte)
                };
                sample = if sample >= 8 { sample - 16 } else { sample };
                sample = (((scale * sample) << 11)
                    + 1024
                    + (coef1 * i32::from(self.hist1) + coef2 * i32::from(self.hist2)))
                    >> 11;
                let sample = clamp(sample);

                out[i] = sample;
                i += 1;

                self.hist2 = self.hist1;
                self.hist1 = sample;
            }
        }
        out
    }
}

/// The amount of samples in a single frame
pub const SAMPLES_PER_FRAME: u32 = 14;
const NIBBLE_TO_S8: [i32; 0x10] = [0, 1, 2, 3, 4, 5, 6, 7, -8, -7, -6, -5, -4, -3, -2, -1];
fn get_low_nibble(byte: u8) -> i32 {
    NIBBLE_TO_S8[usize::from(byte & 0xF)]
}
fn get_high_nibble(byte: u8) -> i32 {
    NIBBLE_TO_S8[usize::from((byte >> 4) & 0xF)]
}

#[allow(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "It's clamped to i16 and therefore safe."
)]
fn clamp(val: i32) -> i16 {
    val.clamp(-32768, 32767) as i16
}
