use std::path::Path;

use dotstar_toolkit_utils::bytes::read_to_vec;
use ubiart_toolkit::cooked::sgs;

fn sgs_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    sgs::parse(&data)?;
    Ok(())
}

fn sgs_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    sgs::parse(&data)?;
    Ok(())
}

fn sgs_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    sgs::parse(&data)?;
    Ok(())
}

fn sgs_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    sgs::parse(&data)?;
    Ok(())
}

fn sgs_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    sgs::parse(&data)?;
    Ok(())
}

fn sgs_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    sgs::parse(&data)?;
    Ok(())
}

fn sgs_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    sgs::parse(&data)?;
    Ok(())
}

datatest_stable::harness!(
    sgs_parse_nx2017,
    "files/2017",
    r".*/sgs.ckd/.*",
    sgs_parse_nx2018,
    "files/2018",
    r".*/sgs.ckd/.*",
    sgs_parse_nx2019,
    "files/2019",
    r".*/sgs.ckd/.*",
    sgs_parse_nx2020,
    "files/2020",
    r".*/sgs.ckd/.*",
    sgs_parse_nx2020_china,
    "files/China",
    r".*/sgs.ckd/.*",
    sgs_parse_nx2021,
    "files/2021",
    r".*/sgs.ckd/.*",
    sgs_parse_nx2022,
    "files/2022",
    r".*/sgs.ckd/.*"
);
