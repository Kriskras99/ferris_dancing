#![allow(clippy::needless_pass_by_value)]

use std::path::Path;

use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use ubiart_toolkit::{cooked::act::Actor, utils::UniqueGameId};

fn act_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with_ctx(&data, UniqueGameId::NX2017)?;
    Ok(())
}

fn act_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with_ctx(&data, UniqueGameId::WIIU2017)?;
    Ok(())
}

fn act_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with_ctx(&data, UniqueGameId::NX2018)?;
    Ok(())
}

fn act_parse_nx2019(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with_ctx(&data, UniqueGameId::NX2019)?;
    Ok(())
}

fn act_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with_ctx(&data, UniqueGameId::NX2020)?;
    Ok(())
}

fn act_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with_ctx(&data, UniqueGameId::NX_CHINA)?;
    Ok(())
}

fn act_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with_ctx(&data, UniqueGameId::NX2021)?;
    Ok(())
}

fn act_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    Actor::deserialize_with_ctx(&data, UniqueGameId::NX2022)?;
    Ok(())
}

datatest_stable::harness!(
    act_parse_nx2017,
    "files/nx2017",
    r".*/act.ckd/.*",
    act_parse_wiiu2017,
    "files/wiiu2017",
    r".*/act.ckd/.*",
    act_parse_nx2018,
    "files/nx2018",
    r".*/act.ckd/.*",
    act_parse_nx2019,
    "files/nx2019",
    r".*/act.ckd/.*",
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