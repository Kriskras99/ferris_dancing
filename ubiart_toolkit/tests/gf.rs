#![allow(clippy::needless_pass_by_value, reason = "Needed by test runner")]
use std::path::Path;

use dotstar_toolkit_utils::bytes::read::BinaryDeserializeExt as _;
use ubiart_toolkit::secure_fat::SecureFat;

fn secure_fat_parse_wiiu2015(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    SecureFat::deserialize(&data)?;
    Ok(())
}

fn secure_fat_parse_wiiu2016(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    SecureFat::deserialize(&data)?;
    Ok(())
}

fn secure_fat_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    SecureFat::deserialize(&data)?;
    Ok(())
}

fn secure_fat_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    SecureFat::deserialize(&data)?;
    Ok(())
}

fn secure_fat_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    SecureFat::deserialize(&data)?;
    Ok(())
}

fn secure_fat_parse_nx2019(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    SecureFat::deserialize(&data)?;
    Ok(())
}

fn secure_fat_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    SecureFat::deserialize(&data)?;
    Ok(())
}

fn secure_fat_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    SecureFat::deserialize(&data)?;
    Ok(())
}

fn secure_fat_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    SecureFat::deserialize(&data)?;
    Ok(())
}

fn secure_fat_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    SecureFat::deserialize(&data)?;
    Ok(())
}

datatest_stable::harness!(
    secure_fat_parse_wiiu2015,
    "files/wiiu2016",
    r".*/gf/.*",
    secure_fat_parse_wiiu2016,
    "files/wiiu2016",
    r".*/gf/.*",
    secure_fat_parse_nx2017,
    "files/nx2017",
    r".*/gf/.*",
    secure_fat_parse_wiiu2017,
    "files/wiiu2017",
    r".*/gf/.*",
    secure_fat_parse_nx2018,
    "files/nx2018",
    r".*/gf/.*",
    secure_fat_parse_nx2019,
    "files/nx2019",
    r".*/gf/.*",
    secure_fat_parse_nx2020,
    "files/nx2020",
    r".*/gf/.*",
    secure_fat_parse_nx2020_china,
    "files/nxChina",
    r".*/gf/.*",
    secure_fat_parse_nx2021,
    "files/nx2021",
    r".*/gf/.*",
    secure_fat_parse_nx2022,
    "files/nx2022",
    r".*/gf/.*"
);
