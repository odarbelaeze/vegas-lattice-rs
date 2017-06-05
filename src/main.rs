#[macro_use] extern crate clap;
use clap::App;
use std::path::Path;

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml).get_matches();
    let input = Path::new(matches.value_of("input").unwrap());
    print!("{:?}", input);
}