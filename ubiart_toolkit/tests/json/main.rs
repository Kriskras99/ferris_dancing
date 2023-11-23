use std::path::Path;

use ubiart_toolkit::{cooked::json, utils::bytes::read_to_vec};

// fn json_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
//     json::open(
//         &input, Game::JustDance2017
//     )
//     .unwrap();
//     Ok(())
// }

// fn json_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
//     json::open(
//         &input, Game::JustDance2018
//     )
//     .unwrap();
//     Ok(())
// }

fn json_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v19(&data, false)?;
    Ok(())
}

fn json_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20(&data, false)?;
    Ok(())
}

fn json_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20c(&data, false)?;
    Ok(())
}

fn json_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v20(&data, false)?;
    Ok(())
}

fn json_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v21(&data, false)?;
    Ok(())
}

fn json_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let data = read_to_vec(input)?;
    let _ = json::parse_v22(&data, false)?;
    Ok(())
}

datatest_stable::harness!(
    // json_parse_nx2017,
    // "tests/json/files/nx2017",
    // r".*\.json\.ckd",
    // json_parse_nx2018,
    // "tests/json/files/nx2018",
    // r".*\.json\.ckd",
    json_parse_nx2019,
    "tests/json/files/nx2019",
    r".*\.json\.ckd",
    json_parse_nx2020,
    "tests/json/files/nx2020",
    r".*\.json\.ckd",
    json_parse_nx2020_china,
    "tests/json/files/nx2020_china",
    r".*\.json\.ckd",
    json_parse_nx2020_japan,
    "tests/json/files/nx2020_japan",
    r".*\.json\.ckd",
    json_parse_nx2021,
    "tests/json/files/nx2021",
    r".*\.json\.ckd",
    json_parse_nx2022,
    "tests/json/files/nx2022",
    r".*\.json\.ckd"
);
