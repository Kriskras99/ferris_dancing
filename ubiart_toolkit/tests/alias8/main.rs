use std::path::Path;

use byteorder::BigEndian;
use dotstar_toolkit_utils::{bytes::read_to_vec, bytes_new::read::BinaryDeserialize};
use ubiart_toolkit::alias8::Alias8;

fn alias8_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = Alias8::deserialize::<BigEndian>(&data.as_slice())?;
    Ok(())
}

fn alias8_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = Alias8::deserialize::<BigEndian>(&data.as_slice())?;
    Ok(())
}

fn alias8_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = Alias8::deserialize::<BigEndian>(&data.as_slice())?;
    Ok(())
}

fn alias8_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = Alias8::deserialize::<BigEndian>(&data.as_slice())?;
    Ok(())
}

fn alias8_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = Alias8::deserialize::<BigEndian>(&data.as_slice())?;
    Ok(())
}

fn alias8_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = Alias8::deserialize::<BigEndian>(&data.as_slice())?;
    Ok(())
}

fn alias8_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = Alias8::deserialize::<BigEndian>(&data.as_slice())?;
    Ok(())
}

fn alias8_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = Alias8::deserialize::<BigEndian>(&data.as_slice())?;
    Ok(())
}

datatest_stable::harness!(
    alias8_parse_nx2017,
    "tests/alias8/files/nx2017",
    r".*\.alias8",
    alias8_parse_nx2018,
    "tests/alias8/files/nx2018",
    r".*\.alias8",
    alias8_parse_nx2019,
    "tests/alias8/files/nx2019",
    r".*\.alias8",
    alias8_parse_nx2020,
    "tests/alias8/files/nx2020",
    r".*\.alias8",
    alias8_parse_nx2020_china,
    "tests/alias8/files/nx2020_china",
    r".*\.alias8",
    alias8_parse_nx2020_japan,
    "tests/alias8/files/nx2020_japan",
    r".*\.alias8",
    alias8_parse_nx2021,
    "tests/alias8/files/nx2021",
    r".*\.alias8",
    alias8_parse_nx2022,
    "tests/alias8/files/nx2022",
    r".*\.alias8"
);
