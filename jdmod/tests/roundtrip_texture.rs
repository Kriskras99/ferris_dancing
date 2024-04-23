use std::{io::Cursor, path::Path};

use dotstar_toolkit_utils::{
    bytes::read_to_vec,
    vfs::{vecfs::VecFs, VirtualPath},
};
use jdmod::utils::{decode_texture, encode_texture};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
use ubiart_toolkit::{cooked::png, utils::UniqueGameId};

fn tga_roundtrip(input: &Path) -> datatest_stable::Result<()> {
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

    let data = read_to_vec(input)?;
    let one = decode_texture(&data, UniqueGameId::NX2022)?;
    let mut cursor = Cursor::new(Vec::new());
    one.write_to(&mut cursor, image::ImageFormat::Png)?;
    let content = cursor.into_inner();
    let mut fs = VecFs::with_capacity(1);
    fs.add_file("decoded.png".into(), content)?;
    let two = encode_texture(&fs, VirtualPath::new("decoded.png"))?;
    let three = png::create_vec(&two)?;
    let _four = decode_texture(&three, UniqueGameId::NX2022)?;
    // if one != four {
    //     panic!("did not match! {input:?}")
    // }
    Ok(())
}

datatest_stable::harness!(
    tga_roundtrip,
    "../ubiart_toolkit/files/2022",
    r".*/tga.ckd/.*",
    tga_roundtrip,
    "../ubiart_toolkit/files/2022",
    r".*/png.ckd/.*"
);
