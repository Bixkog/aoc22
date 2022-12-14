use std::iter::FromIterator;

use itertools::Itertools;

use crate::utils::parse_lines;

fn drop_sand(board: &Vec<Vec<char>>, mut x: usize, mut y: usize) -> Option<(usize, usize)> {
    loop {
        if y == board.len() - 1 {
            return None;
        } else if board[y+1][x] == '.' {
            y += 1;
            continue;
        } else if x == 0 {
            return None;
        } else if board[y+1][x-1] == '.' {
            y += 1;
            x -= 1;
            continue;
        } else if x == board[0].len() - 1 {
            return None;
        } else if board[y+1][x+1] == '.' {
            y += 1;
            x += 1;
            continue;
        }
        return Some((x, y))
    }
}

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let walls: Vec<Vec<(usize, usize)>> = lines
        .into_iter()
        .map(|s| s
            .split(" -> ")
            .map(|coords| coords
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple().unwrap())
            .collect())
        .collect();

    let min_x = walls.iter().flatten().min_by_key(|t| t.0).unwrap().0;
    let max_x = walls.iter().flatten().max_by_key(|t| t.0).unwrap().0 - min_x;
    let max_y = walls.iter().flatten().max_by_key(|t| t.1).unwrap().1;

    let walls = walls
        .into_iter()
        .map(|w| w
            .into_iter()
            .map(|c| (c.0 - min_x, c.1))
            .collect_vec())
        .collect_vec();
    
    let mut board = vec![vec!['.'; max_x + 1]; max_y + 1];
    board[0][500 - min_x] = '+';
    for w in walls {
        for (start, end) in w.iter().zip(w.iter().skip(1)) {
            if start.0 == end.0 {
                for i in if start.1 < end.1 {start.1..=end.1} else {end.1..=start.1} {
                    board[i][start.0] = '#';
                }
            }
            if start.1 == end.1 {
                for i in if start.0 < end.0 {start.0..=end.0} else {end.0..=start.0} {
                    board[start.1][i] = '#';
                }
            }
        }
    };

    loop {
        match drop_sand(&board, 500-min_x, 0) {
            None => break,
            Some((x, y)) => board[y][x] = 'o'
        }
    };

    for y in 0..board.len() {
        println!("{}", String::from_iter(board[y].iter()));
    };
    dbg!(max_x);
    Ok(board.into_iter().flatten().filter(|c| *c == 'o').count() as u64)
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let walls: Vec<Vec<(usize, usize)>> = lines
        .into_iter()
        .map(|s| s
            .split(" -> ")
            .map(|coords| coords
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple().unwrap())
            .collect())
        .collect();


    let max_y = walls.iter().flatten().max_by_key(|t| t.1).unwrap().1 + 2;
    let min_x = walls.iter().flatten().min_by_key(|t| t.0).unwrap().0 - max_y;
    let max_x = walls.iter().flatten().max_by_key(|t| t.0).unwrap().0 - min_x + 2 * max_y;
    
    let walls = walls
        .into_iter()
        .map(|w| w
            .into_iter()
            .map(|c| (c.0 - min_x, c.1))
            .collect_vec())
        .collect_vec();
    
    let mut board = vec![vec!['.'; max_x + 1]; max_y + 1];
    board[max_y] = vec!['#'; max_x + 1];
    board[0][500 - min_x] = '+';
    for w in walls {
        for (start, end) in w.iter().zip(w.iter().skip(1)) {
            if start.0 == end.0 {
                for i in if start.1 < end.1 {start.1..=end.1} else {end.1..=start.1} {
                    board[i][start.0] = '#';
                }
            }
            if start.1 == end.1 {
                for i in if start.0 < end.0 {start.0..=end.0} else {end.0..=start.0} {
                    board[start.1][i] = '#';
                }
            }
        }
    };

    loop {
        match drop_sand(&board, 500-min_x, 0) {
            None => break,
            Some((x, y)) => board[y][x] = 'o'
        }
        if board[0][500 - min_x] == 'o' {
            break
        }
    };

    for y in 0..board.len() {
        println!("{}", String::from_iter(board[y].iter()));
    };
    Ok(board.into_iter().flatten().filter(|c| *c == 'o').count() as u64)
}

#[cfg(test)]
mod tests {
    use crate::task14::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 24);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 93);
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
