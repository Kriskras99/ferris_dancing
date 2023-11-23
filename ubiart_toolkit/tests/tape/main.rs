use std::path::Path;

use ubiart_toolkit::{cooked::json, utils::bytes::read_to_vec};

fn tape_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v17(&data, false)?;
    Ok(())
}

fn tape_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v18(&data, false)?;
    Ok(())
}

fn tape_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v19(&data, false)?;
    Ok(())
}

fn tape_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20(&data, false)?;
    Ok(())
}

fn tape_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20c(&data, false)?;
    Ok(())
}

fn tape_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20(&data, false)?;
    Ok(())
}

fn tape_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v21(&data, false)?;
    Ok(())
}

fn tape_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v22(&data, false)?;
    Ok(())
}

datatest_stable::harness!(
    tape_parse_nx2017,
    "tests/tape/files/nx2017",
    r".*\.tape\.ckd",
    tape_parse_nx2018,
    "tests/tape/files/nx2018",
    r".*\.tape\.ckd",
    tape_parse_nx2019,
    "tests/tape/files/nx2019",
    r".*\.tape\.ckd",
    tape_parse_nx2020,
    "tests/tape/files/nx2020",
    r".*\.tape\.ckd",
    tape_parse_nx2020_china,
    "tests/tape/files/nx2020_china",
    r".*\.tape\.ckd",
    tape_parse_nx2020_japan,
    "tests/tape/files/nx2020_japan",
    r".*\.tape\.ckd",
    tape_parse_nx2021,
    "tests/tape/files/nx2021",
    r".*\.tape\.ckd",
    tape_parse_nx2022,
    "tests/tape/files/nx2022",
    r".*\.tape\.ckd"
);
