use std::path::Path;

use dotstar_toolkit_utils::bytes::read_to_vec;
use ubiart_toolkit::cooked::json;

fn tape_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    json::parse_v17(&data, false)?;
    Ok(())
}

fn tape_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    json::parse_v18(&data, false)?;
    Ok(())
}

fn tape_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    json::parse_v19(&data, false)?;
    Ok(())
}

fn tape_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    json::parse_v20(&data, false)?;
    Ok(())
}

fn tape_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    json::parse_v20c(&data, false)?;
    Ok(())
}

fn tape_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    json::parse_v21(&data, false)?;
    Ok(())
}

fn tape_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    json::parse_v22(&data, false)?;
    Ok(())
}

datatest_stable::harness!(
    tape_parse_nx2017,
    "files/2017",
    r".*/tape.ckd/.*",
    tape_parse_nx2018,
    "files/2018",
    r".*/tape.ckd/.*",
    tape_parse_nx2019,
    "files/2019",
    r".*/tape.ckd/.*",
    tape_parse_nx2020,
    "files/2020",
    r".*/tape.ckd/.*",
    tape_parse_nx2020_china,
    "files/China",
    r".*/tape.ckd/.*",
    tape_parse_nx2021,
    "files/2021",
    r".*/tape.ckd/.*",
    tape_parse_nx2022,
    "files/2022",
    r".*/tape.ckd/.*"
);
