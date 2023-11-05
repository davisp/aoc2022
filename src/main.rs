use clap::Parser;
use color_eyre::eyre::Result;

use dyn_sol::{aoc_imports, aoc_select};

use crate::errors::AoCError;

aoc_imports!();
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

    aoc_select!()
}
