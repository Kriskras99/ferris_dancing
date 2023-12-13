use std::{
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Error};
use clap::Parser;
use image::{imageops, ImageBuffer, ImageOutputFormat, Rgba};
use serde::Serialize;
use ubiart_toolkit::{
    cooked::{
        self,
        png::Png,
        xtx::{Format, Image, Xtx},
    },
    utils::bytes::read_to_vec,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    #[arg(short, long, default_value_t = false)]
    info: bool,
    #[arg(short, long, default_value_t = false)]
    xtx_info: bool,
    #[arg(long)]
    json: Option<PathBuf>,
    output: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let data = read_to_vec(&cli.source)?;
    let png = cooked::png::parse(&data)?;
    drop(data);
    if cli.info {
        println!("Width:            0x{:x}", png.width);
        println!("Height:           0x{:x}", png.height);
        println!("Unk5:             0x{:x}", png.unk5);
        println!("Unk8:             0x{:x}", png.unk8);
        println!("Unk9:             0x{:x}", png.unk9);
        println!("Unk10:            0x{:x}", png.unk10);
    }
    if cli.xtx_info {
        println!("XTX Header:");
        println!("Major version: {}", png.xtx.major_version);
        println!("Minor version: {}", png.xtx.minor_version);
        for (i, image) in png.xtx.images.iter().enumerate() {
            println!("XTX Image {i}: {{");
            let data = image.header;
            println!("  Image size:     0x{:x}", data.image_size);
            println!("  Alignment:      0x{:x}", data.alignment);
            println!("  Width:          0x{:x}", data.width);
            println!("  Height:         0x{:x}", data.height);
            println!("  Depth:          0x{:x}", data.depth);
            println!("  Target:         0x{:x}", data.target);
            println!("  Format:         {:?}", data.format);
            println!("  Mipmaps:        {:x}", data.mipmaps);
            println!("  Slice size:     0x{:x}", data.slice_size);
            println!("  Mipmap offsets: 0x{:x?}", data.mipmap_offsets);
            println!("  Unk1:           0x{:x}", data.unk1);
            println!("}}");
        }
    }

    if let Some(savepath) = cli.output {
        assert!(png.xtx.images.len() == 1, "More than one image in texture!");
        assert!(savepath.is_dir(), "Save path is not a directory!");

        let stem = cli
            .source
            .file_stem()
            .and_then(OsStr::to_str)
            .map(Path::new)
            .and_then(Path::file_stem)
            .and_then(OsStr::to_str)
            .unwrap();

        let big_image = png
            .xtx
            .images
            .first()
            .ok_or_else(|| anyhow!("No image found!"))?;
        if big_image.data.len() > 1 {
            println!("Warning! Not extracting mipmaps, only original image!");
        }
        let hdr = &big_image.header;
        let data_compressed = big_image
            .data
            .first()
            .ok_or_else(|| anyhow!("No data for image!"))?;
        let mut data_decompressed =
            vec![0xFF; usize::try_from(hdr.width * hdr.height * 4).unwrap()];
        match hdr.format {
            cooked::xtx::Format::DXT5 => {
                texpresso::Format::Bc3.decompress(
                    data_compressed,
                    usize::try_from(hdr.width).unwrap(),
                    usize::try_from(hdr.height).unwrap(),
                    &mut data_decompressed,
                );
            }
            cooked::xtx::Format::DXT1 => {
                texpresso::Format::Bc1.decompress(
                    data_compressed,
                    usize::try_from(hdr.width).unwrap(),
                    usize::try_from(hdr.height).unwrap(),
                    &mut data_decompressed,
                );
            }
            _ => panic!("Format {:?} not yet implemented!", hdr.format),
        };

        let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_vec(hdr.width, hdr.height, data_decompressed).unwrap();

        if hdr.width != u32::from(png.width) || hdr.height != u32::from(png.height) {
            buffer = imageops::resize(
                &buffer,
                u32::from(png.width),
                u32::from(png.height),
                imageops::FilterType::Lanczos3,
            );
        }

        let path = savepath.join(format!("{stem}.png"));
        let mut fout = File::create(path).unwrap();
        buffer.write_to(&mut fout, ImageOutputFormat::Png).unwrap();
    }

    if let Some(json_path) = cli.json {
        let metadata = Metadata::from(&png);
        let file = File::create(json_path)?;
        serde_json::to_writer_pretty(file, &metadata)?;
    }

    Ok(())
}

#[derive(Serialize)]
pub struct Metadata {
    pub png: PngMetadata,
    pub xtx: XtxMetadata,
}

impl From<&Png> for Metadata {
    fn from(value: &Png) -> Self {
        Self {
            png: PngMetadata::from(value),
            xtx: XtxMetadata::from(&value.xtx),
        }
    }
}

#[derive(Serialize)]
pub struct PngMetadata {
    pub width: u16,
    pub height: u16,
    pub unk2: u32,
    pub unk5: u16,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u16,
}

impl From<&Png> for PngMetadata {
    fn from(value: &Png) -> Self {
        Self {
            width: value.width,
            height: value.height,
            unk2: value.unk2,
            unk5: value.unk5,
            unk8: value.unk8,
            unk9: value.unk9,
            unk10: value.unk10,
        }
    }
}

#[derive(Serialize)]
pub struct XtxMetadata {
    pub major_version: u32,
    pub minor_version: u32,
    pub images: Vec<XtxImageMetadata>,
}

impl From<&Xtx> for XtxMetadata {
    fn from(value: &Xtx) -> Self {
        Self {
            major_version: value.major_version,
            minor_version: value.minor_version,
            images: value.images.iter().map(XtxImageMetadata::from).collect(),
        }
    }
}

#[derive(Serialize)]
pub struct XtxImageMetadata {
    pub image_size: u64,
    pub alignment: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub target: u32,
    pub format: Format,
    pub mipmaps: u32,
    pub slice_size: u32,
    pub mipmap_offsets: [u32; 0x10],
    pub unk1: u64,
    pub data: Vec<usize>,
}

impl From<&Image> for XtxImageMetadata {
    fn from(value: &Image) -> Self {
        Self {
            image_size: value.header.image_size,
            alignment: value.header.alignment,
            width: value.header.width,
            height: value.header.height,
            depth: value.header.depth,
            target: value.header.target,
            format: value.header.format,
            mipmaps: value.header.mipmaps,
            slice_size: value.header.slice_size,
            mipmap_offsets: value.header.mipmap_offsets,
            unk1: value.header.unk1,
            data: value.data.iter().map(Vec::len).collect(),
        }
    }
}
