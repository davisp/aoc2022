use std::collections::HashMap;

use regex::Regex;

use crate::errors::AoCError;
use crate::reader::read_lines;

#[derive(Debug)]
struct Move {
    count: u64,
    from: u64,
    to: u64,
}

impl Move {
    fn new(count: u64, from: u64, to: u64) -> Move {
        Move { count, from, to }
    }
}

pub fn run(input: String) -> Result<(), AoCError> {
    let stack_re = Regex::new(r"\[([A-Z])\]").unwrap();
    let move_re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();

    let mut stacks: HashMap<u64, Vec<char>> = HashMap::new();
    let mut moves: Vec<Move> = Vec::<Move>::new();

    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        for caps in stack_re.captures_iter(line.as_str()) {
            for cap in caps.iter().skip(1) {
                let idx: u64 = ((cap.unwrap().start() - 1) / 4) as u64;
                let stack = stacks.entry(idx).or_default();
                let char = line.chars().nth(cap.unwrap().start()).unwrap();
                stack.push(char);
            }
        }

        if let Some(caps) = move_re.captures(line.as_str()) {
            let count: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let from: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
            let to: u64 = caps.get(3).unwrap().as_str().parse().unwrap();
            moves.push(Move::new(count, from, to));
        }
    }

    for entry in stacks.iter_mut() {
        entry.1.reverse();
    }

    for m in moves {
        let mut crane: Vec<char> = Vec::<char>::new();

        for _ in 0..m.count {
            let item = stacks.get_mut(&(m.from - 1)).unwrap().pop().unwrap();
            crane.push(item);
        }

        for _ in 0..m.count {
            let item = crane.pop().unwrap();
            stacks.get_mut(&(m.to - 1)).unwrap().push(item);
        }
    }

    let mut sol = Vec::new();
    for i in 0..stacks.len() {
        let item = stacks.get(&(i as u64)).unwrap().iter().last().unwrap();
        sol.push(item);
    }

    println!("{}", String::from_iter(sol.into_iter()));

    Ok(())
}
