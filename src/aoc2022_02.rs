use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::errors::AoCError;
use crate::reader::read_lines;

pub fn run(input: &str) -> Result<(), AoCError> {
    let mut heap = BinaryHeap::new();
    let mut cur_val: u64 = 0;

    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        if line.is_empty() {
            heap.push(Reverse(cur_val));
            cur_val = 0;

            if heap.len() > 3 {
                heap.pop();
            }
        } else {
            let val: u64 = line.parse()?;
            cur_val += val;
        }
    }

    let total: u64 = heap.into_iter().map(|val| val.0).sum();
    println!("Total Calories: {}", total);

    Ok(())
}
