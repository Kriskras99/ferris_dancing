use std::path::Path;

use ubiart_toolkit::cooked::json;

fn frt_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    json::open_v17(input, false).unwrap();
    Ok(())
}

fn frt_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    json::open_v18(input, false).unwrap();
    Ok(())
}

fn frt_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    json::open_v19(input, false).unwrap();
    Ok(())
}

fn frt_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20(input, false).unwrap();
    Ok(())
}

fn frt_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20c(input, false).unwrap();
    Ok(())
}

fn frt_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20(input, false).unwrap();
    Ok(())
}

fn frt_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    json::open_v21(input, false).unwrap();
    Ok(())
}

fn frt_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    json::open_v22(input, false).unwrap();
    Ok(())
}

datatest_stable::harness!(
    frt_parse_nx2017,
    "tests/frt/files/nx2017",
    r".*\.frt\.ckd",
    frt_parse_nx2018,
    "tests/frt/files/nx2018",
    r".*\.frt\.ckd",
    frt_parse_nx2019,
    "tests/frt/files/nx2019",
    r".*\.frt\.ckd",
    frt_parse_nx2020,
    "tests/frt/files/nx2020",
    r".*\.frt\.ckd",
    frt_parse_nx2020_china,
    "tests/frt/files/nx2020_china",
    r".*\.frt\.ckd",
    frt_parse_nx2020_japan,
    "tests/frt/files/nx2020_japan",
    r".*\.frt\.ckd",
    frt_parse_nx2021,
    "tests/frt/files/nx2021",
    r".*\.frt\.ckd",
    frt_parse_nx2022,
    "tests/frt/files/nx2022",
    r".*\.frt\.ckd"
);
