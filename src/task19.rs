use std::collections::HashMap;

use crate::utils::parse_lines;
use itertools::Itertools;
use par_map::ParMap;
use regex::Regex;
 
#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: u64,
    clay_robot_cost: u64,
    obsidian_robot_cost: (u64, u64),
    geode_robot_cost: (u64, u64),
}
 
fn parse_blueprint(s: String) -> Blueprint {
    let re = Regex::new(r"(\d+)").unwrap();
    let captures = re.captures_iter(s.as_str())
        .map(|capt| capt[0].parse::<u64>().unwrap())
        .collect_vec();
    Blueprint { 
        ore_robot_cost: captures[1], 
        clay_robot_cost: captures[2],
        obsidian_robot_cost: (captures[3], captures[4]),
        geode_robot_cost: (captures[5], captures[6]) 
    }
}
 
fn get_blueprint_score(b: Blueprint, time: u64) -> u64 {
    let mut cache = HashMap::new();
    find_max(&b, [1, 0, 0, 0], [0, 0, 0, 0], time, &mut cache)
}

fn compose_state(robots: &[u64; 4], ores: &[u64; 4], time: u64) -> [u64; 7] {
    [
        robots[0], robots[2], robots[3],
        ores[0], ores[2], ores[3],
        time
    ]
}

fn find_max(b: &Blueprint, robots: [u64; 4], ores: [u64; 4], time: u64, cache: &mut HashMap<[u64; 7], u64>) -> u64 {
    let mut max_score = ores[3];
    let state = compose_state(&robots, &ores, time);
    if time < 10 && cache.contains_key(&state) {
        return *cache.get(&state).unwrap()
    }
    if time > 0 && ores[0] >= b.geode_robot_cost.0 && ores[2] >= b.geode_robot_cost.1 {
        max_score = find_max(b, 
                        [robots[0], robots[1], robots[2], robots[3] + 1], 
                        [ores[0] + robots[0] - b.geode_robot_cost.0, ores[1] + robots[1], ores[2] + robots[2] - b.geode_robot_cost.1, ores[3] + robots[3]], 
                        time - 1,
                        cache).max(max_score);
    }
    if time > 4 && robots[2] < b.geode_robot_cost.1 && ores[0] >= b.obsidian_robot_cost.0 && ores[1] >= b.obsidian_robot_cost.1 {
        max_score = find_max(b, 
                            [robots[0], robots[1], robots[2] + 1, robots[3]], 
                            [ores[0] + robots[0] - b.obsidian_robot_cost.0, ores[1] + robots[1] - b.obsidian_robot_cost.1, ores[2] + robots[2], ores[3] + robots[3]], 
                            time - 1,
                            cache).max(max_score);
    }
    if time > 4 && robots[1] < b.obsidian_robot_cost.1 && ores[0] >= b.clay_robot_cost {
        max_score = find_max(b, 
                            [robots[0], robots[1] + 1, robots[2], robots[3]], 
                            [ores[0] + robots[0] - b.clay_robot_cost, ores[1] + robots[1], ores[2] + robots[2], ores[3] + robots[3]], 
                            time - 1,
                            cache).max(max_score);
    }
    if time > 4 && robots[0] < b.ore_robot_cost.max(b.clay_robot_cost).max(b.obsidian_robot_cost.0).max(b.geode_robot_cost.0) && ores[0] >= b.ore_robot_cost {
        max_score = find_max(b, 
                            [robots[0] + 1, robots[1], robots[2], robots[3]], 
                            [ores[0] + robots[0] - b.ore_robot_cost, ores[1] + robots[1], ores[2] + robots[2], ores[3] + robots[3]], 
                            time - 1,
                            cache).max(max_score);
    }
    if time > 0 {
        max_score = find_max(b, robots.clone(), [ores[0] + robots[0], ores[1] + robots[1], ores[2] + robots[2], ores[3] + robots[3]], time - 1, cache).max(max_score);
    }
    cache.insert(state, max_score);
    max_score
}

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let blueprints = lines.into_iter().map(parse_blueprint).collect_vec();
    Ok(dbg!(blueprints
        .into_iter()
        .map(|b| get_blueprint_score(b, 24))
        .zip(1..).collect_vec()).into_iter()
        .fold(0, |s,  (score, id)| s + score * id))
}
 
fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let blueprints = lines.into_iter().map(parse_blueprint).collect_vec();
    Ok(dbg!(blueprints
        .into_iter()
        .take(3)
        .par_map(|b| get_blueprint_score(b, 32))
        .collect_vec()).into_iter()
        .product())
}
 
#[cfg(test)]
mod tests {
    use crate::task19::{task, task_part_two};
 
    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 33);
    }
 
    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 62 * 56);
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