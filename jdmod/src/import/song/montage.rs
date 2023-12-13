//! # Pictogram Spritesheest
//! Code for converting pictogram spritesheets into individual pictograms
use anyhow::{anyhow, Error};
use image::{imageops, ImageBuffer, RgbaImage};
use texpresso::Format;
use ubiart_toolkit::cooked::{self, xtx};

use super::SongImportState;

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
    // Open the montage file
    let montage_file = sis.vfs.open(montage_path.as_ref())?;
    let montage = cooked::png::parse(&montage_file)?;
    let montage_width = u32::from(montage.width);
    let montage_height = u32::from(montage.height);

    // "Calculate" the width of the individual pictos
    let picto_width = match montage.width {
        0xB90 => 0x2E4,
        0x800 | 0x1000 => 0x200,
        _ => panic!("Unknown width! {:x}", montage.width),
    };

    // Calculate the amount of pictos in the montage
    let pictos_horizontal = montage_width / picto_width;
    let pictos_vertical = montage_height / PICTO_HEIGHT;
    assert!(
        montage_width % picto_width == 0,
        "Montage is not divisble by expected pictogram width!"
    );
    assert!(
        montage_height % PICTO_HEIGHT == 0,
        "Montage is not divisible by pictogram height!"
    );
    assert!(
        picto_filenames.len() >= usize::try_from(pictos_horizontal * (pictos_vertical - 1) + 1)?,
        "Not enough filenames for montage size!"
    );
    assert!(
        picto_filenames.len() <= usize::try_from(pictos_horizontal * pictos_vertical)?,
        "Too many filenames for montage size!"
    );

    // Never encountered this, so better quit early
    assert!(
        montage.xtx.images.len() == 1,
        "More than one image in montage!"
    );

    let big_image = &montage.xtx.images[0];
    let header = &big_image.header;
    assert!(
        header.format == xtx::Format::DXT5,
        "Codec is not DXT5! {:?}",
        header.format
    );

    // Decode the image
    let data_compressed = &big_image.data[0];
    // TODO: Replace with Vec::with_capacity
    let mut data_decompressed = vec![0xFF; usize::try_from(header.width * header.height * 4)?];
    Format::Bc3.decompress(
        data_compressed,
        usize::try_from(header.width)?,
        usize::try_from(header.height)?,
        &mut data_decompressed,
    );
    let mut buffer: RgbaImage =
        ImageBuffer::from_vec(header.width, header.height, data_decompressed)
            .ok_or_else(|| anyhow!("Not a RGBA image!?!"))?;

    // Resize to size specified in the png header, if required
    if montage_width != header.width || montage_height != header.height {
        buffer = imageops::resize(
            &buffer,
            montage_width,
            montage_height,
            imageops::FilterType::Lanczos3,
        );
    }

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
