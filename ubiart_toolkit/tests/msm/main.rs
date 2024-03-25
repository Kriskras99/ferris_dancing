use std::{fs::File, path::Path, rc::Rc};

use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use ubiart_toolkit::msm::MovementSpaceMove;

fn msm_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    MovementSpaceMove::deserialize(&file)?;
    Ok(())
}

fn msm_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    MovementSpaceMove::deserialize(&file)?;
    Ok(())
}

fn msm_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    MovementSpaceMove::deserialize(&file)?;
    Ok(())
}

fn msm_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    MovementSpaceMove::deserialize(&file)?;
    Ok(())
}

fn msm_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    MovementSpaceMove::deserialize(&file)?;
    Ok(())
}

fn msm_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    MovementSpaceMove::deserialize(&file)?;
    Ok(())
}

fn msm_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    MovementSpaceMove::deserialize(&file)?;
    Ok(())
}

fn msm_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    MovementSpaceMove::deserialize(&file)?;
    Ok(())
}

datatest_stable::harness!(
    msm_parse_nx2017,
    "tests/msm/files/nx2017",
    r".*\.msm",
    msm_parse_nx2018,
    "tests/msm/files/nx2018",
    r".*\.msm",
    msm_parse_nx2019,
    "tests/msm/files/nx2019",
    r".*\.msm",
    msm_parse_nx2020,
    "tests/msm/files/nx2020",
    r".*\.msm",
    msm_parse_nx2020_china,
    "tests/msm/files/nx2020_china",
    r".*\.msm",
    msm_parse_nx2020_japan,
    "tests/msm/files/nx2020_japan",
    r".*\.msm",
    msm_parse_nx2021,
    "tests/msm/files/nx2021",
    r".*\.msm",
    msm_parse_nx2022,
    "tests/msm/files/nx2022",
    r".*\.msm"
);
