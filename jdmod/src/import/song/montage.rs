//! # Pictogram Spritesheest
//! Code for converting pictogram spritesheets into individual pictograms
use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::testing::{test_eq, test_ge, test_le};
use image::imageops;

use super::SongImportState;
use crate::utils::decode_texture;

/// The height of every picto
const PICTO_HEIGHT: u32 = 0x200;

/// Converts a montage.png.ckd into individual pictos
///
/// `picto_filenames`: needs to be a sorted list
pub fn import(
    sis: &SongImportState<'_>,
    montage_path: &str,
    picto_filenames: &[&str],
) -> Result<(), Error> {
    // Open and decode the montage file
    let montage_file = sis.vfs.open(montage_path.as_ref())?;
    let buffer = decode_texture(&montage_file, sis.ugi)?;
    let montage_width = buffer.width();
    let montage_height = buffer.height();

    // "Calculate" the width of the individual pictos
    let picto_width = match montage_width {
        0xB90 => 0x2E4,
        0x800 | 0x1000 => 0x200,
        _ => return Err(anyhow!("Unknown width! {:x}", montage_width)),
    };

    // Calculate the amount of pictos in the montage
    let pictos_horizontal = montage_width / picto_width;
    let pictos_vertical = montage_height / PICTO_HEIGHT;
    test_eq(&(montage_width % picto_width), &0)
        .context("Montage is not divisble by expected pictogram width!")?;
    test_eq(&(montage_height % PICTO_HEIGHT), &0)
        .context("Montage is not divisible by pictogram height!")?;
    test_ge(
        &picto_filenames.len(),
        &usize::try_from(pictos_horizontal * (pictos_vertical - 1) + 1)?,
    )
    .context("Not enough filenames for montage size!")?;
    test_le(
        &picto_filenames.len(),
        &usize::try_from(pictos_horizontal * pictos_vertical)?,
    )
    .context("Too many filenames for montage size!")?;

    let mut pictos = Vec::with_capacity(usize::try_from(pictos_horizontal * pictos_vertical)?);

    for vertical in 0..pictos_vertical {
        for horizontal in 0..pictos_horizontal {
            pictos.push(
                imageops::crop_imm(
                    &buffer,
                    horizontal * picto_width,
                    vertical * PICTO_HEIGHT,
                    picto_width,
                    PICTO_HEIGHT,
                )
                .to_image(),
            );
        }
    }

    for (filename, image) in picto_filenames.iter().zip(pictos) {
        let path = sis.dirs.pictos().join(filename);
        image.save(path)?;
    }

    Ok(())
}
