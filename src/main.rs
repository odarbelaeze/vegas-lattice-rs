extern crate docopt;
extern crate serde;
extern crate serde_json;
extern crate vegas_lattice;

use std::error::Error;
use std::fs::File;
use std::io::{stdin, Read};
use std::path::Path;

use docopt::Docopt;
use serde::Deserialize;
use vegas_lattice::{io, Alloy, Axis, Lattice, Mask};

const USAGE: &str = "
Vegas lattice.

Usage:
    vegas-lattice check [<input>]
    vegas-lattice pretty [<input>]
    vegas-lattice drop [-x -y -z] [<input>]
    vegas-lattice expand [--along-x=<x> --along-y=<y> --along-z=<z>] [<input>]
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

#[derive(Debug, Deserialize)]
struct Args {
    arg_input: String,
    arg_source: String,
    arg_target: Vec<String>,
    arg_ratio: Vec<u32>,
    arg_mask: String,
    flag_along_x: Option<usize>,
    flag_along_y: Option<usize>,
    flag_along_z: Option<usize>,
    flag_x: bool,
    flag_y: bool,
    flag_z: bool,
    flag_ppu: f64,
    cmd_check: bool,
    cmd_pretty: bool,
    cmd_drop: bool,
    cmd_expand: bool,
    cmd_alloy: bool,
    cmd_mask: bool,
    cmd_into: bool,
    cmd_xyz: bool,
    cmd_tsv: bool,
}

fn read(input: &str) -> Result<Lattice, Box<dyn Error>> {
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

fn check_error(res: Result<(), Box<dyn Error>>) {
    if let Err(e) = res {
        eprintln!("Error: {}", e);
        if let Some(source) = e.source() {
            eprintln!("Cause: {}", source);
        }
    }
}

// Commands over here

fn check(args: Args) -> Result<(), Box<dyn Error>> {
    let lattice = read(&args.arg_input)?;
    write(lattice);
    Ok(())
}

fn pretty(args: Args) -> Result<(), Box<dyn Error>> {
    let lattice = read(&args.arg_input)?;
    write_pretty(lattice);
    Ok(())
}

fn drop(args: Args) -> Result<(), Box<dyn Error>> {
    let mut lattice = read(&args.arg_input)?;
    if args.flag_x {
        lattice = lattice.drop(Axis::X);
    }
    if args.flag_y {
        lattice = lattice.drop(Axis::Y);
    }
    if args.flag_z {
        lattice = lattice.drop(Axis::Z);
    }
    write(lattice);
    Ok(())
}

fn expand(args: Args) -> Result<(), Box<dyn Error>> {
    let mut lattice = read(&args.arg_input)?;
    if let Some(size) = args.flag_along_x {
        lattice = lattice.expand_along(Axis::X, size);
    }
    if let Some(size) = args.flag_along_y {
        lattice = lattice.expand_along(Axis::Y, size);
    }
    if let Some(size) = args.flag_along_z {
        lattice = lattice.expand_along(Axis::Z, size);
    }
    write(lattice);
    Ok(())
}

fn alloy(args: Args) -> Result<(), Box<dyn Error>> {
    let source = args.arg_source;
    let kinds = args.arg_target.iter().map(|s| s.as_str()).collect();
    let ratios = args.arg_ratio;
    let alloy = Alloy::new(kinds, ratios);
    let mut lattice = read(&args.arg_input)?;
    lattice = lattice.alloy_sites(&source, alloy);
    write(lattice);
    Ok(())
}

fn mask(args: Args) -> Result<(), Box<dyn Error>> {
    let path = args.arg_mask;
    let ppu: f64 = args.flag_ppu;
    let mask = Mask::new(Path::new(&path), ppu)?;
    let mut lattice = read(&args.arg_input)?;
    lattice = lattice.apply_mask(mask);
    write(lattice);
    Ok(())
}

fn into(args: Args) -> Result<(), Box<dyn Error>> {
    let lattice = read(&args.arg_input)?;
    if args.cmd_tsv {
        for site in lattice.sites().iter() {
            let (x, y, z) = site.position();
            println!("{}\t{}\t{}\t{}", x, y, z, site.kind())
        }
    } else if args.cmd_xyz {
        println!("{}\n", lattice.sites().len());
        for site in lattice.sites().iter() {
            let (x, y, z) = site.position();
            println!("{} {} {} {}", site.kind(), x, y, z)
        }
    }
    Ok(())
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|doc| doc.version(Some("0.0.1".to_string())).deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_check {
        check_error(check(args));
    } else if args.cmd_pretty {
        check_error(pretty(args));
    } else if args.cmd_drop {
        check_error(drop(args));
    } else if args.cmd_alloy {
        check_error(alloy(args));
    } else if args.cmd_expand {
        check_error(expand(args));
    } else if args.cmd_mask {
        check_error(mask(args));
    } else if args.cmd_into {
        check_error(into(args));
    } else {
        println!("{:?}", args);
    }
}
