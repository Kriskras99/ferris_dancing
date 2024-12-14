use std::borrow::Cow;

use serde::Deserialize;
use test_eq::{test_eq, TestFailure};
use tracing::trace;
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
