use std::path::Path;

use ubiart_toolkit::{cooked::json, utils::Game};

// fn stape_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
//     json::open(
//         &input, Game::JustDance2017
//     )
//     .unwrap();
//     Ok(())
// }

fn stape_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2018).unwrap();
    Ok(())
}

fn stape_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2019).unwrap();
    Ok(())
}

fn stape_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn stape_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDanceChina).unwrap();
    Ok(())
}

fn stape_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2020).unwrap();
    Ok(())
}

fn stape_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2021).unwrap();
    Ok(())
}

fn stape_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    json::open(input, Game::JustDance2022).unwrap();
    Ok(())
}

datatest_stable::harness!(
    // stape_parse_nx2017,
    // "tests/stape/files/nx2017",
    // r".*\.stape\.ckd",
    stape_parse_nx2018,
    "tests/stape/files/nx2018",
    r".*\.stape\.ckd",
    stape_parse_nx2019,
    "tests/stape/files/nx2019",
    r".*\.stape\.ckd",
    stape_parse_nx2020,
    "tests/stape/files/nx2020",
    r".*\.stape\.ckd",
    stape_parse_nx2020_china,
    "tests/stape/files/nx2020_china",
    r".*\.stape\.ckd",
    stape_parse_nx2020_japan,
    "tests/stape/files/nx2020_japan",
    r".*\.stape\.ckd",
    stape_parse_nx2021,
    "tests/stape/files/nx2021",
    r".*\.stape\.ckd",
    stape_parse_nx2022,
    "tests/stape/files/nx2022",
    r".*\.stape\.ckd"
);
