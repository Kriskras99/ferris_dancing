#![allow(clippy::needless_pass_by_value, reason = "Needed by test runner")]

use std::path::Path;

use dotstar_toolkit_utils::bytes::read::BinaryDeserializeExt as _;
use ubiart_toolkit::msm::MovementSpaceMove;

fn msm_parse_wiiu2015(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    MovementSpaceMove::deserialize(&data)?;
    Ok(())
}

fn msm_parse_wiiu2016(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    MovementSpaceMove::deserialize(&data)?;
    Ok(())
}

fn msm_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    MovementSpaceMove::deserialize(&data)?;
    Ok(())
}

fn msm_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    MovementSpaceMove::deserialize(&data)?;
    Ok(())
}

fn msm_parse_win2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    MovementSpaceMove::deserialize(&data)?;
    Ok(())
}

fn msm_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    MovementSpaceMove::deserialize(&data)?;
    Ok(())
}

fn msm_parse_nx2019(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    MovementSpaceMove::deserialize(&data)?;
    Ok(())
}

fn msm_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    MovementSpaceMove::deserialize(&data)?;
    Ok(())
}

fn msm_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    MovementSpaceMove::deserialize(&data)?;
    Ok(())
}

fn msm_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    MovementSpaceMove::deserialize(&data)?;
    Ok(())
}

fn msm_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    MovementSpaceMove::deserialize(&data)?;
    Ok(())
}

datatest_stable::harness!(
    msm_parse_wiiu2015,
    "files/wiiu2015",
    r".*/msm/.*",
    msm_parse_wiiu2016,
    "files/wiiu2016",
    r".*/msm/.*",
    msm_parse_nx2017,
    "files/nx2017",
    r".*/msm/.*",
    msm_parse_win2017,
    "files/win2017",
    r".*/msm/.*",
    msm_parse_wiiu2017,
    "files/wiiu2017",
    r".*/msm/.*",
    msm_parse_nx2018,
    "files/nx2018",
    r".*/msm/.*",
    msm_parse_nx2019,
    "files/nx2019",
    r".*/msm/.*",
    msm_parse_nx2020,
    "files/nx2020",
    r".*/msm/.*",
    msm_parse_nx2020_china,
    "files/nxChina",
    r".*/msm/.*",
    msm_parse_nx2021,
    "files/nx2021",
    r".*/msm/.*",
    msm_parse_nx2022,
    "files/nx2022",
    r".*/msm/.*"
);
