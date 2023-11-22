use std::{fs, path::Path};

use anyhow::{Context, Error};
use dotstar_toolkit_utils::testing::{TestError, TestResult};
use memmap2::Mmap;
use yoke::Yoke;

use crate::{
    json_types::{
        v17::{Template17, Template17Owned},
        v18::{Template18, Template18Owned},
        v19::{Template19, Template19Owned},
        v20::{Template20, Template20Owned},
        v20c::{Template20C, Template20COwned},
        v21::{Template21, Template21Owned},
        v22::{Template22, Template22Owned},
    },
    utils::testing::test,
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

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2017
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v17<P: AsRef<Path>>(path: P, lax: bool) -> Result<Template17Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v17(data, lax))?;
    Ok(Template17Owned::from(yoke))
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

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2018
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v18<P: AsRef<Path>>(path: P, lax: bool) -> Result<Template18Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v18(data, lax))?;
    Ok(Template18Owned::from(yoke))
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

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2019
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v19<P: AsRef<Path>>(path: P, lax: bool) -> Result<Template19Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v19(data, lax))?;
    Ok(Template19Owned::from(yoke))
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

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2020
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v20<P: AsRef<Path>>(path: P, lax: bool) -> Result<Template20Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v20(data, lax))?;
    Ok(Template20Owned::from(yoke))
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

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2020 China
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v20c<P: AsRef<Path>>(path: P, lax: bool) -> Result<Template20COwned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v20c(data, lax))?;
    Ok(Template20COwned::from(yoke))
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

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2021
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v21<P: AsRef<Path>>(path: P, lax: bool) -> Result<Template21Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v21(data, lax))?;
    Ok(Template21Owned::from(yoke))
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

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2022
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v22<P: AsRef<Path>>(path: P, lax: bool) -> Result<Template22Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v22(data, lax))?;
    Ok(Template22Owned::from(yoke))
}

/// Parse a cooked json file from Just Dance 2022
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2022 or the parser encounters an unexpected value.
pub fn parse_v22(src: &[u8], lax: bool) -> Result<Template22<'_>, Error> {
    let src = clean_buffer_tpl(src, lax)?;

    let template: Template22 = serde_json::from_slice(src)
        .with_context(|| std::str::from_utf8(src).unwrap_or("Invalid buffer!").to_string())?;

    Ok(template)
}
