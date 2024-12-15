use std::borrow::Cow;
use serde::Deserialize;
use test_eq::{test_eq, TestFailure};
use tracing::{error, trace};
use ubiart_toolkit_shared_types::errors::ParserError;

/// Remove the '\0' from the end of the `buffer`
pub fn clean_buffer_json(buffer: &[u8], lax: bool) -> Result<&[u8], TestFailure> {
    let result = test_eq!(buffer[buffer.len() - 1], 0x0);
    match (result, lax) {
        (Ok(()), _) => Ok(&buffer[..buffer.len() - 1]),
        (Err(error), true) => {
            trace!("Warning! Ignoring TestError: {error:?}");
            Ok(buffer)
        }
        (Err(error), false) => Err(error),
    }
}

// #[cfg(not(test))]
// /// Parse a json-like file as T
// pub fn parse<'a, T>(src: &'a [u8], lax: bool) -> Result<T, ParserError>
// where
//     T: Deserialize<'a>,
// {
//     let src = clean_buffer_json(src, lax)?;
//     let src = simdutf8::basic::from_utf8(src)?;
//     Ok(serde_json::from_str(src)?)
// }

// #[cfg(test)]
/// Parse a json-like file as T
pub fn parse<'a, T>(src: &'a [u8], lax: bool) -> Result<T, ParserError>
where
    T: Deserialize<'a>,
{
    let src = clean_buffer_json(src, lax)?;
    let src = String::from_utf8_lossy(src);
    match src {
        Cow::Borrowed(src) => Ok(serde_json::from_str(src)?),
        Cow::Owned(src) => {
            let src: &'static mut str = src.leak();
            Ok(serde_json::from_str(src)?)
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum FloatOrU32 {
    F32(f32),
    U32(u32),
}

pub fn deserialize_vec_f32_or_u32<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, SeqAccess, Visitor};
    struct Visit;
    impl<'de> Visitor<'de> for Visit {
        type Value = Vec<u32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            error!("Called expecting vec");
            formatter.write_str("a sequence of floats or integers between 0 and 2^32-1")
        }

        #[allow(clippy::as_conversions, clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "Only way and this is checked")]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut result = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(value) = seq.next_element::<FloatOrU32>()? {
                result.push(match value {
                    FloatOrU32::F32(v) => {
                        let v = v.round();
                        if (0.0..=4_294_967_000.0).contains(&v) {
                            v as u32
                        } else {
                            return Err(Error::custom(format!("float value {v} is out of range")));
                        }
                    }
                    FloatOrU32::U32(v) => v,
                });
            }
            Ok(result)
        }
    }

    deserializer.deserialize_seq(Visit)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum FloatOrI32 {
    F32(f32),
    I32(i32),
}

#[allow(clippy::as_conversions, clippy::cast_possible_truncation, reason = "Only way and this is checked")]
pub fn deserialize_f32_or_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let m = FloatOrI32::deserialize(deserializer)?;
    match m {
        FloatOrI32::F32(v) => {
            let v = v.round();
            if (-2_147_483_600.0..=2_147_483_600.0).contains(&v) {
                Ok(v as i32)
            } else {
                Err(Error::custom(format!("float value {v} is out of range")))
            }
        }
        FloatOrI32::I32(v) => Ok(v),
    }
}
