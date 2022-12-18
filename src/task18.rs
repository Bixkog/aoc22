use std::collections::{HashSet, HashMap};

use itertools::Itertools;

use crate::utils::parse_lines;

type Position = (i64, i64, i64);

fn get_area(cubes: &HashSet<Position>) -> u64 {
    let mut area = 0;
    for c in cubes.iter() {
        if !cubes.contains(&(c.0-1, c.1, c.2)) {
            area += 1;
        }
        if !cubes.contains(&(c.0, c.1-1, c.2)) {
            area += 1;
        }
        if !cubes.contains(&(c.0, c.1, c.2-1)) {
            area += 1;
        }
        if !cubes.contains(&(c.0+1, c.1, c.2)) {
            area += 1;
        }
        if !cubes.contains(&(c.0, c.1+1, c.2)) {
            area += 1;
        }
        if !cubes.contains(&(c.0, c.1, c.2+1)) {
            area += 1;
        }
    }
    area
}

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let cubes: HashSet<Position> = lines
        .into_iter()
        .map(|s| s
            .split(",")
            .map(|id| id
                .parse::<i64>()
                .unwrap())
            .collect_tuple()
            .unwrap())
        .collect();
    Ok(get_area(&cubes))
}

struct Node {
    parent: Position,
    size: u64,
}

type UnionFind = HashMap<Position, Node>;

fn new_set(sets: &mut UnionFind, pos: &Position) {
    sets.insert(pos.clone(), Node {parent: pos.clone(), size: 1});
}

fn find(sets: &mut UnionFind, pos: Position) -> Position {
    if pos != sets[&pos].parent {
        let new_parent = find(sets, sets[&pos].parent.clone());
        sets.get_mut(&pos).unwrap().parent = new_parent;
        return new_parent
    } else {
        pos
    }
}

fn union(sets: &mut UnionFind, pos1: &Position, pos2: &Position) {
    let mut parent1 = find(sets, pos1.clone());
    let mut parent2 = find(sets, pos2.clone());
    if parent1 != parent2 {
        if sets[&parent1].size < sets[&parent2].size {
            (parent1, parent2) = (parent2, parent1);
        }
        sets.get_mut(&parent2).unwrap().parent = parent1;
        sets.get_mut(&parent1).unwrap().size += sets[&parent2].size;
    }
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let cubes: HashSet<Position> = lines
        .into_iter()
        .map(|s| s
            .split(",")
            .map(|id| id
                .parse::<i64>()
                .unwrap())
            .collect_tuple()
            .unwrap())
        .collect();
    let (min_x, max_x) = cubes.iter().map(|p| p.0).minmax().into_option().unwrap();
    let (min_y, max_y) = cubes.iter().map(|p| p.1).minmax().into_option().unwrap();
    let (min_z, max_z) = cubes.iter().map(|p| p.2).minmax().into_option().unwrap();
    
    let mut sets = UnionFind::new();
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                let c_pos = (x, y, z);
                if !cubes.contains(&c_pos) {
                    new_set(&mut sets, &c_pos);
                    for neighbour in [(x-1, y, z), (x, y-1, z), (x, y, z-1)] {
                        if sets.contains_key(&neighbour) && !cubes.contains(&neighbour) {
                            union(&mut sets, &neighbour, &c_pos);
                        }
                    }
                }
            }
        }
    }

    let empty_spaces = sets
        .keys()
        .cloned()
        .collect_vec();
    let bubbles: HashSet<_> = empty_spaces
        .into_iter()
        .filter(|p| find(&mut sets, p.clone()) != (min_x - 1, min_y - 1, min_z - 1))
        .collect();
    Ok(get_area(&cubes) - get_area(&bubbles))
}

#[cfg(test)]
mod tests {
    use crate::task18::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 64);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 58);
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
