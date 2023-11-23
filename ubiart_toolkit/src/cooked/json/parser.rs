use anyhow::{Context, Error};
use dotstar_toolkit_utils::testing::test;
use dotstar_toolkit_utils::testing::{TestError, TestResult};

use crate::json_types::{
    v17::Template17, v18::Template18, v19::Template19, v20::Template20, v20c::Template20C,
    v21::Template21, v22::Template22,
};

/// Remove the '\0' from the end of the `buffer`
///
/// # Errors
/// Will error when the '\0' is missing
fn clean_buffer_tpl(buffer: &[u8], lax: bool) -> Result<&[u8], TestError> {
    let result = test(&buffer[buffer.len() - 1], &0x0);
    match (result, lax) {
        (TestResult::Ok, _) => Ok(&buffer[..buffer.len() - 1]),
        (TestResult::Err(error), true) => {
            println!("Warning! Ignoring TestError: {error:?}");
            Ok(buffer)
        }
        (TestResult::Err(error), false) => Err(error),
    }
}

/// Parse a cooked json file from Just Dance 2017
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2017 or the parser encounters an unexpected value.
pub fn parse_v17(src: &[u8], lax: bool) -> Result<Template17<'_>, Error> {
    let src = clean_buffer_tpl(src, lax)?;

    let template: Template17 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2018
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2018 or the parser encounters an unexpected value.
pub fn parse_v18(src: &[u8], lax: bool) -> Result<Template18<'_>, Error> {
    let src = clean_buffer_tpl(src, lax)?;

    let template: Template18 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2019
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2019 or the parser encounters an unexpected value.
pub fn parse_v19(src: &[u8], lax: bool) -> Result<Template19<'_>, Error> {
    let src = clean_buffer_tpl(src, lax)?;

    let template: Template19 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2020
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2020 or the parser encounters an unexpected value.
pub fn parse_v20(src: &[u8], lax: bool) -> Result<Template20<'_>, Error> {
    let src = clean_buffer_tpl(src, lax)?;

    let template: Template20 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2020C
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2020 China or the parser encounters an unexpected value.
pub fn parse_v20c(src: &[u8], lax: bool) -> Result<Template20C<'_>, Error> {
    let src = clean_buffer_tpl(src, lax)?;

    let template: Template20C = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2021
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2021 or the parser encounters an unexpected value.
pub fn parse_v21(src: &[u8], lax: bool) -> Result<Template21<'_>, Error> {
    let src = clean_buffer_tpl(src, lax)?;

    let template: Template21 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2022
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2022 or the parser encounters an unexpected value.
pub fn parse_v22(src: &[u8], lax: bool) -> Result<Template22<'_>, Error> {
    let src = clean_buffer_tpl(src, lax)?;

    let template: Template22 = serde_json::from_slice(src).with_context(|| {
        std::str::from_utf8(src)
            .unwrap_or("Invalid buffer!")
            .to_string()
    })?;

    Ok(template)
}
