use std::path::Path;

use ubiart_toolkit::{cooked::json, utils::Game};

fn msh_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2017).unwrap();
    Ok(())
}

fn msh_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2018).unwrap();
    Ok(())
}

fn msh_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2019).unwrap();
    Ok(())
}

fn msh_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn msh_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDanceChina).unwrap();
    Ok(())
}

fn msh_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn msh_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2021).unwrap();
    Ok(())
}

fn msh_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2022).unwrap();
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
