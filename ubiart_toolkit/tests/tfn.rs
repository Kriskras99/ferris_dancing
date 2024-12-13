#![allow(clippy::needless_pass_by_value, reason = "Needed by test runner")]

use std::path::Path;

use ubiart_toolkit::cooked::tfn;

fn tfn_parse_wiiu2016(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tfn::parse(&data)?;
    Ok(())
}

fn tfn_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tfn::parse(&data)?;
    Ok(())
}

fn tfn_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tfn::parse(&data)?;
    Ok(())
}

fn tfn_parse_win2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tfn::parse(&data)?;
    Ok(())
}

fn tfn_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tfn::parse(&data)?;
    Ok(())
}

fn tfn_parse_nx2019(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tfn::parse(&data)?;
    Ok(())
}

fn tfn_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tfn::parse(&data)?;
    Ok(())
}

fn tfn_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tfn::parse(&data)?;
    Ok(())
}

fn tfn_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tfn::parse(&data)?;
    Ok(())
}

fn tfn_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tfn::parse(&data)?;
    Ok(())
}

datatest_stable::harness!(
    tfn_parse_wiiu2016,
    "files/wiiu2016",
    r".*/tfn.ckd/.*",
    tfn_parse_nx2017,
    "files/nx2017",
    r".*/tfn.ckd/.*",
    tfn_parse_wiiu2017,
    "files/wiiu2017",
    r".*/tfn.ckd/.*",
    tfn_parse_win2017,
    "files/win2017",
    r".*/tfn.ckd/.*",
    tfn_parse_nx2018,
    "files/nx2018",
    r".*/tfn.ckd/.*",
    tfn_parse_nx2019,
    "files/nx2019",
    r".*/tfn.ckd/.*",
    tfn_parse_nx2020,
    "files/nx2020",
    r".*/tfn.ckd/.*",
    tfn_parse_nx2020_china,
    "files/nxChina",
    r".*/tfn.ckd/.*",
    tfn_parse_nx2021,
    "files/nx2021",
    r".*/tfn.ckd/.*",
    tfn_parse_nx2022,
    "files/nx2022",
    r".*/tfn.ckd/.*"
);
