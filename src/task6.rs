use itertools::Itertools;
use crate::utils::parse_lines;

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let msg = lines[0].chars().collect_vec();
    for i in 0..(msg.len()-3) {
        if msg[i..(i+4)].into_iter().unique().collect_vec().len() == 4 {
            return Ok((i+4) as u64)
        }
    }
    Err("Couldn't find start-of-packet marker.".to_string())
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let msg = lines[0].chars().collect_vec();
    for i in 0..(msg.len()-14) {
        if msg[i..(i+14)].into_iter().unique().collect_vec().len() == 14 {
            return Ok((i+14) as u64)
        }
    }
    Err("Couldn't find start-of-packet marker.".to_string())
}

#[cfg(test)]
mod tests {
    use crate::task6::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 5);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 23);
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
