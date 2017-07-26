extern crate docopt;
extern crate serde_json;
extern crate vegas_lattice;

use std::fs::File;
use std::io::{self, Read};

use docopt::Docopt;
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


fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|doc| doc.version(Some("0.0.1".to_string())).parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("check") {
        // Check program
        let mut data = String::new();
        if !args.get_str("<input>").is_empty() {
            let mut file =
                File::open(args.get_str("<input>")).unwrap_or_else(|e| {
                                                                     println!("error: {:?}", e);
                                                                     std::process::exit(1);
                                                                 });
            file.read_to_string(&mut data).unwrap_or_else(|e| {
                                                              println!("error: {:?}", e);
                                                              std::process::exit(1);
                                                          });
        } else {
            io::stdin().read_to_string(&mut data).unwrap_or_else(|e| {
                                                                     println!("error: {:?}", e);
                                                                     std::process::exit(1);
                                                                 });
        };
        let lattice: Lattice = data.parse().unwrap_or_else(|e| {
                                                               println!("error: {:?}", e);
                                                               std::process::exit(1);
                                                           });
        if args.get_bool("--compressed") {
            println!("{}", serde_json::to_string(&lattice).unwrap());
        } else {
            println!("{}", serde_json::to_string_pretty(&lattice).unwrap());
        }
    } else {
        println!("{:?}", args);
    }

}