use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_lines<P>(filename: P) -> Result<Vec<String>, String> 
where P: AsRef<Path>, {
    if let Ok(lines) = read_lines(filename) {
        lines.map(|s| {s.map_err(|e| {e.to_string()})}).collect()
    } else {
        Err("IO error at opening file: {filename}".to_string())
    }
}
