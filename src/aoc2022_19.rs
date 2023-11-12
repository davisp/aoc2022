use regex::Regex;

use crate::errors::AoCError;
use crate::reader::read_lines;

pub fn run(input: String) -> Result<(), AoCError> {
    let instr_re = Regex::new(r"(noop)|(addx (-?\d+))").unwrap();

    let mut cycle = 0;
    let mut x_reg = 1;
    let mut signal = 0;

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

        for i in 1..=n_cycles {
            if ((cycle + i) - 20) % 40 == 0 {
                signal += (cycle + i) * x_reg;
                println!("CYCLE CYCLE CYCLE: {} = {}", cycle + i, signal);
            }
        }

        cycle += n_cycles;
        x_reg += val;

        println!("Cycle: {} = {}", cycle, x_reg);
    }

    println!("Signal: {}", signal);

    Ok(())
}
