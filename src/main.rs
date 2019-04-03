extern crate docopt;
extern crate serde_json;
extern crate vegas_lattice;

use std::error::Error;
use std::fs::File;
use std::io::{stdin, Read};
use std::path::Path;

use docopt::{ArgvMap, Docopt};
use vegas_lattice::{io, Alloy, Axis, Lattice, Mask};

const USAGE: &str = "
Vegas lattice.

Usage:
    vegas-lattice check [<input>]
    vegas-lattice pretty [<input>]
    vegas-lattice drop [-x -y -z] [<input>]
    vegas-lattice expand [--x=<x> --y=<y> --z=<z>] [<input>]
    vegas-lattice alloy <source> (<target> <ratio>)... [<input>]
    vegas-lattice mask [--ppu=<ppu>] <mask> [<input>]
    vegas-lattice into (xyz|tsv) [<input>]
    vegas-lattice (-h | --help)
    vegas-lattice --version

Options:
    -p --ppu=<ppu>  Pixels per unit [default: 10].
    -h --help       Show this message.
    --version       Show version and exit.
";

fn read(input: &str) -> Result<Lattice, Box<Error>> {
    let mut data = String::new();
    if !input.is_empty() {
        let mut file = File::open(input)?;
        file.read_to_string(&mut data)?;
    } else {
        stdin().read_to_string(&mut data)?;
    };
    let lattice: Lattice = data.parse()?;
    Ok(lattice)
}

fn write(lattice: Lattice) {
    println!("{}", serde_json::to_string(&lattice).unwrap());
}

fn write_pretty(lattice: Lattice) {
    println!("{}", io::to_string_lattice(&lattice).unwrap());
}

fn check_error(res: Result<(), Box<Error>>) {
    if let Err(e) = res {
        eprintln!("Error: {}", e.description());
        if let Some(cause) = e.cause() {
            eprintln!("Cause: {}", cause);
        }
    }
}

// Commands over here

fn check(args: ArgvMap) -> Result<(), Box<Error>> {
    let lattice = read(args.get_str("<input>"))?;
    write(lattice);
    Ok(())
}

fn pretty(args: ArgvMap) -> Result<(), Box<Error>> {
    let lattice = read(args.get_str("<input>"))?;
    write_pretty(lattice);
    Ok(())
}

fn drop(args: ArgvMap) -> Result<(), Box<Error>> {
    let mut lattice = read(args.get_str("<input>"))?;
    for (key, axis) in Axis::map(Some("-".to_string())) {
        if args.get_bool(&key) {
            lattice = lattice.drop(axis);
        }
    }
    write(lattice);
    Ok(())
}

fn expand(args: ArgvMap) -> Result<(), Box<Error>> {
    let map = Axis::map(Some("--".to_string()));
    let mut lattice = read(args.get_str("<input>"))?;
    for (flag, axis) in map.into_iter() {
        let string_value = args.get_str(&flag);
        if !string_value.is_empty() {
            let size: usize = string_value.parse()?;
            lattice = lattice.expand_along(axis, size);
        }
    }
    write(lattice);
    Ok(())
}

fn alloy(args: ArgvMap) -> Result<(), Box<Error>> {
    let source = args.get_str("<source>");
    let kinds = args.get_vec("<target>");
    let ratios = args
        .get_vec("<ratio>")
        .into_iter()
        .map(|r| r.parse())
        .collect::<Result<Vec<u32>, _>>()?;
    let alloy = Alloy::new(kinds, ratios);
    let mut lattice = read(args.get_str("<input>"))?;
    lattice = lattice.alloy_sites(source, alloy);
    write(lattice);
    Ok(())
}

fn mask(args: ArgvMap) -> Result<(), Box<Error>> {
    let path = args.get_str("<mask>");
    let ppu: f64 = args.get_str("--ppu").parse()?;
    let mask = Mask::new(&Path::new(path), ppu)?;
    let mut lattice = read(args.get_str("<input>"))?;
    lattice = lattice.apply_mask(mask);
    write(lattice);
    Ok(())
}

fn into(args: ArgvMap) -> Result<(), Box<Error>> {
    let lattice = read(args.get_str("<input>"))?;
    if args.get_bool("tsv") {
        for site in lattice.sites().iter() {
            let (x, y, z) = site.position();
            println!("{}\t{}\t{}\t{}", x, y, z, site.kind())
        }
    } else if args.get_bool("xyz") {
        println!("{}\n", lattice.sites().len());
        for site in lattice.sites().iter() {
            let (x, y, z) = site.position();
            println!("{} {} {} {}", site.kind(), x, y, z)
        }
    }
    Ok(())
}

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|doc| doc.version(Some("0.0.1".to_string())).parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("check") {
        check_error(check(args));
    } else if args.get_bool("pretty") {
        check_error(pretty(args));
    } else if args.get_bool("drop") {
        check_error(drop(args));
    } else if args.get_bool("alloy") {
        check_error(alloy(args));
    } else if args.get_bool("expand") {
        check_error(expand(args));
    } else if args.get_bool("mask") {
        check_error(mask(args));
    } else if args.get_bool("into") {
        check_error(into(args));
    } else {
        println!("{:?}", args);
    }
}
