use regex::Regex;

use crate::errors::AoCError;
use crate::reader::read_lines;

struct Command {
    name: String,
    args: String,
}

struct FSEntry {
    name: Option<String>,
    is_dir: bool,
    size: u64,
    children: Vec<FSEntry>,
}

impl FSEntry {
    fn root() -> FSEntry {
        FSEntry {
            name: None,
            is_dir: true,
            size: 0,
            children: Vec::new(),
        }
    }

    fn new_dir(name: String, is_dir: bool) -> FSEntry {
        FSEntry {
            name: Some(name),
            is_dir,
            size: 0,
            children: Vec::new(),
        }
    }
}

pub fn run(input: String) -> Result<(), AoCError> {
    let cmd_re = Regex::new(r"\$\s+(cd|ls)(?:\s+(.+))").unwrap();
    let result_re = Regex::new(r"(dir|\d+)\s+(.+)").unwrap();

    let root = FSEntry::root();

    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        if let Some(caps) = cmd_re.captures(line.as_str()) {
            println!("{}", caps.get(1).unwrap().as_str());
        } else if let Some(caps) = result_re.captures(line.as_str()) {
            println!("{}", caps.get(1).unwrap().as_str());
        }
    }

    Ok(())
}
