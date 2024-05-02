use std::path::Path;

use ubiart_toolkit::cooked::isc;

fn isc_parse_nx2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_wiiu2017(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2018(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2019(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2020(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2020_china(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2021(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    isc::parse(&data)?;
    Ok(())
}

fn isc_parse_nx2022(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    isc::parse(&data)?;
    Ok(())
}

datatest_stable::harness!(
    isc_parse_nx2017,
    "files/nx2017",
    r".*/isc.ckd/.*",
    isc_parse_wiiu2017,
    "files/wiiu2017",
    r".*/isc.ckd/.*",
    isc_parse_nx2018,
    "files/nx2018",
    r".*/isc.ckd/.*",
    isc_parse_nx2019,
    "files/nx2019",
    r".*/isc.ckd/.*",
    isc_parse_nx2020,
    "files/nx2020",
    r".*/isc.ckd/.*",
    isc_parse_nx2020_china,
    "files/nxChina",
    r".*/isc.ckd/.*",
    isc_parse_nx2021,
    "files/nx2021",
    r".*/isc.ckd/.*",
    isc_parse_nx2022,
    "files/nx2022",
    r".*/isc.ckd/.*"
);
