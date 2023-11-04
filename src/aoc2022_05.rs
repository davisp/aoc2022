use std::collections::{HashMap, HashSet};

use crate::errors::AoCError;
use crate::reader::read_lines;

fn build_priority_map() -> HashMap<char, u64> {
    let mut ret = HashMap::new();

    for c in 'a'..='z' {
        ret.insert(c, (c as u64 - 'a' as u64) + 1);
    }

    for c in 'A'..='Z' {
        ret.insert(c, (c as u64 - 'A' as u64) + 27);
    }

    ret
}

pub fn run(input: String) -> Result<(), AoCError> {
    let priorities = build_priority_map();
    let mut total = 0;
    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        let line_len = line.len();
        assert_eq!(line_len % 2, 0);

        let compartment1 = &line[..line_len / 2];
        let compartment2 = &line[line_len / 2..];

        let set1: HashSet<char> = HashSet::from_iter(compartment1.chars());
        let set2: HashSet<char> = HashSet::from_iter(compartment2.chars());

        let in_both: Vec<_> = set1.intersection(&set2).collect();
        assert_eq!(in_both.len(), 1);

        match priorities.get(in_both[0]) {
            Some(priority) => total += priority,
            None => return Err(AoCError::InvalidInput(line)),
        }
    }

    println!("Total: {}", total);

    Ok(())
}
