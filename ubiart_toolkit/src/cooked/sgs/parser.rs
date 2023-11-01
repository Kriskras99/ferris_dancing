use std::{fs, path::Path};

use anyhow::Error;
use memmap2::Mmap;
use yoke::Yoke;

use crate::utils::testing::test;

use super::{
    SceneConfigManager, SceneConfigManagerOwned, SceneSettings, SceneSettingsOwned, Sgs, SgsOwned,
};

/// Open the file at the given path and parse it as a sgs file
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open<P: AsRef<Path>>(path: P) -> Result<SgsOwned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse(data))?;
    Ok(SgsOwned::from(yoke))
}

/// Parse a sgs file
///
/// # Errors
/// -  the file is not a sgs file or the parser encounters an unexpected value.
pub fn parse(src: &[u8]) -> Result<Sgs<'_>, Error> {
    let src = clean_buffer_sgs(src)?;
    let sgs: Sgs = serde_json::from_slice(src)?;

    Ok(sgs)
}

/// Open the file at the given path and parse it as a SceneSettings file
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_sgs<P: AsRef<Path>>(path: P) -> Result<SceneSettingsOwned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_sgs(data))?;
    Ok(SceneSettingsOwned::from(yoke))
}

/// Parse a SceneSettings file
///
/// # Errors
/// -  the file is not a sgs file or the parser encounters an unexpected value.
pub fn parse_sgs(src: &[u8]) -> Result<SceneSettings<'_>, Error> {
    let src = clean_buffer_sgs(src)?;
    let template: SceneSettings = serde_json::from_slice(src)?;

    Ok(template)
}

/// Open the file at the given path and parse it as a SgsContainer file
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open_sgscontainer<P: AsRef<Path>>(path: P) -> Result<SceneConfigManagerOwned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse_sgscontainer(data))?;
    Ok(SceneConfigManagerOwned::from(yoke))
}

/// Parse a SgsContainer file
///
/// # Errors
/// -  the file is not a sgs file or the parser encounters an unexpected value.
pub fn parse_sgscontainer(src: &[u8]) -> Result<SceneConfigManager<'_>, Error> {
    let src = clean_buffer_sgs(src)?;
    let sgscontainer: SceneConfigManager = serde_json::from_slice(src)?;

    Ok(sgscontainer)
}

/// Remove the 'S' at the front and '\0' at the back of the buffer.
///
/// # Errors
/// Will error when the 'S' or the '\0' are missing
fn clean_buffer_sgs(buffer: &[u8]) -> Result<&[u8], Error> {
    test(&buffer[0], &b'S')?;
    test(&buffer[buffer.len() - 1], &0x0)?;

    Ok(&buffer[1..buffer.len() - 1])
}
