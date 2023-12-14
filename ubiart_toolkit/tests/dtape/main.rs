use std::path::Path;

use dotstar_toolkit_utils::bytes::read_to_vec;
use ubiart_toolkit::cooked::json;

fn dtape_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v17(&data, false)?;
    Ok(())
}

fn dtape_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v18(&data, false)?;
    Ok(())
}

fn dtape_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v19(&data, false)?;
    Ok(())
}

fn dtape_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20(&data, false)?;
    Ok(())
}

fn dtape_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20c(&data, false)?;
    Ok(())
}

fn dtape_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20(&data, false)?;
    Ok(())
}

fn dtape_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v21(&data, false)?;
    Ok(())
}

fn dtape_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v22(&data, false)?;
    Ok(())
}

datatest_stable::harness!(
    dtape_parse_nx2017,
    "tests/dtape/files/nx2017",
    r".*\.dtape\.ckd",
    dtape_parse_nx2018,
    "tests/dtape/files/nx2018",
    r".*\.dtape\.ckd",
    dtape_parse_nx2019,
    "tests/dtape/files/nx2019",
    r".*\.dtape\.ckd",
    dtape_parse_nx2020,
    "tests/dtape/files/nx2020",
    r".*\.dtape\.ckd",
    dtape_parse_nx2020_china,
    "tests/dtape/files/nx2020_china",
    r".*\.dtape\.ckd",
    dtape_parse_nx2020_japan,
    "tests/dtape/files/nx2020_japan",
    r".*\.dtape\.ckd",
    dtape_parse_nx2021,
    "tests/dtape/files/nx2021",
    r".*\.dtape\.ckd",
    dtape_parse_nx2022,
    "tests/dtape/files/nx2022",
    r".*\.dtape\.ckd"
);
