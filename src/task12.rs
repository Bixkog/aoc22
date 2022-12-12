use std::collections::VecDeque;

use itertools::Itertools;
use crate::utils::parse_lines;

fn find_source_sink(lines: &Vec<String>) -> Result<((usize, usize), (usize, usize)), String> {
    let row_len = lines[0].len();
    let chars = lines.iter().flat_map(|s| s.chars());
    let source_pos_1d = chars.clone().find_position(|c| *c == 'S').ok_or("Can't find source".to_string())?.0;
    let sink_pos_1d = chars.clone().find_position(|c| *c == 'E').ok_or("Can't find sink".to_string())?.0;
    Ok(((source_pos_1d / row_len, source_pos_1d % row_len),
        (sink_pos_1d / row_len, sink_pos_1d % row_len)))
}

fn char_to_elevation(c: char) -> i32 {
    match c {
        'S' => 'a' as i32,
        'E' => 'z' as i32,
        c => c as i32
    }
}

fn neighbours(pos: (usize, usize), max0: i32, max1: i32) -> Vec<(usize, usize)> {
   vec![(pos.0 as i32 + 1, pos.1  as i32), 
        (pos.0  as i32 - 1, pos.1  as i32), 
        (pos.0  as i32, pos.1  as i32 + 1), 
        (pos.0  as i32, pos.1  as i32 - 1)].into_iter()
        .filter(|t| (*t).0 >= 0 && (*t).0 < max0 && (*t).1 >= 0 && (*t).1 < max1)
        .map(|(x, y)| (x as usize, y as usize))
        .collect_vec()
   
}

fn bootstrap_queue(q: &mut VecDeque<(usize, usize)>, elevation: &Vec<Vec<i32>>, distance: &mut Vec<Vec<usize>>) {
    let row_len = elevation[0].len();
    let a_positions = elevation.iter()
        .flatten()
        .zip(0..)
        .filter_map(|(e, pos)| {if *e == 'a' as i32 {Some(pos)} else {None}})
        .map(|pos_1d| (pos_1d / row_len, pos_1d % row_len ));
    for a_pos in a_positions {
        distance[a_pos.0][a_pos.1] = 0;
        q.push_back(a_pos);
    }
}    

fn shortes_path(elevation: Vec<Vec<i32>>, _: (usize, usize), sink: (usize, usize)) -> usize {
    const MAX_DIST: usize = 1000000;
    let (max0, max1) = (elevation.len(), elevation[0].len());
    let mut distance = vec![vec![MAX_DIST; max1]; max0];
    // part 1
    // distance[source.0][source.1] = 0;
    // let mut q = VecDeque::from(vec![source]);
    let mut q = VecDeque::new();
    bootstrap_queue(&mut q, &elevation, &mut distance);
    while !q.is_empty() && distance[sink.0][sink.1] == MAX_DIST {
        let pos = q.pop_front().unwrap();
        let pos_elevation = elevation[pos.0][pos.1];
        for neighbour in neighbours(pos, max0 as i32, max1 as i32) {
            let neighbour_elevation = elevation[neighbour.0][neighbour.1];
            if distance[neighbour.0][neighbour.1] == MAX_DIST && neighbour_elevation <= pos_elevation + 1  {
                distance[neighbour.0][neighbour.1] = distance[pos.0][pos.1] + 1;
                q.push_back(neighbour);
            }
        }
    }
    distance[sink.0][sink.1] 
}

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let (source, sink) = find_source_sink(&lines)?;

    let elevation = lines.into_iter()
                                        .map(|l| l.chars().map(char_to_elevation).collect_vec())
                                        .collect_vec();

    Ok(shortes_path(elevation, source, sink) as u64)
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    task(input_path)
}

#[cfg(test)]
mod tests {
    use crate::task12::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 31);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 29);
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
