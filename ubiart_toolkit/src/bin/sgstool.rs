use std::path::PathBuf;

use clap::Parser;

use ubiart_toolkit::cooked;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.source;
    let filename = path.file_name().unwrap().to_str().unwrap();

    if filename == "sgscontainer.ckd" {
        let _sgs = match cooked::sgs::open_sgscontainer(&path) {
            Ok(sgs) => sgs,
            Err(e) => panic!("{path:?}: {e:?}"),
        };
    } else {
        let _sgs = match cooked::sgs::open_sgs(&path) {
            Ok(sgs) => sgs,
            Err(e) => panic!("{path:?}: {e:?}"),
        };
    }
}
