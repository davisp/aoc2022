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

    let mut visible: usize = 0;

    // Outside perimiter is visible
    visible += width + width + height + height - 4;

    for x in 1..width - 1 {
        // Top down
        let mut curr_height = forest[0][x];
        for y in 1..height - 1 {
            if forest[y][x] > curr_height {
                if board[y][x] == 0 {
                    visible += 1;
                    board[y][x] = 1;
                }
                curr_height = forest[y][x];
            }
        }

        // Bottom up
        curr_height = forest[height - 1][x];
        for y in (1..height - 1).rev() {
            if forest[y][x] > curr_height {
                if board[y][x] == 0 {
                    visible += 1;
                    board[y][x] = 1;
                }
                curr_height = forest[y][x];
            }
        }
    }

    for y in 1..height - 1 {
        // Left right
        let mut curr_height = forest[y][0];
        for x in 1..(width - 1) {
            if forest[y][x] > curr_height {
                if board[y][x] == 0 {
                    visible += 1;
                    board[y][x] = 1;
                }
                curr_height = forest[y][x];
            }
        }

        // Right left
        curr_height = forest[y][width - 1];
        for x in (1..width - 1).rev() {
            if forest[y][x] > curr_height {
                if board[y][x] == 0 {
                    visible += 1;
                    board[y][x] = 1;
                }
                curr_height = forest[y][x];
            }
        }
    }

    println!("Visible: {}", visible);

    Ok(())
}
