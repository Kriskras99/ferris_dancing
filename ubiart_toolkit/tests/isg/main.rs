use std::path::Path;

use ubiart_toolkit::{cooked::json, utils::bytes::read_to_vec};

fn isg_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v17(&data, false)?;
    Ok(())
}

fn isg_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v18(&data, false)?;
    Ok(())
}

fn isg_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v19(&data, false)?;
    Ok(())
}

fn isg_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20(&data, false)?;
    Ok(())
}

fn isg_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20c(&data, false)?;
    Ok(())
}

fn isg_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20(&data, false)?;
    Ok(())
}

fn isg_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v21(&data, false)?;
    Ok(())
}

fn isg_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v22(&data, false)?;
    Ok(())
}

datatest_stable::harness!(
    isg_parse_nx2017,
    "tests/isg/files/nx2017",
    r".*\.isg\.ckd",
    isg_parse_nx2018,
    "tests/isg/files/nx2018",
    r".*\.isg\.ckd",
    isg_parse_nx2019,
    "tests/isg/files/nx2019",
    r".*\.isg\.ckd",
    isg_parse_nx2020,
    "tests/isg/files/nx2020",
    r".*\.isg\.ckd",
    isg_parse_nx2020_china,
    "tests/isg/files/nx2020_china",
    r".*\.isg\.ckd",
    isg_parse_nx2020_japan,
    "tests/isg/files/nx2020_japan",
    r".*\.isg\.ckd",
    isg_parse_nx2021,
    "tests/isg/files/nx2021",
    r".*\.isg\.ckd",
    isg_parse_nx2022,
    "tests/isg/files/nx2022",
    r".*\.isg\.ckd"
);
