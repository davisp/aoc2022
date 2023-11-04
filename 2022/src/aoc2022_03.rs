use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::errors::AoCError;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// A X 1 - Rock
// B Y 2 - Paper
// C Z 3 - Scissors
fn build_score_table() -> HashMap<String, u64> {
    let mut ret = HashMap::new();

    ret.insert(String::from("A X"), 4); // Tie + Rock
    ret.insert(String::from("A Y"), 8); // Win + Paper
    ret.insert(String::from("A Z"), 3); // Lose + Scissors
    ret.insert(String::from("B X"), 1); // Lose + Rock
    ret.insert(String::from("B Y"), 5); // Tie + Paper
    ret.insert(String::from("B Z"), 9); // Win + Scissors
    ret.insert(String::from("C X"), 7); // Win + Rock
    ret.insert(String::from("C Y"), 2); // Lose + Paper
    ret.insert(String::from("C Z"), 6); // Tie + Scissors

    ret
}

pub fn run(input: &str) -> Result<(), AoCError> {
    let score_table = build_score_table();
    let mut score: u64 = 0;

    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        match score_table.get(&line) {
            Some(points) => score += points,
            None => return Err(AoCError::InvalidInput(line)),
        }
    }

    println!("Score: {}", score);

    Ok(())
}
