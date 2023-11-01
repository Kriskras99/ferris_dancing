use std::path::Path;

use ubiart_toolkit::{cooked::json, utils::Game};

fn dtape_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2017).unwrap();
    Ok(())
}

fn dtape_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2018).unwrap();
    Ok(())
}

fn dtape_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2019).unwrap();
    Ok(())
}

fn dtape_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn dtape_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDanceChina).unwrap();
    Ok(())
}

fn dtape_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn dtape_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2021).unwrap();
    Ok(())
}

fn dtape_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2022).unwrap();
    Ok(())
}

datatest_stable::harness!(
    dtape_parse_nx2017,
    "tests/dtape/files/nx2017",
    r".*\.dtape\.ckd",
    dtape_parse_nx2018,
    "tests/dtape/files/nx2018",
    r".*\.dtape\.ckd",
    dtape_parse_nx2019,
    "tests/dtape/files/nx2019",
    r".*\.dtape\.ckd",
    dtape_parse_nx2020,
    "tests/dtape/files/nx2020",
    r".*\.dtape\.ckd",
    dtape_parse_nx2020_china,
    "tests/dtape/files/nx2020_china",
    r".*\.dtape\.ckd",
    dtape_parse_nx2020_japan,
    "tests/dtape/files/nx2020_japan",
    r".*\.dtape\.ckd",
    dtape_parse_nx2021,
    "tests/dtape/files/nx2021",
    r".*\.dtape\.ckd",
    dtape_parse_nx2022,
    "tests/dtape/files/nx2022",
    r".*\.dtape\.ckd"
);
