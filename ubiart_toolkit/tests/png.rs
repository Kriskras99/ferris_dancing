#![allow(clippy::needless_pass_by_value)]
use std::path::Path;

use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use ubiart_toolkit::{cooked::png::Png, utils::UniqueGameId};

fn png_parse_wiiu2015(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::WIIU2015)?;
    Ok(())
}

fn png_parse_wiiu2016(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::WIIU2016)?;
    Ok(())
}

fn png_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2017)?;
    Ok(())
}

fn png_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::WIIU2017)?;
    Ok(())
}

fn png_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2018)?;
    Ok(())
}

fn png_parse_nx2019(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2019)?;
    Ok(())
}

fn png_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2020)?;
    Ok(())
}

fn png_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX_CHINA)?;
    Ok(())
}

fn png_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2021)?;
    Ok(())
}

fn png_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Png::deserialize_with(&data, UniqueGameId::NX2022)?;
    Ok(())
}

datatest_stable::harness!(
    png_parse_wiiu2015,
    "files/wiiu2015",
    r".*/png.ckd/.*",
    png_parse_wiiu2016,
    "files/wiiu2016",
    r".*/png.ckd/.*",
    png_parse_nx2017,
    "files/nx2017",
    r".*/png.ckd/.*",
    png_parse_wiiu2017,
    "files/wiiu2017",
    r".*/png.ckd/.*",
    png_parse_nx2018,
    "files/nx2018",
    r".*/png.ckd/.*",
    png_parse_nx2019,
    "files/nx2019",
    r".*/png.ckd/.*",
    png_parse_nx2020,
    "files/nx2020",
    r".*/png.ckd/.*",
    png_parse_nx2020_china,
    "files/nxChina",
    r".*/png.ckd/.*",
    png_parse_nx2021,
    "files/nx2021",
    r".*/png.ckd/.*",
    png_parse_nx2022,
    "files/nx2022",
    r".*/png.ckd/.*"
);
