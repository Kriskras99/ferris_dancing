use std::{fs::File, path::Path};

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
    "files/2017",
    r".*/msm/.*",
    msm_parse_nx2018,
    "files/2018",
    r".*/msm/.*",
    msm_parse_nx2019,
    "files/2019",
    r".*/msm/.*",
    msm_parse_nx2020,
    "files/2020",
    r".*/msm/.*",
    msm_parse_nx2020_china,
    "files/China",
    r".*/msm/.*",
    msm_parse_nx2021,
    "files/2021",
    r".*/msm/.*",
    msm_parse_nx2022,
    "files/2022",
    r".*/msm/.*"
);
