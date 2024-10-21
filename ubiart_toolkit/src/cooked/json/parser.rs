use dotstar_toolkit_utils::{
    test_eq,
    testing::{TestError, TestResult},
};
use serde::Deserialize;
use ubiart_toolkit_json_types::v16::Template16;

use crate::{
    json_types::{
        v17::Template17, v18::Template18, v19::Template19, v20::Template20, v20c::Template20C,
        v21::Template21, v22::Template22,
    },
    utils::errors::ParserError,
};

/// Remove the '\0' from the end of the `buffer`
pub fn clean_buffer_json(buffer: &[u8], lax: bool) -> Result<&[u8], TestError> {
    let result = test_eq!(buffer[buffer.len() - 1], 0x0);
    match (result, lax) {
        (TestResult::Ok, _) => Ok(&buffer[..buffer.len() - 1]),
        (TestResult::Err(error), true) => {
            println!("Warning! Ignoring TestError: {error:?}");
            Ok(buffer)
        }
        (TestResult::Err(error), false) => Err(error),
    }
}

/// Parse a json-like file as T
pub fn parse<'a, T>(src: &'a [u8], lax: bool) -> Result<T, ParserError>
where
    T: Deserialize<'a>,
{
    let src = clean_buffer_json(src, lax)?;
    let src = simdutf8::basic::from_utf8(src)?;
    Ok(serde_json::from_str(src)?)
}

/// Parse a cooked json file from Just Dance 2016
pub fn parse_v16(src: &[u8], lax: bool) -> Result<Template16<'_>, ParserError> {
    let src = clean_buffer_json(src, lax)?;

    let template: Template16 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2017
pub fn parse_v17(src: &[u8], lax: bool) -> Result<Template17<'_>, ParserError> {
    let src = clean_buffer_json(src, lax)?;

    let template: Template17 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2018
pub fn parse_v18(src: &[u8], lax: bool) -> Result<Template18<'_>, ParserError> {
    let src = clean_buffer_json(src, lax)?;

    let template: Template18 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2019
pub fn parse_v19(src: &[u8], lax: bool) -> Result<Template19<'_>, ParserError> {
    let src = clean_buffer_json(src, lax)?;

    let template: Template19 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2020
pub fn parse_v20(src: &[u8], lax: bool) -> Result<Template20<'_>, ParserError> {
    let src = clean_buffer_json(src, lax)?;

    let template: Template20 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2020C
pub fn parse_v20c(src: &[u8], lax: bool) -> Result<Template20C<'_>, ParserError> {
    let src = clean_buffer_json(src, lax)?;

    let template: Template20C = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2021
pub fn parse_v21(src: &[u8], lax: bool) -> Result<Template21<'_>, ParserError> {
    let src = clean_buffer_json(src, lax)?;

    let template: Template21 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2022
pub fn parse_v22(src: &[u8], lax: bool) -> Result<Template22<'_>, ParserError> {
    let src = clean_buffer_json(src, lax)?;

    let template: Template22 = serde_json::from_slice(src)
        .map_err(ParserError::from)
        .map_err(|e| {
            e.context(
                std::str::from_utf8(src)
                    .unwrap_or("Invalid buffer!")
                    .to_string(),
            )
        })?;

    Ok(template)
}
