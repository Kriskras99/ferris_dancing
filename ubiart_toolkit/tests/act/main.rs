use std::{fs::File, path::Path, rc::Rc};

use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use ubiart_toolkit::{cooked::act::Actor, utils::plumbing::{Nx2017, Nx2018, Nx2019, Nx2020, Nx2021, Nx2022, NxChina}};

fn act_parse_nx2017(input: &Path) -> datatest_stable::Result<()> {
    let file = Rc::new(File::open(input)?);
    Actor::<Nx2017>::deserialize(&file)?;
    Ok(())
}

fn act_parse_nx2018(input: &Path) -> datatest_stable::Result<()> {
    let file = Rc::new(File::open(input)?);
    Actor::<Nx2018>::deserialize(&file)?;
    Ok(())
}

fn act_parse_nx2019(input: &Path) -> datatest_stable::Result<()> {
    let file = Rc::new(File::open(input)?);
    Actor::<Nx2019>::deserialize(&file)?;
    Ok(())
}

fn act_parse_nx2020(input: &Path) -> datatest_stable::Result<()> {
    let file = Rc::new(File::open(input)?);
    Actor::<Nx2020>::deserialize(&file)?;
    Ok(())
}

fn act_parse_nx2020_china(input: &Path) -> datatest_stable::Result<()> {
    let file = Rc::new(File::open(input)?);
    Actor::<NxChina>::deserialize(&file)?;
    Ok(())
}

fn act_parse_nx2020_japan(input: &Path) -> datatest_stable::Result<()> {
    let file = Rc::new(File::open(input)?);
    Actor::<Nx2020>::deserialize(&file)?;
    Ok(())
}

fn act_parse_nx2021(input: &Path) -> datatest_stable::Result<()> {
    let file = Rc::new(File::open(input)?);
    Actor::<Nx2021>::deserialize(&file)?;
    Ok(())
}

fn act_parse_nx2022(input: &Path) -> datatest_stable::Result<()> {
    let file = Rc::new(File::open(input)?);
    Actor::<Nx2022>::deserialize(&file)?;
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
