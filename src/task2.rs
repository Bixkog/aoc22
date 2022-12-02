use crate::utils::parse_lines;

fn play_to_score(play: String) -> Result<u64, String> {
    match play.as_str() {
        "A X" => Ok(3 + 1),
        "A Y" => Ok(6 + 2),
        "A Z" => Ok(0 + 3),
        "B X" => Ok(0 + 1),
        "B Y" => Ok(3 + 2),
        "B Z" => Ok(6 + 3),
        "C X" => Ok(6 + 1),
        "C Y" => Ok(0 + 2),
        "C Z" => Ok(3 + 3),
        _ => Err("Invalid play: {play}".to_string())
    }
}

fn play_to_score_part_two(play: String) -> Result<u64, String> {
    match play.as_str() {
        "A X" => Ok(0 + 3),
        "A Y" => Ok(3 + 1),
        "A Z" => Ok(6 + 2),
        "B X" => Ok(0 + 1),
        "B Y" => Ok(3 + 2),
        "B Z" => Ok(6 + 3),
        "C X" => Ok(0 + 2),
        "C Y" => Ok(3 + 3),
        "C Z" => Ok(6 + 1),
        _ => Err("Invalid play: {play}".to_string())
    }
}

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    Ok(
        lines.into_iter()
            .map(|play| play_to_score(play))
            .collect::<Result<Vec<u64>, String>>()?
            .into_iter()
            .sum()
    )
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
        let lines: Vec<String> = parse_lines(input_path)?;
    Ok(
        lines.into_iter()
            .map(|play| play_to_score_part_two(play))
            .collect::<Result<Vec<u64>, String>>()?
            .into_iter()
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use crate::task2::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 15);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 12);
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
