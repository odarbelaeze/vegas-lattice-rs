extern crate docopt;
use docopt::Docopt;


const USAGE: &'static str = "
Vegas lattice.

Usage:
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

    println!("{:?}", args);
}