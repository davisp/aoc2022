use clap::Parser;
use color_eyre::eyre::Result;

use crate::errors::AoCError;

mod aoc2022_01;
mod aoc2022_02;
mod aoc2022_03;
mod aoc2022_04;
mod errors;
mod reader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1, value_name = "SOLVER")]
    solver: String,
}

fn main() -> Result<(), AoCError> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.solver.as_str() {
        "01" => aoc2022_01::run("inputs/01"),
        "02" => aoc2022_02::run("inputs/02"),
        "03" => aoc2022_03::run("inputs/03"),
        "04" => aoc2022_04::run("inputs/04"),
        _ => panic!("Unknown solver: {}", args.solver),
    }
}
