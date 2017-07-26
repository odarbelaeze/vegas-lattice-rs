extern crate docopt;
extern crate serde_json;
extern crate vegas_lattice;

use std::fs::File;
use std::io::{self, Read};

use docopt::{ArgvMap, Docopt};
use vegas_lattice::Lattice;


const USAGE: &'static str = "
Vegas lattice.

Usage:
    vlattice check [--compressed] [<input>]
    vlattice (-h | --help)
    vlattice --version

Options:
    -h --help       Show this message.
    --version       Show version and exit.
";


fn check(args: ArgvMap) -> Result<(), Box<std::error::Error>> {
    let mut data = String::new();
    if !args.get_str("<input>").is_empty() {
        let mut file =
            File::open(args.get_str("<input>"))?;
        file.read_to_string(&mut data)?;
    } else {
        io::stdin().read_to_string(&mut data)?;
    };
    let lattice: Lattice = data.parse()?;
    if args.get_bool("--compressed") {
        println!("{}", serde_json::to_string(&lattice).unwrap());
    } else {
        println!("{}", serde_json::to_string_pretty(&lattice).unwrap());
    }
    Ok(())
}


fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|doc| doc.version(Some("0.0.1".to_string())).parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("check") {
        check(args).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });
    } else {
        println!("{:?}", args);
    }

}