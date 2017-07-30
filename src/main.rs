extern crate docopt;
extern crate serde_json;
extern crate vegas_lattice;

use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

use docopt::{ArgvMap, Docopt};
use vegas_lattice::{Axis, Lattice};


const USAGE: &'static str = "
Vegas lattice.

Usage:
    vegas-lattice check [<input>]
    vegas-lattice compress [<input>]
    vegas-lattice drop [-x -y -z] [<input>]
    vegas-lattice expand [--x=<x> --y=<y> --z=<z>] [<input>]
    vegas-lattice (-h | --help)
    vegas-lattice --version

Options:
    -h --help       Show this message.
    --version       Show version and exit.
";


fn read(input: &str) -> Result<Lattice, Box<Error>> {
    let mut data = String::new();
    if !input.is_empty() {
        let mut file = File::open(input)?;
        file.read_to_string(&mut data)?;
    } else {
        io::stdin().read_to_string(&mut data)?;
    };
    let lattice: Lattice = data.parse()?;
    Ok(lattice)
}


fn write_compressed(lattice: Lattice) {
    println!("{}", serde_json::to_string(&lattice).unwrap());
}


fn write(lattice: Lattice) {
    println!("{}", serde_json::to_string_pretty(&lattice).unwrap());
}


fn check(args: ArgvMap) -> Result<(), Box<Error>> {
    let lattice = read(args.get_str("<input>"))?;
    write(lattice);
    Ok(())
}


fn compress(args: ArgvMap) -> Result<(), Box<Error>> {
    let lattice = read(args.get_str("<input>"))?;
    write_compressed(lattice);
    Ok(())
}


fn drop(args: ArgvMap) -> Result<(), Box<Error>> {
    let mut lattice = read(args.get_str("<input>"))?;
    if args.get_bool("-x") {
        lattice = lattice.drop(Axis::X);
    }
    if args.get_bool("-y") {
        lattice = lattice.drop(Axis::Y);
    }
    if args.get_bool("-z") {
        lattice = lattice.drop(Axis::Z);
    }
    write(lattice);
    Ok(())
}


fn axis_map<'a>() -> Vec<(&'a str, Axis)> {
    vec![("--x", Axis::X), ("--y", Axis::Y), ("--z", Axis::Z)]
}

fn expand(args: ArgvMap) -> Result<(), Box<Error>> {
    let map = axis_map();
    let mut lattice = read(args.get_str("<input>"))?;
    for (flag, axis) in map.into_iter() {
        let string_value = args.get_str(flag);
        if !string_value.is_empty() {
            let size: usize = string_value.parse()?;
            lattice = lattice.expand_along(axis, size);
        }
    }
    write(lattice);
    Ok(())
}


fn check_error(res: Result<(), Box<Error>>) {
    match res {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
        _ => {},
    }
}


fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|doc| doc.version(Some("0.0.1".to_string())).parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("check") {
        check_error(check(args));
    } else if args.get_bool("compress") {
        check_error(compress(args));
    } else if args.get_bool("drop") {
        check_error(drop(args));
    } else if args.get_bool("expand") {
        check_error(expand(args));
    } else {
        println!("{:?}", args);
    }
}