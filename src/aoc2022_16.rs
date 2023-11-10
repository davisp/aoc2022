use std::cmp::max;

use crate::errors::AoCError;
use crate::reader::read_lines;

#[allow(clippy::needless_range_loop)]
pub fn run(input: String) -> Result<(), AoCError> {
    let mut board: Vec<Vec<usize>> = Vec::new();
    let mut forest: Vec<Vec<usize>> = Vec::new();

    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        board.push(vec![0; line.len()]);
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as usize)
        }
        forest.push(row);
    }

    let width: usize = forest[0].len();
    let height: usize = forest.len();

    let mut score: u64 = 0;
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            // Look north
            let mut north_count: u64 = 1;
            let curr = forest[y][x];
            let mut i = y - 1;
            while forest[i][x] < curr {
                if i == 0 {
                    break;
                }
                north_count += 1;
                i -= 1;
            }

            // Look south
            let mut south_count: u64 = 1;
            let curr = forest[y][x];
            let mut i = y + 1;
            while forest[i][x] < curr {
                if i >= height - 1 {
                    break;
                }
                south_count += 1;
                i += 1;
            }

            // Look west
            let mut west_count: u64 = 1;
            let curr = forest[y][x];
            let mut i = x - 1;
            while forest[y][i] < curr {
                if i == 0 {
                    break;
                }
                west_count += 1;
                i -= 1;
            }

            // Look east
            let mut east_count: u64 = 1;
            let curr = forest[y][x];
            let mut i = x + 1;
            while forest[y][i] < curr {
                if i >= width - 1 {
                    break;
                }
                east_count += 1;
                i += 1;
            }

            let curr_score =
                north_count * south_count * west_count * east_count;
            // println!(
            //     "({}, {}) = {} * {} * {} * {} = {}",
            //     x,
            //     y,
            //     north_count,
            //     south_count,
            //     west_count,
            //     east_count,
            //     curr_score
            // );

            score = max(score, curr_score);
        }
    }

    println!("Score: {}", score);

    Ok(())
}
