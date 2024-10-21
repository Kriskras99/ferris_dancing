#![allow(clippy::needless_pass_by_value, reason = "Needed by test runner")]

use std::path::Path;

use dotstar_toolkit_utils::bytes::read::BinaryDeserialize as _;
use ubiart_toolkit::{cooked::png::Png, utils::UniqueGameId};

fn tga_parse_wiiu2015(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::WIIU2015)?;
    Ok(())
}

fn tga_parse_wiiu2016(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::WIIU2016)?;
    Ok(())
}

fn tga_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2017)?;
    Ok(())
}

fn tga_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::WIIU2017)?;
    Ok(())
}

fn tga_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2018)?;
    Ok(())
}

fn tga_parse_nx2019(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2019V2)?;
    Ok(())
}

fn tga_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2020)?;
    Ok(())
}

fn tga_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX_CHINA)?;
    Ok(())
}

fn tga_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2021)?;
    Ok(())
}

fn tga_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2022)?;
    Ok(())
}

datatest_stable::harness!(
    tga_parse_wiiu2015,
    "files/wiiu2015",
    r".*/tga.ckd/.*",
    tga_parse_wiiu2016,
    "files/wiiu2016",
    r".*/tga.ckd/.*",
    tga_parse_nx2017,
    "files/nx2017",
    r".*/tga.ckd/.*",
    tga_parse_wiiu2017,
    "files/wiiu2017",
    r".*/tga.ckd/.*",
    tga_parse_nx2018,
    "files/nx2018",
    r".*/tga.ckd/.*",
    tga_parse_nx2019,
    "files/nx2019",
    r".*/tga.ckd/.*",
    tga_parse_nx2020,
    "files/nx2020",
    r".*/tga.ckd/.*",
    tga_parse_nx2020_china,
    "files/nxChina",
    r".*/tga.ckd/.*",
    tga_parse_nx2021,
    "files/nx2021",
    r".*/tga.ckd/.*",
    tga_parse_nx2022,
    "files/nx2022",
    r".*/tga.ckd/.*"
);
