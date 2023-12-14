use std::path::Path;

use dotstar_toolkit_utils::bytes::read_to_vec;
use ubiart_toolkit::{cooked::act, utils::Game};

fn act_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = act::parse(&data, Game::JustDance2017)?;
    Ok(())
}

fn act_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = act::parse(&data, Game::JustDance2018)?;
    Ok(())
}

fn act_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = act::parse(&data, Game::JustDance2019)?;
    Ok(())
}

fn act_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = act::parse(&data, Game::JustDance2020)?;
    Ok(())
}

fn act_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = act::parse(&data, Game::JustDanceChina)?;
    Ok(())
}

fn act_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = act::parse(&data, Game::JustDance2020)?;
    Ok(())
}

fn act_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = act::parse(&data, Game::JustDance2021)?;
    Ok(())
}

fn act_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = act::parse(&data, Game::JustDance2022)?;
    Ok(())
}

datatest_stable::harness!(
    act_parse_nx2017,
    "tests/act/files/nx2017",
    r".*\.act\.ckd",
    act_parse_nx2018,
    "tests/act/files/nx2018",
    r".*\.act\.ckd",
    act_parse_nx2019,
    "tests/act/files/nx2019",
    r".*\.act\.ckd",
    act_parse_nx2020,
    "tests/act/files/nx2020",
    r".*\.act\.ckd",
    act_parse_nx2020_china,
    "tests/act/files/nx2020_china",
    r".*\.act\.ckd",
    act_parse_nx2020_japan,
    "tests/act/files/nx2020_japan",
    r".*\.act\.ckd",
    act_parse_nx2021,
    "tests/act/files/nx2021",
    r".*\.act\.ckd",
    act_parse_nx2022,
    "tests/act/files/nx2022",
    r".*\.act\.ckd"
);
