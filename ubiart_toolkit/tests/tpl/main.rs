use std::path::Path;

use ubiart_toolkit::cooked::json;

fn tpl_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    json::open_v17(input, false).unwrap();
    Ok(())
}

fn tpl_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    json::open_v18(input, false).unwrap();
    Ok(())
}

fn tpl_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    json::open_v19(input, false).unwrap();
    Ok(())
}

fn tpl_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20(input, false).unwrap();
    Ok(())
}

fn tpl_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20c(input, false).unwrap();
    Ok(())
}

fn tpl_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20(input, false).unwrap();
    Ok(())
}

fn tpl_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    json::open_v21(input, false).unwrap();
    Ok(())
}

fn tpl_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    json::open_v22(input, false).unwrap();
    Ok(())
}

datatest_stable::harness!(
    tpl_parse_nx2017,
    "tests/tpl/files/nx2017",
    r".*\.tpl\.ckd",
    tpl_parse_nx2018,
    "tests/tpl/files/nx2018",
    r".*\.tpl\.ckd",
    tpl_parse_nx2019,
    "tests/tpl/files/nx2019",
    r".*\.tpl\.ckd",
    tpl_parse_nx2020,
    "tests/tpl/files/nx2020",
    r".*\.tpl\.ckd",
    tpl_parse_nx2020_china,
    "tests/tpl/files/nx2020_china",
    r".*\.tpl\.ckd",
    tpl_parse_nx2020_japan,
    "tests/tpl/files/nx2020_japan",
    r".*\.tpl\.ckd",
    tpl_parse_nx2021,
    "tests/tpl/files/nx2021",
    r".*\.tpl\.ckd",
    tpl_parse_nx2022,
    "tests/tpl/files/nx2022",
    r".*\.tpl\.ckd"
);
