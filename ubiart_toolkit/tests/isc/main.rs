use std::path::Path;

use ubiart_toolkit::{cooked::isc, utils::bytes::read_to_vec};

fn isc_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = isc::parse(&data)?;
    Ok(())
}

datatest_stable::harness!(
    isc_parse_nx2017,
    "tests/isc/files/nx2017",
    r".*\.isc\.ckd",
    isc_parse_nx2018,
    "tests/isc/files/nx2018",
    r".*\.isc\.ckd",
    isc_parse_nx2019,
    "tests/isc/files/nx2019",
    r".*\.isc\.ckd",
    isc_parse_nx2020,
    "tests/isc/files/nx2020",
    r".*\.isc\.ckd",
    isc_parse_nx2020_china,
    "tests/isc/files/nx2020_china",
    r".*\.isc\.ckd",
    isc_parse_nx2020_japan,
    "tests/isc/files/nx2020_japan",
    r".*\.isc\.ckd",
    isc_parse_nx2021,
    "tests/isc/files/nx2021",
    r".*\.isc\.ckd",
    isc_parse_nx2022,
    "tests/isc/files/nx2022",
    r".*\.isc\.ckd"
);
