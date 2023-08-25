# vegas-lattice-rs

![Crates.io](https://img.shields.io/crates/v/vegas-lattice.svg)
![Build Status](https://github.com/odarbelaeze/vegas-lattice-rs/actions/workflows/rust.yml/badge.svg?branch=main)
[![DOI](https://zenodo.org/badge/90330925.svg)](https://zenodo.org/badge/latestdoi/90330925)

A little tool to build lattices and samples out of patterns written in
[rust].

Take a look at the [wiki].

## Installation

`vegas-lattice-rs` can be used as a standalone executable in order to build
yourself some lattices and otherwise it can be used as a rust crate (library).
If you have `cargo` installed in your system, you can grab the executable from
[crates.io] using:

```
cargo install vegas-lattice
```

after runing that you will have an executable `vegas-lattice` in your system
that will run as expected. Otherwise you can grab an appropriate binary from
the [releases] page.

If you intent to use it as a library just add the the following line to your
`Cargo.toml`:

```
vegas-lattice = "*"
```

Pin it at will when you're done, since this is an actively developed package.

## Usage

I'd recommend to alias `vegas-lattice` to something shorter, since the
pipelines can get really complex real quick.

```
alias vl=vegas-lattice
```

Now, lets write a basic example,

```
vl expand docs/bcc.json --x 10 --y 10 --z 5 \
    | vl alloy Fe Fe+ 50 Fe 50 \
    | vl into xyz
```

This command will create a 10x10x5 bcc lattice, and will turn half the iron
sites into iron + and after that it will transform the lattice into an xyz file
representation.

Notice that you can pipe the output of one command to the next one using the
standard io.

For more explanation in how the different commands work, take a look at the
[wiki] page.

[crates.io]: https://crates.io/
[rust]: https://www.rust-lang.org/en-US/
[wiki]: https://github.com/odarbelaeze/vegas-lattice-rs/wiki
[releases]: https://github.com/odarbelaeze/vegas-lattice-rs/releases
