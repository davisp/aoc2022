use crate::errors::AoCError;
use crate::reader::read_lines;

pub fn run(input: String) -> Result<(), AoCError> {
    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        println!("{}", line)
    }

    Ok(())
}
