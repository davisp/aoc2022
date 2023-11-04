use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use color_eyre::eyre::Result;

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

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let mut heap = BinaryHeap::new();
    let mut cur_val: u64 = 0;

    let lines = read_lines(args.input)?;
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
