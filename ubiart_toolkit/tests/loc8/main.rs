use std::path::Path;

use ubiart_toolkit::loc8;

fn loc8_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    loc8::open(input).unwrap();
    Ok(())
}

fn loc8_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    loc8::open(input).unwrap();
    Ok(())
}

fn loc8_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    loc8::open(input).unwrap();
    Ok(())
}

fn loc8_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    loc8::open(input).unwrap();
    Ok(())
}

fn loc8_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    loc8::open(input).unwrap();
    Ok(())
}

fn loc8_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    loc8::open(input).unwrap();
    Ok(())
}

fn loc8_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    loc8::open(input).unwrap();
    Ok(())
}

fn loc8_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    loc8::open(input).unwrap();
    Ok(())
}

datatest_stable::harness!(
    loc8_parse_nx2017,
    "tests/loc8/files/nx2017",
    r".*\.loc8",
    loc8_parse_nx2018,
    "tests/loc8/files/nx2018",
    r".*\.loc8",
    loc8_parse_nx2019,
    "tests/loc8/files/nx2019",
    r".*\.loc8",
    loc8_parse_nx2020,
    "tests/loc8/files/nx2020",
    r".*\.loc8",
    loc8_parse_nx2020_china,
    "tests/loc8/files/nx2020_china",
    r".*\.loc8",
    loc8_parse_nx2020_japan,
    "tests/loc8/files/nx2020_japan",
    r".*\.loc8",
    loc8_parse_nx2021,
    "tests/loc8/files/nx2021",
    r".*\.loc8",
    loc8_parse_nx2022,
    "tests/loc8/files/nx2022",
    r".*\.loc8"
);
