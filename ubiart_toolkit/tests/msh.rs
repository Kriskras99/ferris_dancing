#![allow(clippy::needless_pass_by_value, reason = "Needed by test runner")]

use std::path::Path;

use ubiart_toolkit::cooked::msh;

fn msh_parse_wiiu2016(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    msh::parse(&data)?;
    Ok(())
}

fn msh_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    msh::parse(&data)?;
    Ok(())
}

fn msh_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    msh::parse(&data)?;
    Ok(())
}

fn msh_parse_win2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    msh::parse(&data)?;
    Ok(())
}

fn msh_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    msh::parse(&data)?;
    Ok(())
}

fn msh_parse_nx2019(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    msh::parse(&data)?;
    Ok(())
}

fn msh_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    msh::parse(&data)?;
    Ok(())
}

fn msh_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    msh::parse(&data)?;
    Ok(())
}

fn msh_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    msh::parse(&data)?;
    Ok(())
}

fn msh_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    msh::parse(&data)?;
    Ok(())
}

datatest_stable::harness!(
    msh_parse_wiiu2016,
    "files/wiiu2016",
    r".*/msh.ckd/.*",
    msh_parse_nx2017,
    "files/nx2017",
    r".*/msh.ckd/.*",
    msh_parse_win2017,
    "files/win2017",
    r".*/msh.ckd/.*",
    msh_parse_wiiu2017,
    "files/wiiu2017",
    r".*/msh.ckd/.*",
    msh_parse_nx2018,
    "files/nx2018",
    r".*/msh.ckd/.*",
    msh_parse_nx2019,
    "files/nx2019",
    r".*/msh.ckd/.*",
    msh_parse_nx2020,
    "files/nx2020",
    r".*/msh.ckd/.*",
    msh_parse_nx2020_china,
    "files/nxChina",
    r".*/msh.ckd/.*",
    msh_parse_nx2021,
    "files/nx2021",
    r".*/msh.ckd/.*",
    msh_parse_nx2022,
    "files/nx2022",
    r".*/msh.ckd/.*"
);
