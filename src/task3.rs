use std::{collections::HashSet, ops::Add};

use itertools::Itertools;

use crate::utils::parse_lines;


fn split_racksack(mut racksack: String) -> (String, String) {
    let suffix = racksack.split_off(racksack.len() / 2);
    return (racksack, suffix)
}

fn get_racksack_intersection_item(splitted_racksack: (String, String)) -> Result<char, String> {
    let (prefix, suffix) = splitted_racksack;
    let prefix_set: HashSet<char> = prefix.chars().collect();
    let suffix_set: HashSet<char> = suffix.chars().collect();
    let intersection: Vec<char> = prefix_set.intersection(&suffix_set).cloned().collect();
    if intersection.len() != 1 {
        return Err("Intersection does not contain exactly one element.".to_string())
    }
    Ok(intersection[0])
}

fn get_group_intersection_item(group: Vec<String>) -> Result<char, String> {
    if group.len() != 3 {
        return Err("Group is not of size 3.".to_string())
    }
    let mut set: HashSet<char> = group[0].chars().collect();
    for i in 1..3{
        set = set.intersection(&group[i].chars().collect()).cloned().collect()
    }
    let intersection: Vec<char> = set.into_iter().collect();
    if intersection.len() != 1 {
        return Err("Intersection does not contain exactly one element.".to_string())
    }
    Ok(intersection[0])
}


fn get_item_priority(item: char) -> Result<u64, String> {
    let item_priority: u64 = item as u64;
    if item_priority >= 97 { // 'a' == 97
        Ok(item_priority - 97 + 1)
    } else if item_priority >= 65 { // 'A' == 65
        Ok(item_priority - 65 + 27)
    } else {
        Err(format!("Invalid item code: {} == {}.", item, item_priority).to_string())
    }
}

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    lines.into_iter()
        .map(split_racksack)
        .map(get_racksack_intersection_item)
        .map_ok(get_item_priority)
        .flatten_ok()
        .fold_ok(0, Add::add)
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    lines
        .chunks(3)
        .map(Vec::from)
        .map(get_group_intersection_item)
        .map_ok(get_item_priority)
        .flatten_ok()
        .fold_ok(0, Add::add)
}

#[cfg(test)]
mod tests {
    use crate::task3::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 157);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 70);
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
