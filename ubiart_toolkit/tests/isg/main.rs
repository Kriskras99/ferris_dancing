use std::path::Path;

use ubiart_toolkit::cooked::json;

fn isg_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    json::open_v17(input, false).unwrap();
    Ok(())
}

fn isg_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    json::open_v18(input, false).unwrap();
    Ok(())
}

fn isg_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    json::open_v19(input, false).unwrap();
    Ok(())
}

fn isg_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20(input, false).unwrap();
    Ok(())
}

fn isg_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20c(input, false).unwrap();
    Ok(())
}

fn isg_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20(input, false).unwrap();
    Ok(())
}

fn isg_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    json::open_v21(input, false).unwrap();
    Ok(())
}

fn isg_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    json::open_v22(input, false).unwrap();
    Ok(())
}

datatest_stable::harness!(
    isg_parse_nx2017,
    "tests/isg/files/nx2017",
    r".*\.isg\.ckd",
    isg_parse_nx2018,
    "tests/isg/files/nx2018",
    r".*\.isg\.ckd",
    isg_parse_nx2019,
    "tests/isg/files/nx2019",
    r".*\.isg\.ckd",
    isg_parse_nx2020,
    "tests/isg/files/nx2020",
    r".*\.isg\.ckd",
    isg_parse_nx2020_china,
    "tests/isg/files/nx2020_china",
    r".*\.isg\.ckd",
    isg_parse_nx2020_japan,
    "tests/isg/files/nx2020_japan",
    r".*\.isg\.ckd",
    isg_parse_nx2021,
    "tests/isg/files/nx2021",
    r".*\.isg\.ckd",
    isg_parse_nx2022,
    "tests/isg/files/nx2022",
    r".*\.isg\.ckd"
);
