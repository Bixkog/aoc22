use crate::utils::parse_lines;
use itertools::Itertools;

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    lines
        .into_iter()
        .map(|s| {if s.is_empty() {0} else {s.parse::<u64>().unwrap()}})
        .collect::<Vec<u64>>()
        .group_by(|_, b|  *b > 0)
        .map(|elf_calories| elf_calories
            .into_iter()
            .fold(0, |acc, food_calories| acc + food_calories))
        .max()
        .ok_or("Can't use max on empty list.".to_string())
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    Ok(lines
        .into_iter()
        .map(|s| {if s.is_empty() {0} else {s.parse::<u64>().unwrap()}})
        .collect::<Vec<u64>>()
        .group_by(|_, b|  *b > 0)
        .map(|elf_calories| elf_calories
            .into_iter()
            .sum::<u64>())
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .take(3)
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::task1::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 24000);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 45000);
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
