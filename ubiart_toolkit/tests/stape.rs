#![allow(clippy::needless_pass_by_value, reason = "Needed by test runner")]

use std::path::Path;

use ubiart_toolkit::{cooked::tape, utils::UniqueGameId};

fn stape_parse_wiiu2015(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::WIIU2015, false)?;
    Ok(())
}

fn stape_parse_wiiu2016(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::WIIU2016, false)?;
    Ok(())
}

fn stape_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2017, false)?;
    Ok(())
}

fn stape_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::WIIU2017, false)?;
    Ok(())
}

fn stape_parse_win2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::WIN2017, false)?;
    Ok(())
}

fn stape_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2018, false)?;
    Ok(())
}

fn stape_parse_nx2019(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2019V2, false)?;
    Ok(())
}

fn stape_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2020, false)?;
    Ok(())
}

fn stape_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX_CHINA, false)?;
    Ok(())
}

fn stape_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2022, false)?;
    Ok(())
}

fn stape_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    tape::parse(&data, UniqueGameId::NX2022, false)?;
    Ok(())
}

datatest_stable::harness!(
    stape_parse_wiiu2015,
    "files/wiiu2015",
    r".*/stape.ckd/.*",
    stape_parse_wiiu2016,
    "files/wiiu2016",
    r".*/stape.ckd/.*",
    stape_parse_nx2017,
    "files/nx2017",
    r".*/stape.ckd/.*",
    stape_parse_win2017,
    "files/win2017",
    r".*/stape.ckd/.*",
    stape_parse_wiiu2017,
    "files/wiiu2017",
    r".*/stape.ckd/.*",
    stape_parse_nx2018,
    "files/nx2018",
    r".*/stape.ckd/.*",
    stape_parse_nx2019,
    "files/nx2019",
    r".*/stape.ckd/.*",
    stape_parse_nx2020,
    "files/nx2020",
    r".*/stape.ckd/.*",
    stape_parse_nx2020_china,
    "files/nxChina",
    r".*/stape.ckd/.*",
    stape_parse_nx2021,
    "files/nx2021",
    r".*/stape.ckd/.*",
    stape_parse_nx2022,
    "files/nx2022",
    r".*/stape.ckd/.*"
);
