use std::{fs::File, path::Path};

use ubiart_toolkit::{
    cooked::act,
    utils::{Game, Platform, UniqueGameId},
};

fn act_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    let gp = UniqueGameId {
        game: Game::JustDance2017,
        platform: Platform::Nx,
        id: 0,
    };
    act::parse(&file, &mut 0, gp)?;
    Ok(())
}

fn act_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    let gp = UniqueGameId {
        game: Game::JustDance2018,
        platform: Platform::Nx,
        id: 0,
    };
    act::parse(&file, &mut 0, gp)?;
    Ok(())
}

fn act_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    let gp = UniqueGameId {
        game: Game::JustDance2019,
        platform: Platform::Nx,
        id: 0,
    };
    act::parse(&file, &mut 0, gp)?;
    Ok(())
}

fn act_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    let gp = UniqueGameId {
        game: Game::JustDance2020,
        platform: Platform::Nx,
        id: 0,
    };
    act::parse(&file, &mut 0, gp)?;
    Ok(())
}

fn act_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    let gp = UniqueGameId {
        game: Game::JustDanceChina,
        platform: Platform::Nx,
        id: 0,
    };
    act::parse(&file, &mut 0, gp)?;
    Ok(())
}

fn act_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    let gp = UniqueGameId {
        game: Game::JustDance2021,
        platform: Platform::Nx,
        id: 0,
    };
    act::parse(&file, &mut 0, gp)?;
    Ok(())
}

fn act_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let file = File::open(input)?;
    let gp = UniqueGameId {
        game: Game::JustDance2022,
        platform: Platform::Nx,
        id: 0,
    };
    act::parse(&file, &mut 0, gp)?;
    Ok(())
}

datatest_stable::harness!(
    act_parse_nx2017,
    "files/2017",
    r".*/act.ckd/.*",
    act_parse_nx2018,
    "files/2018",
    r".*/act.ckd/.*",
    act_parse_nx2019,
    "files/2019",
    r".*/act.ckd/.*",
    act_parse_nx2020,
    "files/2020",
    r".*/act.ckd/.*",
    act_parse_nx2020_china,
    "files/China",
    r".*/act.ckd/.*",
    act_parse_nx2021,
    "files/2021",
    r".*/act.ckd/.*",
    act_parse_nx2022,
    "files/2022",
    r".*/act.ckd/.*"
);
