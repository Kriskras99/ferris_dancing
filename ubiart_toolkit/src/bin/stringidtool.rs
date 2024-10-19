use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, IsTerminal},
    path::PathBuf,
};

use clap::Parser;
use ubiart_toolkit::utils::string_id;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    file: Option<PathBuf>,
    string: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let is_pipe = stdin().is_terminal();
    let have_file = cli.file.is_some();
    let have_string = cli.string.is_some();
    let total = u8::from(is_pipe) + u8::from(have_file) + u8::from(have_string);

    if total != 1 {
        panic!("You need to do only one of the following: use --file, specify a string, or pipe to stdin!");
    }

    if let Some(file) = cli.file {
        let file = File::open(file).unwrap();
        let bufread = BufReader::new(file);
        for line in bufread.lines() {
            let line = line.unwrap();
            println!("0x{:08x}: {}", string_id(&line), line);
        }
    } else if let Some(string) = cli.string {
        println!("0x{:08x}: {}", string_id(&string), string);
    } else {
        let bufread = BufReader::new(stdin());
        for line in bufread.lines() {
            let line = line.unwrap();
            println!("0x{:08x}: {}", string_id(&line), line);
        }
    }
}
