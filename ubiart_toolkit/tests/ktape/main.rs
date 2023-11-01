use std::path::Path;

use ubiart_toolkit::{cooked::json, utils::Game};

fn ktape_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2017).unwrap();
    Ok(())
}

fn ktape_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2018).unwrap();
    Ok(())
}

fn ktape_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2019).unwrap();
    Ok(())
}

fn ktape_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn ktape_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDanceChina).unwrap();
    Ok(())
}

fn ktape_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn ktape_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2021).unwrap();
    Ok(())
}

fn ktape_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2022).unwrap();
    Ok(())
}

datatest_stable::harness!(
    ktape_parse_nx2017,
    "tests/ktape/files/nx2017",
    r".*\.ktape\.ckd",
    ktape_parse_nx2018,
    "tests/ktape/files/nx2018",
    r".*\.ktape\.ckd",
    ktape_parse_nx2019,
    "tests/ktape/files/nx2019",
    r".*\.ktape\.ckd",
    ktape_parse_nx2020,
    "tests/ktape/files/nx2020",
    r".*\.ktape\.ckd",
    ktape_parse_nx2020_china,
    "tests/ktape/files/nx2020_china",
    r".*\.ktape\.ckd",
    ktape_parse_nx2020_japan,
    "tests/ktape/files/nx2020_japan",
    r".*\.ktape\.ckd",
    ktape_parse_nx2021,
    "tests/ktape/files/nx2021",
    r".*\.ktape\.ckd",
    ktape_parse_nx2022,
    "tests/ktape/files/nx2022",
    r".*\.ktape\.ckd"
);
