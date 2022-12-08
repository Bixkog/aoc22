use itertools::{Itertools, Zip, multizip, izip};
use crate::utils::parse_lines;

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let row_len = lines[0].len() as i32;
    let col_len = lines.len() as i32;
    println!("{} {}", row_len, col_len);
    let forest = lines.join("").chars().map(|c| c as i32 - '0' as i32).collect_vec();
    let mut visible = vec![0; forest.len()];
    let is_visible = &mut |max: &mut i32, x: i32, y: i32| {
         if forest[(y * row_len + x) as usize] > *max {
            *max = forest[(y * row_len + x) as usize];
            visible[(y * row_len + x) as usize] = 1;
        }
    };
    for y in 0..col_len {
        let mut max = [-1; 4];
        for x in 0..row_len {
            is_visible(&mut max[0], x, y);
            is_visible(&mut max[1], row_len - x - 1, y);
            is_visible(&mut max[2], y, x);
            is_visible(&mut max[3], y, col_len - x - 1);
        }
        
    }
    Ok(visible.into_iter().sum())
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let row_len = lines[0].len() as i32;
    let col_len = lines.len() as i32;
    let forest = lines.join("").chars().map(|c| c as i32 - '0' as i32).collect_vec();
    let mut view_distance = vec![vec![0; forest.len()]; 4]; // left right top bottom
    let calculate_view_distance = 
        &mut |direction: usize, last_height_pos: &mut [i32; 10], x: i32, y: i32| {
        let c_height = forest[(y * row_len + x) as usize] as usize;
        view_distance[direction][(y * row_len + x) as usize] = match direction {
            0 => x - *last_height_pos[c_height..].iter().max().unwrap(),
            1 => *last_height_pos[c_height..].iter().min().unwrap() - x,
            2 => y - *last_height_pos[c_height..].iter().max().unwrap(),
            3 => *last_height_pos[c_height..].iter().min().unwrap() - y,
            _ => panic!("Invalid direction."),
        };
        last_height_pos[c_height] = if direction < 2 {x} else {y}; 
    };
    for y in 0..col_len {
        let mut last_height_pos = [[0; 10]; 4];
        last_height_pos[1] = [row_len - 1; 10];
        last_height_pos[3] = [col_len - 1; 10];
        for x in 1..row_len {
            calculate_view_distance(0, &mut last_height_pos[0], x, y);
            calculate_view_distance(1, &mut last_height_pos[1], row_len - x - 1, y);
            calculate_view_distance(2, &mut last_height_pos[2], y, x);
            calculate_view_distance(3, &mut last_height_pos[3], y, col_len - x - 1);
        }
    }
    Ok(
        izip!(view_distance[0].iter(), view_distance[1].iter(), view_distance[2].iter(), view_distance[3].iter())
        .map(|(l, r, t, b)| l * r * t * b )
        .max().unwrap() as u64)
}

#[cfg(test)]
mod tests {
    use crate::task8::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 21);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 8);
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
