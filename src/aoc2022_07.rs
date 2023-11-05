use regex::Regex;

use crate::errors::AoCError;
use crate::reader::read_lines;

pub fn run(input: String) -> Result<(), AoCError> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut count: u64 = 0;

    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        let caps = re.captures(line.as_str()).unwrap();

        let l1: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
        let l2: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
        let r1: u64 = caps.get(3).unwrap().as_str().parse().unwrap();
        let r2: u64 = caps.get(4).unwrap().as_str().parse().unwrap();

        assert!(l1 <= l2);
        assert!(r1 <= r2);

        if (l1 <= r1 && l2 >= r2) || (r1 <= l1 && r2 >= l2) {
            count += 1;
        }
    }

    println!("Count: {}", count);

    Ok(())
}
