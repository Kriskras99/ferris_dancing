use std::{fs, path::Path};

use anyhow::Error;
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
        Template, TemplateOwned,
    },
    utils::{testing::test, Game},
};

/// Open the file as a template from the game `Game`
///
/// # Errors
/// Will error if parsing fails
pub fn open<P: AsRef<Path>>(path: P, game: Game) -> Result<TemplateOwned<Mmap>, Error> {
    match game {
        Game::JustDance2017 => Ok(TemplateOwned::V17(open_v17(path)?)),
        Game::JustDance2018 => Ok(TemplateOwned::V18(open_v18(path)?)),
        Game::JustDance2019 => Ok(TemplateOwned::V19(open_v19(path)?)),
        Game::JustDance2020 => Ok(TemplateOwned::V20(open_v20(path)?)),
        Game::JustDanceChina => Ok(TemplateOwned::V20C(open_v20c(path)?)),
        Game::JustDance2021 => Ok(TemplateOwned::V21(open_v21(path)?)),
        Game::JustDance2022 => Ok(TemplateOwned::V22(open_v22(path)?)),
        _ => unimplemented!("Game version needs to be between 2017-2022 for now"),
    }
}

/// Parse the `src` as a template from the game `Game`
///
/// # Errors
/// Will error if parsing fails
pub fn parse(src: &[u8], game: Game) -> Result<Template<'_>, Error> {
    match game {
        Game::JustDance2017 => Ok(Template::V17(parse_v17(src)?)),
        Game::JustDance2018 => Ok(Template::V18(parse_v18(src)?)),
        Game::JustDance2019 => Ok(Template::V19(parse_v19(src)?)),
        Game::JustDance2020 => Ok(Template::V20(parse_v20(src)?)),
        Game::JustDanceChina => Ok(Template::V20C(parse_v20c(src)?)),
        Game::JustDance2021 => Ok(Template::V21(parse_v21(src)?)),
        Game::JustDance2022 => Ok(Template::V22(parse_v22(src)?)),
        _ => unimplemented!("Game version needs to be between 2017-2022 for now"),
    }
}

/// Remove the '\0' from the end of the `buffer`
///
/// # Errors
/// Will error when the '\0' is missing
fn clean_buffer_tpl(buffer: &[u8]) -> Result<&[u8], Error> {
    test(&buffer[buffer.len() - 1], &0x0)?;
    Ok(&buffer[..buffer.len() - 1])
}

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2017
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v17<P: AsRef<Path>>(path: P) -> Result<Template17Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v17(data))?;
    Ok(Template17Owned::from(yoke))
}

/// Parse a cooked json file from Just Dance 2017
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2017 or the parser encounters an unexpected value.
pub fn parse_v17(src: &[u8]) -> Result<Template17<'_>, Error> {
    let src = clean_buffer_tpl(src)?;

    let template: Template17 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2018
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v18<P: AsRef<Path>>(path: P) -> Result<Template18Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v18(data))?;
    Ok(Template18Owned::from(yoke))
}

/// Parse a cooked json file from Just Dance 2018
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2018 or the parser encounters an unexpected value.
pub fn parse_v18(src: &[u8]) -> Result<Template18<'_>, Error> {
    let src = clean_buffer_tpl(src)?;

    let template: Template18 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2019
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v19<P: AsRef<Path>>(path: P) -> Result<Template19Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v19(data))?;
    Ok(Template19Owned::from(yoke))
}

/// Parse a cooked json file from Just Dance 2019
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2019 or the parser encounters an unexpected value.
pub fn parse_v19(src: &[u8]) -> Result<Template19<'_>, Error> {
    let src = clean_buffer_tpl(src)?;

    let template: Template19 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2020
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v20<P: AsRef<Path>>(path: P) -> Result<Template20Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v20(data))?;
    Ok(Template20Owned::from(yoke))
}

/// Parse a cooked json file from Just Dance 2020
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2020 or the parser encounters an unexpected value.
pub fn parse_v20(src: &[u8]) -> Result<Template20<'_>, Error> {
    let src = clean_buffer_tpl(src)?;

    let template: Template20 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2020 China
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v20c<P: AsRef<Path>>(path: P) -> Result<Template20COwned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v20c(data))?;
    Ok(Template20COwned::from(yoke))
}

/// Parse a cooked json file from Just Dance 2020C
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2020 China or the parser encounters an unexpected value.
pub fn parse_v20c(src: &[u8]) -> Result<Template20C<'_>, Error> {
    let src = clean_buffer_tpl(src)?;

    let template: Template20C = serde_json::from_slice(src)?;

    Ok(template)
}

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2021
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v21<P: AsRef<Path>>(path: P) -> Result<Template21Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v21(data))?;
    Ok(Template21Owned::from(yoke))
}

/// Parse a cooked json file from Just Dance 2021
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2021 or the parser encounters an unexpected value.
pub fn parse_v21(src: &[u8]) -> Result<Template21<'_>, Error> {
    let src = clean_buffer_tpl(src)?;

    let template: Template21 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Open the file at the given path and parse it as a cooked json file from Just Dance 2022
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_v22<P: AsRef<Path>>(path: P) -> Result<Template22Owned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_v22(data))?;
    Ok(Template22Owned::from(yoke))
}

/// Parse a cooked json file from Just Dance 2022
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2022 or the parser encounters an unexpected value.
pub fn parse_v22(src: &[u8]) -> Result<Template22<'_>, Error> {
    let src = clean_buffer_tpl(src)?;

    let template: Template22 = serde_json::from_slice(src)?;

    Ok(template)
}

/// Parse a cooked json file from Just Dance 2022
///
/// # Errors
/// -  the file is not a cooked json file from Just Dance 2022 or the parser encounters an unexpected value.
pub fn parse_v22_lax(src: &[u8]) -> Result<Template22<'_>, Error> {
    let src = clean_buffer_tpl(src).unwrap_or(src);

    let template: Template22 = serde_json::from_slice(src)?;

    Ok(template)
}
