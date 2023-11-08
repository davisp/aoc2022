use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use regex::Regex;

use crate::errors::AoCError;
use crate::reader::read_lines;

#[derive(Debug)]
struct FSEntry {
    name: String,
    size: u64,
    children: Vec<FSEntryRef>,
}

type FSEntryRef = Rc<RefCell<FSEntry>>;

impl FSEntry {
    fn root() -> FSEntry {
        FSEntry::new_dir("/".to_string())
    }

    fn new_dir(name: String) -> FSEntry {
        FSEntry {
            name,
            size: 0,
            children: Vec::new(),
        }
    }

    fn new_file(name: String, size: u64) -> FSEntry {
        FSEntry {
            name,
            size,
            children: Vec::new(),
        }
    }

    fn lookup(&self, paths: &[String]) -> FSEntryRef {
        let next = &paths[0];

        for child in self.children.iter() {
            let name = child.borrow().name.clone();
            if name == next.as_str() {
                if paths.len() == 1 {
                    return child.clone();
                } else {
                    return child.borrow().lookup(&paths[1..]);
                }
            }
        }

        panic!("Failed to find child node: {}", paths[0]);
    }
}

struct FSEntryPretty<'a>(&'a FSEntryRef);

impl<'a> fmt::Debug for FSEntryPretty<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "(dir) {}", this.name)?;
        } else {
            writeln!(f, "({}) {}", this.size, this.name)?;
        }

        for child in this.children.iter() {
            for line in format!("{:?}", FSEntryPretty(child)).lines() {
                writeln!(f, "  {}", line)?;
            }
        }

        Ok(())
    }
}

pub fn run(input: String) -> Result<(), AoCError> {
    let cmd_re = Regex::new(r"\$\s+(cd|ls)(?:\s+(.+))?").unwrap();
    let result_re = Regex::new(r"(dir|\d+)\s+(.+)").unwrap();

    let root = Rc::new(RefCell::new(FSEntry::root()));
    let mut cwd: Vec<String> = Vec::new();

    let lines = read_lines(input)?;
    for line in lines.into_iter().flatten() {
        //println!("Line: {}", line);
        if let Some(caps) = cmd_re.captures(line.as_str()) {
            let cmd = caps.get(1).unwrap().as_str();
            if cmd == "ls" {
                continue;
            }
            let dname = caps.get(2).unwrap().as_str();
            if dname == "/" {
                cwd.clear();
            } else if dname.contains('/') {
                panic!("Multi-level cd!");
            } else if dname == ".." {
                cwd.pop();
            } else {
                cwd.push(dname.to_string());
            }
        } else if let Some(caps) = result_re.captures(line.as_str()) {
            let is_dir = caps.get(1).unwrap().as_str() == "dir";
            let entry = if is_dir {
                Rc::new(RefCell::new(FSEntry::new_dir(
                    caps.get(2).unwrap().as_str().to_string(),
                )))
            } else {
                let size: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
                Rc::new(RefCell::new(FSEntry::new_file(
                    caps.get(2).unwrap().as_str().to_string(),
                    size,
                )))
            };
            if cwd.is_empty() {
                root.borrow_mut().children.push(entry)
            } else {
                let parent = root.borrow().lookup(&cwd[0..]);
                parent.borrow_mut().children.push(entry);
            }
        }
    }

    println!("{:?}", FSEntryPretty(&root));

    Ok(())
}
