use itertools::Itertools;
use crate::utils::parse_lines;

fn is_rope_stretched(t_pos: (i32, i32), h_pos: (i32, i32)) -> bool {
    (t_pos.0 - h_pos.0).abs() > 1 || (t_pos.1 - h_pos.1).abs() > 1
}

fn tail_positions(lines: Vec<String>) -> Result<Vec<(i32, i32)>, String> {
    Ok(lines.into_iter()
        .map(|s| {
            let mut line_part = s.split(" ");
            let direction = line_part.next().ok_or("Empty line.".to_string())?.to_string();
            let distance = line_part.next().ok_or("Can't read distance.".to_string())?
                .parse::<usize>().or(Err("Can't parse distance.".to_string()))?;
            Ok::<Vec<String>, String>(vec![direction; distance])
        })
        .flatten_ok()
        .fold_ok((vec![(0, 0)], (0, 0), (0, 0)), |s, direction|{
            let (history, t_pos, h_pos) = s;
            let new_h_pos = match direction.as_str() {
                "R" => (h_pos.0 + 1, h_pos.1),
                "L" => (h_pos.0 - 1, h_pos.1),
                "U" => (h_pos.0, h_pos.1 + 1),
                "D" => (h_pos.0, h_pos.1 - 1),
                _ => panic!("Invalid direction.")
            };
            let new_t_pos = if is_rope_stretched(t_pos, new_h_pos) { h_pos } else { t_pos };
            (history.into_iter().chain([new_t_pos]).collect(), new_t_pos, new_h_pos)
        })?
        .0)
}

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    Ok(tail_positions(lines)?.into_iter().unique().count() as u64)
}

fn are_diagonal(prev_pos: (i32, i32), pos: (i32, i32)) -> bool {
    (prev_pos.0 - pos.0).abs() == 1 && (prev_pos.1 - pos.1).abs() == 1
    
}

fn follow_positions(positions: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    positions.into_iter().skip(1).fold(
        (vec![(0, 0)], (0, 0)), |(tail_positions, prev_pos), pos| {
            let t_pos = tail_positions.last().unwrap();
            let new_t_pos = 
            if is_rope_stretched(*t_pos, pos) { 
                if are_diagonal(prev_pos, pos) {
                    if are_diagonal(*t_pos, prev_pos) {
                        ((pos.0 + t_pos.0) / 2, (pos.1 + t_pos.1) / 2)
                    } else {
                        (pos.0 + t_pos.0 - prev_pos.0, t_pos.1 + pos.1 - prev_pos.1)
                    }
                } else {
                    prev_pos
                }
            } else { 
                *t_pos 
            };
            (tail_positions.into_iter().chain([new_t_pos]).collect(), pos)
        }
    ).0
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    
    // let mut rope_positions = vec![vec![]; 10];

    // rope_positions[0] = lines.clone().into_iter()
    //     .map(|s| {
    //         let mut line_part = s.split(" ");
    //         let direction = line_part.next().ok_or("Empty line.".to_string())?.to_string();
    //         let distance = line_part.next().ok_or("Can't read distance.".to_string())?
    //             .parse::<usize>().or(Err("Can't parse distance.".to_string()))?;
    //         Ok::<Vec<String>, String>(vec![direction; distance])
    //     })
    //     .flatten_ok()
    //     .fold_ok(vec![(0, 0)], |history, direction|{
    //         let h_pos = history.last().unwrap();
    //         let new_h_pos = match direction.as_str() {
    //             "R" => (h_pos.0 + 1, h_pos.1),
    //             "L" => (h_pos.0 - 1, h_pos.1),
    //             "U" => (h_pos.0, h_pos.1 + 1),
    //             "D" => (h_pos.0, h_pos.1 - 1),
    //             _ => panic!("Invalid direction.")
    //         };
    //         history.into_iter().chain([new_h_pos]).collect()
    //     })?;

    // rope_positions[1] = tail_positions(lines.clone())?;
    // for i in 2..10 {
    //     rope_positions[i] = follow_positions(rope_positions[i-1].clone());
    // };

    // let mut final_board = vec![vec!['.'; 26]; 22];
    // for i in 0..rope_positions[0].len() {
    //     let mut board = vec![vec!['.'; 26]; 22];
    //     board[5][11] = 's';
    //     for p in 0..10 {
    //         let (x, y) = rope_positions[p][i];
    //         board[(y + 5) as usize][(x + 11) as usize] = char::from_digit(p as u32, 10).unwrap();
    //     }
    //     let (x, y) = rope_positions[9][i];
    //     final_board[(y + 5) as usize][(x + 11) as usize] = '#';
    //     for r in (0..22).rev() {
    //         println!("{}", board[r].iter().collect::<String>());
    //     }
    //     println!()
    // }
    //  for r in (0..22).rev() {
    //     println!("{}", final_board[r].iter().collect::<String>());
    // }
    // println!();




    Ok((0..8).fold(tail_positions(lines)?, |head_positions, _| follow_positions(head_positions))
        .into_iter().unique().count() as u64)
}

#[cfg(test)]
mod tests {
    use crate::task9::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 13);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 36);
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
