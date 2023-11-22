use std::path::Path;

use ubiart_toolkit::cooked::json;

fn tape_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    json::open_v17(input, false).unwrap();
    Ok(())
}

fn tape_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    json::open_v18(input, false).unwrap();
    Ok(())
}

fn tape_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    json::open_v19(input, false).unwrap();
    Ok(())
}

fn tape_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20(input, false).unwrap();
    Ok(())
}

fn tape_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20c(input, false).unwrap();
    Ok(())
}

fn tape_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20(input, false).unwrap();
    Ok(())
}

fn tape_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    json::open_v21(input, false).unwrap();
    Ok(())
}

fn tape_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    json::open_v22(input, false).unwrap();
    Ok(())
}

datatest_stable::harness!(
    tape_parse_nx2017,
    "tests/tape/files/nx2017",
    r".*\.tape\.ckd",
    tape_parse_nx2018,
    "tests/tape/files/nx2018",
    r".*\.tape\.ckd",
    tape_parse_nx2019,
    "tests/tape/files/nx2019",
    r".*\.tape\.ckd",
    tape_parse_nx2020,
    "tests/tape/files/nx2020",
    r".*\.tape\.ckd",
    tape_parse_nx2020_china,
    "tests/tape/files/nx2020_china",
    r".*\.tape\.ckd",
    tape_parse_nx2020_japan,
    "tests/tape/files/nx2020_japan",
    r".*\.tape\.ckd",
    tape_parse_nx2021,
    "tests/tape/files/nx2021",
    r".*\.tape\.ckd",
    tape_parse_nx2022,
    "tests/tape/files/nx2022",
    r".*\.tape\.ckd"
);
