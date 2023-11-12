use std::collections::HashSet;

use regex::Regex;

use crate::errors::AoCError;
use crate::reader::read_lines;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    dir: Direction,
    dist: u64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
}

// Some possible states:
// 1 2 3
// 4 H 5
// 6 7 8
fn update_tail(h: &Point, t: &mut Point) {
    let dx = h.x - t.x;
    let dy = h.y - t.y;
    assert!(
        (-2..=2).contains(&dx) && (-2..=2).contains(&dy),
        "Whut?! {:?} {:?}",
        h,
        t
    );

    if (-1..=1).contains(&dx) && (-1..=1).contains(&dy) {
        // Still touching, nothing to do.
        return;
    }

    if dx == -2 || dx == 2 {
        assert!((-1..=1).contains(&dy), "Whut?! {:?} {:?}", h, t);
        if dx < 0 {
            t.x = h.x + 1;
            t.y = h.y;
        } else {
            t.x = h.x - 1;
            t.y = h.y;
        }
        return;
    }

    if dy == -2 || dy == 2 {
        assert!((-1..=1).contains(&dx), "Whut?! {:?} {:?}", h, t);
        if dy < 0 {
            t.x = h.x;
            t.y = h.y + 1;
        } else {
            t.x = h.x;
            t.y = h.y - 1;
        }
        return;
    }

    unreachable!();
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
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    visited.insert(tail.clone());

    for m in moves {
        let (x, y) = match m.dir {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        for _ in 0..m.dist {
            head.x += x;
            head.y += y;
            update_tail(&head, &mut tail);
            visited.insert(tail.clone());
        }
    }

    println!("Count: {}", visited.len());

    Ok(())
}
