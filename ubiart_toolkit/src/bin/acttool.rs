use std::{fs::File, path::PathBuf, rc::Rc};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use ubiart_toolkit::{cooked::act::Actor, utils::plumbing::{Nx2017, Nx2018, Nx2019, Nx2020, Nx2021, Nx2022, NxChina}};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    game: String,
    source: PathBuf,
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn main() {
    let cli = Cli::parse();
    let file = Rc::new(File::open(&cli.source).unwrap());

    match cli.game.to_lowercase().as_str() {
        "2022" => println!("{:#?}", Actor::<Nx2022>::deserialize(&file).unwrap()),
        "2021" => println!("{:#?}", Actor::<Nx2021>::deserialize(&file).unwrap()),
        "2020" => println!("{:#?}", Actor::<Nx2020>::deserialize(&file).unwrap()),
        "china" => println!("{:#?}", Actor::<NxChina>::deserialize(&file).unwrap()),
        "2019" => println!("{:#?}", Actor::<Nx2019>::deserialize(&file).unwrap()),
        "2018" => println!("{:#?}", Actor::<Nx2018>::deserialize(&file).unwrap()),
        "2017" => println!("{:#?}", Actor::<Nx2017>::deserialize(&file).unwrap()),
        _ => panic!("Unrecognized game version: {}", cli.game),
    }
}
