use std::collections::HashSet;

use regex::Regex;

use crate::errors::AoCError;
use crate::reader::read_lines;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    dir: Direction,
    dist: u64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn update_knot(rope: &mut [Point], idx: usize) {
    let dx = rope[idx - 1].x - rope[idx].x;
    let dy = rope[idx - 1].y - rope[idx].y;
    assert!(
        (-2..=2).contains(&dx) && (-2..=2).contains(&dy),
        "Whut?! {} {:?} {:?}",
        idx,
        rope[idx - 1],
        rope[idx]
    );

    if (-1..=1).contains(&dx) && (-1..=1).contains(&dy) {
        // Still touching, nothing to do.
        return;
    }

    let dx = match dx {
        -2 => -1,
        2 => 1,
        _ => 0,
    };

    let dy = match dy {
        -2 => -1,
        2 => 1,
        _ => 0,
    };

    rope[idx].x = rope[idx - 1].x - dx;
    rope[idx].y = rope[idx - 1].y - dy;
}

pub fn run(input: String) -> Result<(), AoCError> {
    let move_re = Regex::new(r"([UDLR])\s+(\d+)").unwrap();

    let mut moves = Vec::new();

    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        match move_re.captures(line.as_str()) {
            Some(caps) => {
                let dir = match &caps[1] {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Invalid direction: {}", &caps[1]),
                };
                let dist: u64 = caps[2].parse()?;
                moves.push(Move { dir, dist });
            }
            None => panic!("Invalid input line: {}", line),
        }
    }

    let mut visited = HashSet::new();
    let mut rope = vec![Point { x: 0, y: 0 }; 10];
    visited.insert(rope[rope.len() - 1].clone());

    for m in moves {
        let (x, y) = match m.dir {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        for _ in 0..m.dist {
            rope[0].x += x;
            rope[0].y += y;
            for i in 1..rope.len() {
                update_knot(&mut rope, i);
            }
            visited.insert(rope[rope.len() - 1].clone());
        }
    }

    println!("Count: {}", visited.len());

    Ok(())
}
