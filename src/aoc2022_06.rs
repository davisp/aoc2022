use std::collections::{HashMap, HashSet};

use itertools::Itertools;

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
    for mut chunk in lines.chunks(3).into_iter() {
        let set1: HashSet<char> =
            HashSet::from_iter(chunk.next().unwrap()?.as_str().chars());
        let set2: HashSet<char> =
            HashSet::from_iter(chunk.next().unwrap()?.as_str().chars());
        let set3: HashSet<char> =
            HashSet::from_iter(chunk.next().unwrap()?.as_str().chars());

        let in_all = [&set2, &set3]
            .iter()
            .fold(set1, |s1, s2| s1.intersection(s2).cloned().collect());

        assert!(chunk.next().is_none());

        match in_all.iter().next() {
            Some(priority) => total += priorities.get(priority).unwrap(),
            None => {
                return Err(AoCError::InvalidInput(String::from("Some stuff")))
            }
        }
    }

    println!("Total: {}", total);

    Ok(())
}
