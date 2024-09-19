use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use std::error::Error;
use std::fs::File;
use std::io::{stdin, Read};
use std::path::{Path, PathBuf};
use vegas_lattice::{error::Result, io, Alloy, Lattice, Mask};

fn read(input: Option<&Path>) -> Result<Lattice> {
    let mut data = String::new();
    if let Some(path) = input {
        let mut file = File::open(path)?;
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

fn check(input: Option<&Path>) -> Result<()> {
    let lattice = read(input)?;
    write(lattice);
    Ok(())
}

fn pretty(input: Option<&Path>) -> Result<()> {
    let lattice = read(input)?;
    write_pretty(lattice);
    Ok(())
}

fn drop(input: Option<&Path>, drop_x: bool, drop_y: bool, drop_z: bool) -> Result<()> {
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
    input: Option<&Path>,
    along_x: Option<usize>,
    along_y: Option<usize>,
    along_z: Option<usize>,
) -> Result<()> {
    let mut lattice = read(input)?;
    lattice = lattice.expand(
        along_x.unwrap_or(1),
        along_y.unwrap_or(1),
        along_z.unwrap_or(1),
    );
    write(lattice);
    Ok(())
}

fn alloy(input: Option<&Path>, source: &str, targets: Vec<String>) -> Result<()> {
    let kinds: Vec<_> = targets.iter().step_by(2).map(|s| s.as_str()).collect();
    let ratios: Vec<_> = targets
        .iter()
        .skip(1)
        .step_by(2)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let target: Vec<_> = kinds.into_iter().zip(ratios).collect();
    let alloy = Alloy::try_from_targets(target)?;
    let mut lattice = read(input)?;
    lattice = lattice.alloy_sites(source, alloy);
    write(lattice);
    Ok(())
}

fn mask(input: Option<&Path>, path: &Path, ppu: f64) -> Result<()> {
    let mask = Mask::try_new(path, ppu)?;
    let mut lattice = read(input)?;
    lattice = lattice.apply_mask(mask);
    write(lattice);
    Ok(())
}

fn into(input: Option<&Path>, format: Format) -> Result<()> {
    let lattice = read(input)?;
    match format {
        Format::Tsv => {
            for site in lattice.sites().iter() {
                let (x, y, z) = site.position();
                println!("{}\t{}\t{}\t{}", x, y, z, site.kind())
            }
        }
        Format::Xyz => {
            for site in lattice.sites().iter() {
                let (x, y, z) = site.position();
                println!("{} {} {} {}", site.kind(), x, y, z)
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    /// XYZ file format
    Xyz,
    /// TSV file format
    Tsv,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    /// Create a simple cubic lattice
    Sc {
        #[arg(long = "lattice-parameter", short, default_value = "1.0")]
        /// Lattice parameter
        a: f64,
    },
    /// Create a body centered cubic lattice
    Bcc {
        #[arg(long = "lattice-parameter", short, default_value = "1.0")]
        /// Lattice parameter
        a: f64,
    },
    /// Create a face centered cubic lattice
    Fcc {
        #[arg(long = "lattice-parameter", short, default_value = "1.0")]
        /// Lattice parameter
        a: f64,
    },
    /// Check lattice
    Check {
        /// Input file
        input: Option<PathBuf>,
    },
    /// Pretty print lattice
    Pretty {
        /// Input file
        input: Option<PathBuf>,
    },
    /// Drop periodic boundary conditions
    Drop {
        /// Input file
        input: Option<PathBuf>,
        #[arg(short, long = "along-x", default_value = "false")]
        /// Drop periodic boundary conditions along x-axis
        x: bool,
        #[arg(short, long = "along-y", default_value = "false")]
        /// Drop periodic boundary conditions along y-axis
        y: bool,
        #[arg(short, long = "along-z", default_value = "false")]
        /// Drop periodic boundary conditions along z-axis
        z: bool,
    },
    /// Expand lattice
    Expand {
        /// Input file
        input: Option<PathBuf>,
        #[arg(short, long = "along-x")]
        /// Expand lattice along x-axis
        x: Option<usize>,
        #[arg(short, long = "along-y")]
        /// Expand lattice along y-axis
        y: Option<usize>,
        #[arg(short, long = "along-z")]
        /// Expand lattice along z-axis
        z: Option<usize>,
    },
    /// Create an alloy
    Alloy {
        /// Source kind
        source: String,
        #[arg(
            short,
            long,
            value_names = ["target", "ratio"],
            number_of_values = 2,
            action = ArgAction::Append,
        )]
        /// Target kind with is corresponding ratio
        target: Vec<String>,
        /// Input file
        input: Option<PathBuf>,
    },
    /// Apply a mask
    Mask {
        /// Mask file
        mask: PathBuf,
        /// Input file
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "10")]
        /// Pixels per unit
        ppu: f64,
    },
    /// Convert lattice into a different format
    Into {
        /// Output format
        format: Format,
        /// Input file
        input: Option<PathBuf>,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.subcmd {
        SubCommand::Sc { a } => {
            let lattice = Lattice::sc(a);
            write(lattice);
            Ok(())
        }
        SubCommand::Bcc { a } => {
            let lattice = Lattice::bcc(a);
            write(lattice);
            Ok(())
        }
        SubCommand::Fcc { a } => {
            let lattice = Lattice::fcc(a);
            write(lattice);
            Ok(())
        }
        SubCommand::Check { input } => check(input.as_deref()),
        SubCommand::Pretty { input } => pretty(input.as_deref()),
        SubCommand::Drop { input, x, y, z } => drop(input.as_deref(), x, y, z),
        SubCommand::Expand { input, x, y, z } => expand(input.as_deref(), x, y, z),
        SubCommand::Alloy {
            source,
            target,
            input,
        } => alloy(input.as_deref(), &source, target),
        SubCommand::Mask {
            mask: mask_path,
            input,
            ppu,
        } => mask(input.as_deref(), &mask_path, ppu),
        SubCommand::Into { format, input } => into(input.as_deref(), format),
    };

    check_error(result);
}
