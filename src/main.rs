extern crate serde;
extern crate serde_json;
extern crate vegas_lattice;
use clap::parser::ValuesRef;
use clap::{crate_authors, crate_name, crate_version, Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{stdin, Read};
use std::path::Path;
use vegas_lattice::{error::Result, io, Alloy, Lattice, Mask};

fn read(input: Option<&str>) -> Result<Lattice> {
    let mut data = String::new();
    if let Some(filename) = input {
        let mut file = File::open(filename)?;
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

fn check_error(res: Result<()>) {
    if let Err(e) = res {
        eprintln!("Error: {}", e);
        if let Some(source) = e.source() {
            eprintln!("Cause: {}", source);
        }
    }
}

// Commands over here

fn check(input: Option<&str>) -> Result<()> {
    let lattice = read(input)?;
    write(lattice);
    Ok(())
}

fn pretty(input: Option<&str>) -> Result<()> {
    let lattice = read(input)?;
    write_pretty(lattice);
    Ok(())
}

fn drop(input: Option<&str>, drop_x: bool, drop_y: bool, drop_z: bool) -> Result<()> {
    let mut lattice = read(input)?;
    if drop_x {
        lattice = lattice.drop_x();
    }
    if drop_y {
        lattice = lattice.drop_y();
    }
    if drop_z {
        lattice = lattice.drop_z();
    }
    write(lattice);
    Ok(())
}

fn expand(
    input: Option<&str>,
    along_x: Option<&usize>,
    along_y: Option<&usize>,
    along_z: Option<&usize>,
) -> Result<()> {
    let mut lattice = read(input)?;
    lattice = lattice.expand(
        along_x.copied().unwrap_or(1),
        along_y.copied().unwrap_or(1),
        along_z.copied().unwrap_or(1),
    );
    write(lattice);
    Ok(())
}

fn alloy(input: Option<&str>, source: &str, targets: Vec<(&str, u32)>) -> Result<()> {
    let alloy = Alloy::from_targets(targets);
    let mut lattice = read(input)?;
    lattice = lattice.alloy_sites(source, alloy);
    write(lattice);
    Ok(())
}

fn mask(input: Option<&str>, path: &str, ppu: f64) -> Result<()> {
    let mask = Mask::new(Path::new(&path), ppu)?;
    let mut lattice = read(input)?;
    lattice = lattice.apply_mask(mask);
    write(lattice);
    Ok(())
}

fn into(input: Option<&str>, format: &str) -> Result<()> {
    let lattice = read(input)?;
    match format {
        "tsv" => {
            for site in lattice.sites().iter() {
                let (x, y, z) = site.position();
                println!("{}\t{}\t{}\t{}", x, y, z, site.kind())
            }
        }
        "xyz" => {
            for site in lattice.sites().iter() {
                let (x, y, z) = site.position();
                println!("{} {} {} {}", site.kind(), x, y, z)
            }
        }
        _ => (),
    }
    Ok(())
}

fn main() {
    let cmd = Command::new(crate_name!())
        .bin_name(crate_name!())
        .about("Vegas lattice")
        .version(crate_version!())
        .author(crate_authors!())
        .long_about("Vegas lattice helps you manipulate lattice structures.")
        .subcommand_required(true)
        .subcommand(
            Command::new("sc").about("Simple cubic lattice").arg(
                Arg::new("a")
                    .long("lattice-parameter")
                    .short('a')
                    .default_value("1.0")
                    .value_parser(|s: &str| s.parse::<f64>())
                    .help("Lattice parameter"),
            ),
        )
        .subcommand(
            Command::new("bcc")
                .about("Body centered cubic lattice")
                .arg(
                    Arg::new("a")
                        .long("lattice-parameter")
                        .short('a')
                        .default_value("1.0")
                        .value_parser(|s: &str| s.parse::<f64>())
                        .help("Lattice parameter"),
                ),
        )
        .subcommand(
            Command::new("fcc")
                .about("Face centered cubic lattice")
                .arg(
                    Arg::new("a")
                        .long("lattice-parameter")
                        .short('a')
                        .default_value("1.0")
                        .value_parser(|s: &str| s.parse::<f64>())
                        .help("Lattice parameter"),
                ),
        )
        .subcommand(
            Command::new("check")
                .about("Check lattice")
                .arg(Arg::new("input").help("Input file").required(false)),
        )
        .subcommand(
            Command::new("pretty")
                .about("Pretty print lattice")
                .arg(Arg::new("input").help("Input file").required(false)),
        )
        .subcommand(
            Command::new("drop")
                .about("Drop periodic boundary conditions")
                .arg(Arg::new("input").help("Input file").required(false))
                .arg(
                    Arg::new("x")
                        .short('x')
                        .long("along-x")
                        .num_args(0)
                        .default_value("false")
                        .default_missing_value("true")
                        .value_parser(|s: &str| s.parse::<bool>())
                        .help("Drop periodic boundary conditions along x-axis"),
                )
                .arg(
                    Arg::new("y")
                        .short('y')
                        .long("along-y")
                        .num_args(0)
                        .default_value("false")
                        .default_missing_value("true")
                        .value_parser(|s: &str| s.parse::<bool>())
                        .help("Drop periodic boundary conditions along y-axis"),
                )
                .arg(
                    Arg::new("z")
                        .short('z')
                        .long("along-z")
                        .num_args(0)
                        .default_value("false")
                        .default_missing_value("true")
                        .value_parser(|s: &str| s.parse::<bool>())
                        .help("Drop periodic boundary conditions along z-axis"),
                ),
        )
        .subcommand(
            Command::new("expand")
                .about("Expand lattice")
                .arg(Arg::new("input").help("Input file").required(false))
                .arg(
                    Arg::new("x")
                        .short('x')
                        .long("along-x")
                        .value_parser(|s: &str| s.parse::<usize>())
                        .help("Expand lattice along x-axis"),
                )
                .arg(
                    Arg::new("y")
                        .short('y')
                        .long("along-y")
                        .value_parser(|s: &str| s.parse::<usize>())
                        .help("Expand lattice along y-axis"),
                )
                .arg(
                    Arg::new("z")
                        .short('z')
                        .long("along-z")
                        .value_parser(|s: &str| s.parse::<usize>())
                        .help("Expand lattice along z-axis"),
                ),
        )
        .subcommand(
            Command::new("alloy")
                .about("Alloy lattice")
                .arg(Arg::new("source").help("Source kind").required(true))
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .help("Target kind with is corresponding ratio")
                        .value_names(["target", "ratio"])
                        .action(ArgAction::Append)
                        .num_args(2),
                )
                .arg(
                    Arg::new("input")
                        .help("Input file")
                        .required(false)
                        .last(true),
                ),
        )
        .subcommand(
            Command::new("mask")
                .about("Mask lattice")
                .arg(Arg::new("mask").help("Mask file").required(true))
                .arg(Arg::new("input").help("Input file").required(false))
                .arg(
                    Arg::new("ppu")
                        .short('p')
                        .long("ppu")
                        .default_value("10")
                        .value_parser(|s: &str| s.parse::<f64>())
                        .help("Pixels per unit"),
                ),
        )
        .subcommand(
            Command::new("into")
                .about("Convert lattice into a different format")
                .arg(
                    Arg::new("format")
                        .help("Output format")
                        .required(true)
                        .value_parser(["xyz", "tsv"]),
                )
                .arg(Arg::new("input").help("Input file").required(false)),
        );

    let matches = cmd.get_matches();
    let result: Result<()> = match matches.subcommand() {
        Some(("sc", sub_matches)) => {
            let a = sub_matches.get_one::<f64>("a").unwrap();
            let lattice = Lattice::sc(*a);
            write(lattice);
            Ok(())
        }
        Some(("bcc", sub_matches)) => {
            let a = sub_matches.get_one::<f64>("a").unwrap();
            let lattice = Lattice::bcc(*a);
            write(lattice);
            Ok(())
        }
        Some(("fcc", sub_matches)) => {
            let a = sub_matches.get_one::<f64>("a").unwrap();
            let lattice = Lattice::fcc(*a);
            write(lattice);
            Ok(())
        }
        Some(("check", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").map(|s| s.as_str());
            check(input)
        }
        Some(("pretty", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").map(|s| s.as_str());
            pretty(input)
        }
        Some(("drop", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").map(|s| s.as_str());
            let drop_x = sub_matches.get_one::<bool>("x").unwrap();
            let drop_y = sub_matches.get_one::<bool>("y").unwrap();
            let drop_z = sub_matches.get_one::<bool>("z").unwrap();
            drop(input, *drop_x, *drop_y, *drop_z)
        }
        Some(("expand", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").map(|s| s.as_str());
            let along_x = sub_matches.get_one::<usize>("x");
            let along_y = sub_matches.get_one::<usize>("y");
            let along_z = sub_matches.get_one::<usize>("z");
            expand(input, along_x, along_y, along_z)
        }
        Some(("alloy", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").map(|s| s.as_str());
            let source = sub_matches.get_one::<String>("source").unwrap();
            let targets = sub_matches
                .get_many::<String>("target")
                .unwrap_or(ValuesRef::default());
            let kinds: Vec<_> = targets.clone().step_by(2).map(|s| s.as_str()).collect();
            let ratios: Vec<_> = targets
                .skip(1)
                .step_by(2)
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let target: Vec<_> = kinds.into_iter().zip(ratios).collect();
            alloy(input, source, target)
        }
        Some(("mask", sub_matches)) => {
            let path = sub_matches.get_one::<String>("mask").unwrap();
            let input = sub_matches.get_one::<String>("input").map(|s| s.as_str());
            let ppu = sub_matches.get_one::<f64>("ppu").unwrap();
            mask(input, path, *ppu)
        }
        Some(("into", sub_matches)) => {
            let format = sub_matches.get_one::<String>("format").unwrap();
            let input = sub_matches.get_one::<String>("input").map(|s| s.as_str());
            into(input, format)
        }
        _ => Ok(()),
    };
    check_error(result);
}
