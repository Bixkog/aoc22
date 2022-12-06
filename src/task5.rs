use itertools::Itertools;
use regex::Regex;
use crate::utils::parse_lines;

fn parse_containers(mut input: Vec<String>) -> Result<Vec<Vec<char>>, String> {
    if input.len() < 2 {
        return Err("Invalid containers input.".to_string());
    }
    input.pop();
    let bytes_input = input.iter().map(|s| s.as_bytes()).collect_vec();
    let number_of_columns = input[0].len() / 4 + 1;
    let mut columns: Vec<Vec<char>> = vec![vec![]; number_of_columns];
    for column_id in 0..(number_of_columns) {
        for row_id in (0..input.len()).rev() {
            let container = bytes_input[row_id][1 + column_id * 4] as char;
            if container != ' ' {
                columns[column_id].push(container)
            } 
           
        }
    }
    Ok(columns)
}

fn parse_moves(input: Vec<String>) -> Result<Vec<(usize, usize, usize)>, String> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").or(Err("Can't compile regex.".to_string()))?;
    input
        .iter()
        .map(|s| re.captures(s.as_str()).ok_or("Error at regex parsing moves.".to_string()))
        .map_ok(|c| 
            {
                let parse_err = "Can't parse number.".to_string();
                Ok::<_, String>((
                    c[1].parse().or(Err(parse_err.clone()))?, 
                    c[2].parse().or(Err(parse_err.clone()))?, 
                    c[3].parse().or(Err(parse_err.clone()))?))
            })
        .flatten_ok()
        .collect()
}

enum CraneVersions {
    CrateMover9000,
    CrateMover9001
}

fn simulate_crane(mut containers: Vec<Vec<char>>, moves: Vec<(usize, usize, usize)>, crane_version: CraneVersions) -> Result<String, String> {
    for (q, from, to) in moves {
        let reduced_length = containers[from - 1].len() - q;
        let moved_containers = match crane_version {
             CraneVersions::CrateMover9000 => Vec::from(&containers[from - 1][reduced_length..]).into_iter().rev().collect(),
             CraneVersions::CrateMover9001 => Vec::from(&containers[from - 1][reduced_length..]),
        };
        containers[to - 1].extend(moved_containers);
        containers[from - 1].truncate(reduced_length);
    };
    containers.into_iter()
        .map(|container| container.last().ok_or("Container is empty, can't get last element.".to_string()).cloned())
        .collect()
}

fn task(input_path: &str) -> Result<String, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let blocks = lines.split(|s| s.is_empty()).map(Vec::from).collect_vec();
    if blocks.len() != 2 {
        return Err("Error at splitting input into two blocks.".to_string());
    }
    let containers = parse_containers(blocks[0].clone())?;
    let moves = parse_moves(blocks[1].clone())?;

    simulate_crane(containers, moves, CraneVersions::CrateMover9000)
}

fn task_part_two(input_path: &str) -> Result<String, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let blocks = lines.split(|s| s.is_empty()).map(Vec::from).collect_vec();
    if blocks.len() != 2 {
        return Err("Error at splitting input into two blocks.".to_string());
    }
    let containers = parse_containers(blocks[0].clone())?;
    let moves = parse_moves(blocks[1].clone())?;

    simulate_crane(containers, moves, CraneVersions::CrateMover9001)
}

#[cfg(test)]
mod tests {
    use crate::task5::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), "CMZ");
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), "MCD");
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
