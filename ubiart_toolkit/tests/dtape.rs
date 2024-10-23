#![allow(clippy::needless_pass_by_value, reason = "Needed by test runner")]

use std::path::Path;

use ubiart_toolkit::{cooked::tape, utils::UniqueGameId};

fn dtape_parse_wiiu2015(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::WIIU2015)?;
    Ok(())
}

fn dtape_parse_wiiu2016(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::WIIU2016)?;
    Ok(())
}

fn dtape_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2017)?;
    Ok(())
}

fn dtape_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::WIIU2017)?;
    Ok(())
}

fn dtape_parse_win2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::WIN2017)?;
    Ok(())
}

fn dtape_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2018)?;
    Ok(())
}

fn dtape_parse_nx2019(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2019V2)?;
    Ok(())
}

fn dtape_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2020)?;
    Ok(())
}

fn dtape_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX_CHINA)?;
    Ok(())
}

fn dtape_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2022)?;
    Ok(())
}

fn dtape_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2022)?;
    Ok(())
}

datatest_stable::harness!(
    dtape_parse_wiiu2015,
    "files/wiiu2015",
    r".*/dtape.ckd/.*",
    dtape_parse_wiiu2016,
    "files/wiiu2016",
    r".*/dtape.ckd/.*",
    dtape_parse_nx2017,
    "files/nx2017",
    r".*/dtape.ckd/.*",
    dtape_parse_wiiu2017,
    "files/wiiu2017",
    r".*/dtape.ckd/.*",
    dtape_parse_win2017,
    "files/win2017",
    r".*/dtape.ckd/.*",
    dtape_parse_nx2018,
    "files/nx2018",
    r".*/dtape.ckd/.*",
    dtape_parse_nx2019,
    "files/nx2019",
    r".*/dtape.ckd/.*",
    dtape_parse_nx2020,
    "files/nx2020",
    r".*/dtape.ckd/.*",
    dtape_parse_nx2020_china,
    "files/nxChina",
    r".*/dtape.ckd/.*",
    dtape_parse_nx2021,
    "files/nx2021",
    r".*/dtape.ckd/.*",
    dtape_parse_nx2022,
    "files/nx2022",
    r".*/dtape.ckd/.*"
);
