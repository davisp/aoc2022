use regex::Regex;

use crate::errors::AoCError;
use crate::reader::read_lines;

pub fn run(input: String) -> Result<(), AoCError> {
    let instr_re = Regex::new(r"(noop)|(addx (-?\d+))").unwrap();

    let mut cycle: usize = 0;
    let mut x_reg = 1;
    let mut pixels: Vec<char> = vec!['.'; 240];

    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        let mut n_cycles = 0;
        let mut val = 0;

        match instr_re.captures(line.as_str()) {
            Some(caps) => {
                if caps.get(1).is_some() {
                    n_cycles = 1;
                } else if caps.get(2).is_some() {
                    n_cycles = 2;
                    val = caps[3].parse()?;
                }
            }
            None => panic!("Invalid input: {}", line),
        }

        let pixel = cycle as i64 % 40;
        if (pixel - x_reg).abs() <= 1 {
            pixels[cycle] = '#';
        }

        cycle += 1;
        println!("Cycle: {} = {}", cycle, x_reg);

        // Finished processing a no-op instructions
        if n_cycles == 1 {
            continue;
        }

        let pixel = cycle as i64 % 40;
        if (pixel - x_reg).abs() <= 1 {
            pixels[cycle] = '#';
        }

        cycle += 1;
        println!("Cycle: {} = {}", cycle, x_reg);

        // Set register value *after* second cycle.
        x_reg += val;
    }

    for i in (0..240).step_by(40) {
        let row: String = pixels[i..i + 40].iter().collect();
        println!("{}", row);
    }

    Ok(())
}
