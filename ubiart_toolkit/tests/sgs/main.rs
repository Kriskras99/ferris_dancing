use std::path::Path;

use ubiart_toolkit::cooked::sgs;

fn sgs_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    sgs::open(input).unwrap();
    Ok(())
}

fn sgs_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    sgs::open(input).unwrap();
    Ok(())
}

fn sgs_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    sgs::open(input).unwrap();
    Ok(())
}

fn sgs_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    sgs::open(input).unwrap();
    Ok(())
}

fn sgs_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    sgs::open(input).unwrap();
    Ok(())
}

fn sgs_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    sgs::open(input).unwrap();
    Ok(())
}

fn sgs_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    sgs::open(input).unwrap();
    Ok(())
}

fn sgs_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    sgs::open(input).unwrap();
    Ok(())
}

datatest_stable::harness!(
    sgs_parse_nx2017,
    "tests/sgs/files/nx2017",
    r".*\.sgs\.ckd",
    sgs_parse_nx2018,
    "tests/sgs/files/nx2018",
    r".*\.sgs\.ckd",
    sgs_parse_nx2019,
    "tests/sgs/files/nx2019",
    r".*\.sgs\.ckd",
    sgs_parse_nx2020,
    "tests/sgs/files/nx2020",
    r".*\.sgs\.ckd",
    sgs_parse_nx2020_china,
    "tests/sgs/files/nx2020_china",
    r".*\.sgs\.ckd",
    sgs_parse_nx2020_japan,
    "tests/sgs/files/nx2020_japan",
    r".*\.sgs\.ckd",
    sgs_parse_nx2021,
    "tests/sgs/files/nx2021",
    r".*\.sgs\.ckd",
    sgs_parse_nx2022,
    "tests/sgs/files/nx2022",
    r".*\.sgs\.ckd"
);
