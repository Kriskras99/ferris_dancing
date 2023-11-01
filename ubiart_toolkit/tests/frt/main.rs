use std::path::Path;

use ubiart_toolkit::{cooked::json, utils::Game};

fn frt_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2017).unwrap();
    Ok(())
}

fn frt_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2018).unwrap();
    Ok(())
}

fn frt_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2019).unwrap();
    Ok(())
}

fn frt_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn frt_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDanceChina).unwrap();
    Ok(())
}

fn frt_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn frt_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2021).unwrap();
    Ok(())
}

fn frt_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2022).unwrap();
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
