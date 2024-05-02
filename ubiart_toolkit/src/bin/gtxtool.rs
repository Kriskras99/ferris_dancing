use std::{
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

use clap::Parser;
use image::{imageops, ImageBuffer, ImageFormat, Rgba};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
use ubiart_toolkit::{
    cooked,
    utils::{Game, Platform, UniqueGameId},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    #[arg(short, long, default_value_t = false)]
    info: bool,
    #[arg(short, long, default_value_t = false)]
    gtx_info: bool,
    #[arg(long)]
    json: Option<PathBuf>,
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
    let png = cooked::png::parse(
        &file,
        UniqueGameId {
            game: Game::JustDance2016,
            platform: Platform::WiiU,
            id: 0,
        },
    )
    .unwrap();
    let gtx = png.texture.gtx().unwrap();
    if cli.info {
        println!("Width:            0x{:x}", png.width);
        println!("Height:           0x{:x}", png.height);
        println!("Unk2:             0x{:x}", png.unk2);
        println!("Unk5:             0x{:x}", png.unk5);
        println!("Unk8:             0x{:x}", png.unk8);
        println!("Unk9:             0x{:x}", png.unk9);
        println!("Unk10:            0x{:x}", png.unk10);
    }
    if cli.gtx_info {
        println!("GTX Header:");
        println!("Align mode: {}", gtx.gfd.align_mode);
        for (i, image) in gtx.images.iter().enumerate() {
            println!("GTX Image {i}: {{");
            let data = image.surface;
            println!("  Dim:                0x{:x}", data.dim);
            println!("  Width:              0x{:x}", data.width);
            println!("  Height:             0x{:x}", data.height);
            println!("  Depth:              0x{:x}", data.depth);
            println!("  Mipmap count:       0x{:x}", data.num_mips);
            println!("  Format:             {:?}", data.format);
            println!("  Use it?:            0x{:x}", data.use_it);
            println!("  Image size:         0x{:x}", data.image_size);
            println!("  Image pointer:      0x{:x}", data.image_ptr);
            println!("  Mipmap size:        0x{:x}", data.mip_size);
            println!("  Mipmap pointer:     0x{:x}", data.mip_ptr);
            println!("  Address tile mode:  {:?}", data.tile_mode);
            println!("  swizzle:            0x{:x}", data.swizzle);
            println!("  Alignment:          0x{:x}", data.alignment);
            println!("  Pitch:              0x{:x}", data.pitch);
            println!("  Mipmap offsets:     0x{:x?}", data.mip_offsets);
            println!("  Bits per pixel:     0x{:x}", data.bpp);
            println!("  Real size:          0x{:x}", data.real_size);
            println!("}}");
        }
    }

    if let Some(savepath) = cli.output {
        assert!(gtx.images.len() == 1, "More than one image in texture!");
        assert!(savepath.is_dir(), "Save path is not a directory!");

        let stem = cli
            .source
            .file_stem()
            .and_then(OsStr::to_str)
            .map(Path::new)
            .and_then(Path::file_stem)
            .and_then(OsStr::to_str)
            .unwrap();

        let big_image = gtx.images.first().unwrap();
        let hdr = &big_image.surface;

        tracing::trace!("Data size: {}", big_image.data.len());

        let width = usize::try_from(hdr.width).unwrap();
        let height = usize::try_from(hdr.height).unwrap();
        // let offset = index.offset;
        let comp_size = usize::try_from(hdr.image_size).unwrap();
        let uncomp_size = width * height * 4;
        tracing::trace!(
            "width: {width}, height: {height}, comp_size: {comp_size}, uncomp_size: {uncomp_size}"
        );
        let mut data_decompressed = vec![0xFF; uncomp_size];
        let data = &big_image.data[..comp_size];
        match hdr.format {
            cooked::gtx::Format::TBc3Unorm | cooked::gtx::Format::TBc3Srgb => {
                texpresso::Format::Bc3.decompress(data, width, height, &mut data_decompressed);
            }
            cooked::gtx::Format::TBc1Unorm | cooked::gtx::Format::TBc1Srgb => {
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
        if width != usize::from(png.width) || height != usize::from(png.height) {
            buffer = imageops::resize(
                &buffer,
                u32::from(png.width),
                u32::from(png.height),
                imageops::FilterType::Lanczos3,
            );
        }

        let path = savepath.join(format!("{stem}.png"));
        let mut fout = File::create(path).unwrap();
        buffer.write_to(&mut fout, ImageFormat::Png).unwrap();
    }
}
