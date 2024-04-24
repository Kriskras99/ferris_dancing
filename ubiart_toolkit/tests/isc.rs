use std::path::Path;

use dotstar_toolkit_utils::bytes::read_to_vec;
use ubiart_toolkit::cooked::isc;

fn isc_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    isc::parse(&data)?;
    Ok(())
}

datatest_stable::harness!(
    isc_parse_nx2017,
    "files/2017",
    r".*/isc.ckd/.*",
    isc_parse_nx2018,
    "files/2018",
    r".*/isc.ckd/.*",
    isc_parse_nx2019,
    "files/2019",
    r".*/isc.ckd/.*",
    isc_parse_nx2020,
    "files/2020",
    r".*/isc.ckd/.*",
    isc_parse_nx2020_china,
    "files/China",
    r".*/isc.ckd/.*",
    isc_parse_nx2021,
    "files/2021",
    r".*/isc.ckd/.*",
    isc_parse_nx2022,
    "files/2022",
    r".*/isc.ckd/.*"
);
