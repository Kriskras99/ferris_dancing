#![allow(clippy::needless_pass_by_value, reason = "Needed by test runner")]

use std::path::Path;

use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use ubiart_toolkit::{cooked::act::Actor, utils::UniqueGameId};

fn act_parse_wiiu2015(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with(&data, UniqueGameId::WIIU2015)?;
    Ok(())
}

fn act_parse_wiiu2016(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with(&data, UniqueGameId::WIIU2016)?;
    Ok(())
}

fn act_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with(&data, UniqueGameId::NX2017)?;
    Ok(())
}

fn act_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with(&data, UniqueGameId::WIIU2017)?;
    Ok(())
}

fn act_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with(&data, UniqueGameId::NX2018)?;
    Ok(())
}

fn act_parse_nx2019v1(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with(&data, UniqueGameId::NX2019V1)?;
    Ok(())
}

fn act_parse_nx2019v2(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with(&data, UniqueGameId::NX2019V2)?;
    Ok(())
}

fn act_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with(&data, UniqueGameId::NX2020)?;
    Ok(())
}

fn act_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with(&data, UniqueGameId::NX_CHINA)?;
    Ok(())
}

fn act_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with(&data, UniqueGameId::NX2021)?;
    Ok(())
}

fn act_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with(&data, UniqueGameId::NX2022)?;
    Ok(())
}

datatest_stable::harness!(
    act_parse_wiiu2015,
    "files/wiiu2015",
    r".*/act.ckd/.*",
    act_parse_wiiu2016,
    "files/wiiu2016",
    r".*/act.ckd/.*",
    act_parse_nx2017,
    "files/nx2017",
    r".*/act.ckd/.*",
    act_parse_wiiu2017,
    "files/wiiu2017",
    r".*/act.ckd/.*",
    act_parse_nx2018,
    "files/nx2018",
    r".*/act.ckd/.*",
    act_parse_nx2019v1, // 284652.419607  286387.428726  287166.431935
    "files/nx2019",
    r"28[467][0-9]{3}\.[0-9]{6}/act.ckd/.*",
    act_parse_nx2019v2, // 288327.434641  290003.438003  290004.438004  290809.446516  292406.455024
    "files/nx2019",
    r"2((88)|(9[0-9]))[0-9]{3}\.[0-9]{6}/act.ckd/.*",
    // act_parse_nx2019v2,
    // "files/nx2019",
    // r"29[0-9]{4}\.[0-9]{6}/act.ckd/.*",
    act_parse_nx2020,
    "files/nx2020",
    r".*/act.ckd/.*",
    act_parse_nx2020_china,
    "files/nxChina",
    r".*/act.ckd/.*",
    act_parse_nx2021,
    "files/nx2021",
    r".*/act.ckd/.*",
    act_parse_nx2022,
    "files/nx2022",
    r".*/act.ckd/.*"
);
