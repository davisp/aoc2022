use std::collections::{HashSet, VecDeque};

use crate::errors::AoCError;
use crate::reader::read_lines;

fn is_start_of_packet(window: &VecDeque<char>) -> bool {
    let mut s: HashSet<char> = HashSet::new();

    for c in window {
        s.insert(*c);
    }

    s.len() == 4
}

fn solve(line: String) {
    let mut window: VecDeque<char> = VecDeque::new();

    for (idx, c) in line.chars().enumerate() {
        if window.len() < 3 {
            window.push_back(c);
            continue;
        }

        window.push_back(c);
        if window.len() > 4 {
            window.pop_front();
        }

        assert!(window.len() == 4, "Len: {}", window.len());

        if is_start_of_packet(&window) {
            // +1 for 1 based indexing in problem statement.
            println!("Index: {}", idx + 1);
            return;
        }
    }
}

pub fn run(input: String) -> Result<(), AoCError> {
    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        solve(line);
    }

    Ok(())
}
