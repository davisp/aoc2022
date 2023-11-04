use clap::Parser;
use color_eyre::eyre::Result;

use crate::errors::AoCError;

mod aoc2022_01;
mod aoc2022_02;
mod aoc2022_03;
mod aoc2022_04;
mod aoc2022_05;
mod errors;
mod reader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1, value_name = "SOLVER")]
    solver: String,

    #[arg(long, short, action)]
    sample: bool,
}

fn select_input(solver: &String, use_sample: bool) -> String {
    if use_sample {
        String::from("inputs/") + &solver + &String::from("_sample")
    } else {
        String::from("inputs/") + &solver
    }
}

fn main() -> Result<(), AoCError> {
    color_eyre::install()?;
    let args = Args::parse();
    let input = select_input(&args.solver, args.sample);

    match args.solver.as_str() {
        "01" => aoc2022_01::run(input),
        "02" => aoc2022_02::run(input),
        "03" => aoc2022_03::run(input),
        "04" => aoc2022_04::run(input),
        "05" => aoc2022_05::run(input),
        _ => panic!("Unknown solver: {}", args.solver),
    }
}
