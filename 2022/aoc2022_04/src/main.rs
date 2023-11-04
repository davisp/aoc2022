use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use color_eyre::eyre::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Invalid play: {0}")]
    InvalidPlay(String),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1, value_name = "FILE")]
    input: String,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// A X 1 - Rock / Lose
// B Y 2 - Paper / Tie
// C Z 3 - Scissors / Win
fn build_score_table() -> HashMap<String, u64> {
    let mut ret = HashMap::new();

    ret.insert(String::from("A X"), 3); // Lose + Scissors
    ret.insert(String::from("A Y"), 4); // Tie + Rock
    ret.insert(String::from("A Z"), 8); // Win + Paper
    ret.insert(String::from("B X"), 1); // Lose + Rock
    ret.insert(String::from("B Y"), 5); // Tie + Paper
    ret.insert(String::from("B Z"), 9); // Win + Scissors
    ret.insert(String::from("C X"), 2); // Lose + Paper
    ret.insert(String::from("C Y"), 6); // Tie + Scissors
    ret.insert(String::from("C Z"), 7); // Win + Rock

    ret
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let score_table = build_score_table();
    let mut score: u64 = 0;

    let lines = read_lines(args.input)?;
    for line in lines.into_iter().flatten() {
        match score_table.get(&line) {
            Some(points) => score += points,
            None => panic!("Invalid line: {}", line),
        }
    }

    println!("Score: {}", score);

    Ok(())
}
