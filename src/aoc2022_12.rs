use std::collections::{HashSet, VecDeque};

use crate::errors::AoCError;
use crate::reader::read_lines;

const WINDOW_SIZE: usize = 14;

fn is_start_of_packet(window: &VecDeque<char>) -> bool {
    let mut s: HashSet<char> = HashSet::new();

    for c in window {
        s.insert(*c);
    }

    s.len() == WINDOW_SIZE
}

fn solve(line: String) {
    let mut window: VecDeque<char> = VecDeque::new();

    for (idx, c) in line.chars().enumerate() {
        if window.len() < WINDOW_SIZE - 1 {
            window.push_back(c);
            continue;
        }

        window.push_back(c);
        if window.len() > WINDOW_SIZE {
            window.pop_front();
        }

        assert!(window.len() == WINDOW_SIZE, "Len {}", window.len());

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
