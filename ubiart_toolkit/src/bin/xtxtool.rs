use std::{
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use image::{imageops, ImageBuffer, ImageFormat, Rgba};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
use ubiart_toolkit::{
    cooked::{png::Png, xtx},
    utils::{Game, Platform, UniqueGameId},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    #[arg(short, long, default_value_t = false)]
    info: bool,
    #[arg(short, long, default_value_t = false)]
    xtx_info: bool,
    output: Option<PathBuf>,
}

#[tracing::instrument]
fn main() {
    let cli = Cli::parse();

    let fmt_layer = tracing_subscriber::fmt::layer()
        // Display source code file paths
        .with_file(false)
        // Display source code line numbers
        .with_line_number(false)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(true)
        .without_time();
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let file = File::open(&cli.source).unwrap();
    let png = Png::deserialize_with_ctx(
        &file,
        UniqueGameId {
            game: Game::JustDance2022,
            platform: Platform::Nx,
            id: 0,
        },
    )
    .unwrap();
    let xtx = png.texture.xtx().unwrap();
    if cli.info {
        println!("Width:            0x{:x}", png.width);
        println!("Height:           0x{:x}", png.height);
        println!("Unk2:             0x{:x}", png.unk2);
        println!("Unk5:             0x{:x}", png.unk5);
        println!("Unk8:             0x{:x}", png.unk8);
        println!("Unk9:             0x{:x}", png.unk9);
        println!("Unk10:            0x{:x}", png.unk10);
    }
    if cli.xtx_info {
        println!("XTX Header:");
        println!("Major version: {}", xtx.major_version);
        println!("Minor version: {}", xtx.minor_version);
        for (i, image) in xtx.images.iter().enumerate() {
            println!("XTX Image {i}: {{");
            let data = image.header;
            println!("  Image size:       0x{:x}", data.image_size);
            println!("  Alignment:        0x{:x}", data.alignment);
            println!("  Width:            0x{:x}", data.width);
            println!("  Height:           0x{:x}", data.height);
            println!("  Depth:            0x{:x}", data.depth);
            println!("  Target:           0x{:x}", data.target);
            println!("  Format:           {:?}", data.format);
            println!("  Mipmaps:          0x{:x}", data.mipmaps);
            println!("  Slice size:       0x{:x}", data.slice_size);
            println!("  Mipmap offsets:   0x{:x?}", data.mipmap_offsets);
            println!("  Block Height:     2^{}", data.block_height_log2);
            println!("  Indexes:          0x{:x?}", image.indexes);
            println!("}}");
        }
    }

    if let Some(savepath) = cli.output {
        assert!(xtx.images.len() == 1, "More than one image in texture!");
        assert!(savepath.is_dir(), "Save path is not a directory!");

        let stem = cli
            .source
            .file_stem()
            .and_then(OsStr::to_str)
            .map(Path::new)
            .and_then(Path::file_stem)
            .and_then(OsStr::to_str)
            .unwrap();

        let big_image = xtx.images.first().unwrap();
        let hdr = &big_image.header;

        tracing::trace!("Data size: {}", big_image.data.len());

        let mut offset = 0;

        for (level, index) in big_image.indexes.iter().enumerate() {
            let width = index.width;
            let height = index.height;
            // let offset = index.offset;
            let comp_size = index.size;
            let uncomp_size = width * height * 4;
            tracing::trace!("Level: {level}, width: {width}, height: {height}, offset: {offset}, comp_size: {comp_size}, uncomp_size: {uncomp_size}");
            let mut data_decompressed = vec![0xFF; uncomp_size];
            let data = &big_image.data[offset..offset + comp_size];
            match hdr.format {
                xtx::Format::DXT5 => {
                    texpresso::Format::Bc3.decompress(data, width, height, &mut data_decompressed);
                }
                xtx::Format::DXT1 => {
                    texpresso::Format::Bc1.decompress(data, width, height, &mut data_decompressed);
                }
                _ => panic!("Format {:?} not yet implemented!", hdr.format),
            };

            let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(
                u32::try_from(width).unwrap(),
                u32::try_from(height).unwrap(),
                data_decompressed,
            )
            .unwrap();
            tracing::trace!("width: {width}, height: {height}");

            // Todo: also shift png.width/height by level
            if width != usize::from(1.max(png.width >> level))
                || height != usize::from(1.max(png.height >> level))
            {
                buffer = imageops::resize(
                    &buffer,
                    u32::from(png.width),
                    u32::from(png.height),
                    imageops::FilterType::Lanczos3,
                );
            }

            let path = savepath.join(format!("{stem}.{level}.png"));
            let mut fout = File::create(path).unwrap();
            buffer.write_to(&mut fout, ImageFormat::Png).unwrap();

            offset += comp_size;
        }
    }
}
