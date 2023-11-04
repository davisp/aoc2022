use std::cmp;

use crate::errors::AoCError;
use crate::reader::read_lines;

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
