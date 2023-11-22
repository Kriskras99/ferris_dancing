use std::path::Path;

use ubiart_toolkit::cooked::json;

fn msh_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    json::open_v17(input, false).unwrap();
    Ok(())
}

fn msh_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    json::open_v18(input, false).unwrap();
    Ok(())
}

fn msh_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    json::open_v19(input, false).unwrap();
    Ok(())
}

fn msh_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20(input, false).unwrap();
    Ok(())
}

fn msh_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20c(input, false).unwrap();
    Ok(())
}

fn msh_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    json::open_v20(input, false).unwrap();
    Ok(())
}

fn msh_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    json::open_v21(input, false).unwrap();
    Ok(())
}

fn msh_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    json::open_v22(input, false).unwrap();
    Ok(())
}

datatest_stable::harness!(
    msh_parse_nx2017,
    "tests/msh/files/nx2017",
    r".*\.msh\.ckd",
    msh_parse_nx2018,
    "tests/msh/files/nx2018",
    r".*\.msh\.ckd",
    msh_parse_nx2019,
    "tests/msh/files/nx2019",
    r".*\.msh\.ckd",
    msh_parse_nx2020,
    "tests/msh/files/nx2020",
    r".*\.msh\.ckd",
    msh_parse_nx2020_china,
    "tests/msh/files/nx2020_china",
    r".*\.msh\.ckd",
    msh_parse_nx2020_japan,
    "tests/msh/files/nx2020_japan",
    r".*\.msh\.ckd",
    msh_parse_nx2021,
    "tests/msh/files/nx2021",
    r".*\.msh\.ckd",
    msh_parse_nx2022,
    "tests/msh/files/nx2022",
    r".*\.msh\.ckd"
);
