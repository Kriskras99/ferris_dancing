use std::path::Path;

use dotstar_toolkit_utils::bytes::read_to_vec;
use ubiart_toolkit::{cooked::png, utils::UniqueGameId};

fn tga_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = png::parse(&data, UniqueGameId::NX2017);
    Ok(())
}

fn tga_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = png::parse(&data, UniqueGameId::NX2018);
    Ok(())
}

fn tga_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = png::parse(&data, UniqueGameId::NX2019);
    Ok(())
}

fn tga_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = png::parse(&data, UniqueGameId::NX2020);
    Ok(())
}

fn tga_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = png::parse(&data, UniqueGameId::NX_CHINA);
    Ok(())
}

fn tga_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = png::parse(&data, UniqueGameId::NX2021);
    Ok(())
}

fn tga_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = png::parse(&data, UniqueGameId::NX2022);
    Ok(())
}

datatest_stable::harness!(
    tga_parse_nx2017,
    "files/2017",
    r".*/tga.ckd/.*",
    tga_parse_nx2018,
    "files/2018",
    r".*/tga.ckd/.*",
    tga_parse_nx2019,
    "files/2019",
    r".*/tga.ckd/.*",
    tga_parse_nx2020,
    "files/2020",
    r".*/tga.ckd/.*",
    tga_parse_nx2020_china,
    "files/China",
    r".*/tga.ckd/.*",
    tga_parse_nx2021,
    "files/2021",
    r".*/tga.ckd/.*",
    tga_parse_nx2022,
    "files/2022",
    r".*/tga.ckd/.*"
);
