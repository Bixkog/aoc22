use std::ops::Add;

use itertools::Itertools;

use crate::utils::parse_lines;

#[derive(Clone)]
struct Range {
    pub min: u64,
    pub max: u64
}

impl Range {
    fn from_str(str_range: String) -> Result<Range, String> {
        let splitted_str = str_range.split("-").collect_vec();
        if splitted_str.len() != 2 {
            return Err(format!("Can't split range from: {}", str_range).to_string());
        }
        return Ok(
            Range {
                min: splitted_str[0].parse()
                    .or(Err(format!("Can't parse first part of range: {}", str_range).to_string()))?,
                max: splitted_str[1].parse()
                    .or(Err(format!("Can't parse second part of range: {}", str_range).to_string()))?,
            }
        )
    }

    fn is_in(&self, other: &Range) -> bool {
        self.min >= other.min && self.max <= other.max
    }

    fn overlaps(&self, other: &Range) -> bool {
        !(self.min > other.max || self.max < other.min)
    }
}

fn parse_ranges(line: String) -> Result<(Range, Range), String> {
    let ranges: Vec<Range> = line.split(",")
        .map(|s| s.to_string())
        .map(Range::from_str)
        .collect::<Result<Vec<Range>, String>>()?;
    if ranges.len() != 2 {
        return Err(format!("Can't split {} into exactly two ranges.", line).to_string());
    }
    Ok((ranges[0].clone(), ranges[1].clone()))
}

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    lines
        .into_iter()
        .map(parse_ranges)
        .map_ok(|(r1, r2)| r1.is_in(&r2) || r2.is_in(&r1))
        .map_ok(|b| b as u64)
        .fold_ok(0, Add::add)
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    lines
        .into_iter()
        .map(parse_ranges)
        .map_ok(|(r1, r2)| r1.overlaps(&r2))
        .map_ok(|b| b as u64)
        .fold_ok(0, Add::add)
}

#[cfg(test)]
mod tests {
    use crate::task4::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 2);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 4);
    }

    #[test]
    fn target() {
        println!("{}", task("input.txt").unwrap());
    }

    #[test]
    fn target_part_two() {
        println!("{}", task_part_two("input.txt").unwrap());
    }
}
