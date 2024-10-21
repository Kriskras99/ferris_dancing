#![allow(clippy::needless_pass_by_value, reason = "Needed by test runner")]

use std::path::Path;

use ubiart_toolkit::cooked::json;

fn frt_parse_wiiu2016(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    json::parse_v16(&data, false)?;
    Ok(())
}

fn frt_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    json::parse_v17(&data, false)?;
    Ok(())
}

fn frt_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    json::parse_v17(&data, false)?;
    Ok(())
}

fn frt_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    json::parse_v18(&data, false)?;
    Ok(())
}

fn frt_parse_nx2019(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    json::parse_v19(&data, false)?;
    Ok(())
}

fn frt_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    json::parse_v20(&data, false)?;
    Ok(())
}

fn frt_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    json::parse_v20c(&data, false)?;
    Ok(())
}

fn frt_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    json::parse_v21(&data, false)?;
    Ok(())
}

fn frt_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    json::parse_v22(&data, false)?;
    Ok(())
}

datatest_stable::harness!(
    frt_parse_wiiu2016,
    "files/wiiu2016",
    r".*/frt.ckd/.*",
    frt_parse_nx2017,
    "files/nx2017",
    r".*/frt.ckd/.*",
    frt_parse_wiiu2017,
    "files/wiiu2017",
    r".*/frt.ckd/.*",
    frt_parse_nx2018,
    "files/nx2018",
    r".*/frt.ckd/.*",
    frt_parse_nx2019,
    "files/nx2019",
    r".*/frt.ckd/.*",
    frt_parse_nx2020,
    "files/nx2020",
    r".*/frt.ckd/.*",
    frt_parse_nx2020_china,
    "files/nxChina",
    r".*/frt.ckd/.*",
    frt_parse_nx2021,
    "files/nx2021",
    r".*/frt.ckd/.*",
    frt_parse_nx2022,
    "files/nx2022",
    r".*/frt.ckd/.*"
);
