use std::cmp;
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

pub fn run(input: &str) -> Result<(), AoCError> {
    let mut max_val: u64 = 0;
    let mut cur_val: u64 = 0;

    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        if line.is_empty() {
            max_val = cmp::max(cur_val, max_val);
            cur_val = 0;
        } else {
            let val: u64 = line.parse()?;
            cur_val += val;
        }
    }

    println!("Max calories: {}", max_val);

    Ok(())
}
