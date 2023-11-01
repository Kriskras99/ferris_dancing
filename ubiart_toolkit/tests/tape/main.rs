use std::path::Path;

use ubiart_toolkit::{cooked::json, utils::Game};

fn tape_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2017).unwrap();
    Ok(())
}

fn tape_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2018).unwrap();
    Ok(())
}

fn tape_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2019).unwrap();
    Ok(())
}

fn tape_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn tape_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDanceChina).unwrap();
    Ok(())
}

fn tape_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn tape_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2021).unwrap();
    Ok(())
}

fn tape_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2022).unwrap();
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
