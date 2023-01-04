use std::iter::FromIterator;

use itertools::Itertools;
use regex::Regex;

use crate::utils::parse_lines;

fn parse_moves(s: &String) -> (Vec<i64>, Vec<char>) {
    let re = Regex::new(r"([0-9]+)").unwrap();
    let dists = re.captures_iter(s.as_str())
        .map(|capt|
            capt[0].parse::<i64>().unwrap()
        )
        .collect_vec();
    let re = Regex::new(r"(L|R)").unwrap();
    let rots = re.captures_iter(s.as_str())
        .map(|capt|
            capt[0].chars().next().unwrap()
        )
        .collect_vec();
    (dists, rots)
}

fn walk(board: &Vec<(usize, Vec<char>)>,pos: &mut (usize, usize), ori: &i32, dist: i64) {
    for _ in 0..dist {
        match ori {
            0 =>  {
                let mut new_y;
                if pos.1 == 0 || board[pos.1 - 1].1[pos.0] == ' ' {
                    new_y = pos.1;
                    loop {
                        if new_y + 1 == board.len() || board[new_y + 1].1[pos.0] == ' ' {
                            break;
                        }
                        new_y += 1;
                    };
                } else {
                    new_y = pos.1 - 1;
                }
                if board[new_y].1[pos.0] == '#' {
                    break;
                }
                pos.1 = new_y;
            }
            1 => {
                let mut new_x = pos.0 + 1;
                if board[pos.1].1.len() == new_x || board[pos.1].1[new_x] == ' ' {
                    new_x = board[pos.1].0;
                }
                if board[pos.1].1[new_x] == '#' {
                    break;
                }
                pos.0 = new_x;
            }
            2 => {
                let mut new_y;
                if pos.1 + 1 == board.len() || board[pos.1 + 1].1[pos.0] == ' ' {
                    new_y = pos.1;
                    loop {
                        if new_y == 0 || board[new_y - 1].1[pos.0] == ' ' {
                            break;
                        }
                        new_y -= 1;
                    };
                } else {
                    new_y = pos.1 + 1;
                }
                if board[new_y].1[pos.0] == '#' {
                    break;
                }
                pos.1 = new_y;
            }
            3 => {
                let new_x;
                if pos.0 == 0 {
                    new_x = board[pos.1].1.len() - board[pos.1].1.iter().rev().position(|c| *c != ' ').unwrap() - 1;
                } else {
                    new_x = pos.0 - 1;
                }
                if board[pos.1].1[new_x] == '#' {
                    break;
                }
                pos.0 = new_x;
            }
            _ => unimplemented!()
        }
    }
}

fn task(input_path: &str) -> Result<i64, String> {
    let mut lines: Vec<String> = parse_lines(input_path)?;
    let (dists, rots) = parse_moves(lines.last().unwrap());
    lines.pop(); lines.pop();
    let max_len = lines.iter().map(|l| l.len()).max().unwrap();
    let board = lines.into_iter()
        .map(|l| {
            let mut v = l.chars().collect_vec();
            v.resize(max_len, ' ');
            (v.iter().position(|c| *c != ' ').unwrap(), v)
        })
        .collect_vec();
    let mut pos = (0, 0);
    let mut ori = 1;
    pos.0 = board[0].0;
    for (dist, rot) in dists.into_iter().zip(rots.into_iter().chain("N".chars())) {
        walk(&board, &mut pos, &ori, dist);
        match rot {
            'R' => ori = (ori + 1) % 4,
            'L' => ori = (ori - 1).rem_euclid(4),
            _ => {}
        }
    }
    Ok((1004 + pos.1 * 1000 + pos.0 * 4 + (ori - 1).rem_euclid(4) as usize) as i64)
}

fn corner_walk(pos: &(usize, usize), ori: i32) -> ((usize, usize), i32) {
    match ori {
        0 => {
            if pos.1 == 0 && pos.0 < 150 && pos.0 >= 100 {
                // A.T -> F.B
                ((pos.0 - 100, 199), 0)
            } else if pos.1 == 0 && pos.0 < 100 && pos.0 >= 50 {
                // B.T -> F.L
                ((0, pos.0 + 100), 1)
            } else if pos.1 == 100 && pos. 0 < 50 {
                // E.T -> C.L
                ((50, 50 + pos.0), 1)
            } else {
                unimplemented!()
            }
        }
        1 => {
            if pos.0 == 149 && pos.1 < 50 {
                // A.R -> D.R
                ((99, 149 - pos.1), 3)
            } else if pos.0 == 99 && pos.1 >= 50 && pos.1 < 100 {
                // C.R -> A.B
                ((50 + pos.1, 49), 0)
            } else if pos.0 == 99 && pos.1 >= 100 && pos.1 < 150 {
                // D.R -> A.R
                ((149, 49 - (pos.1 - 100)), 3)
            } else if pos.0 == 49 && pos.1 >= 150 {
                // F.R -> D.B
                ((pos.1 - 150 + 50, 149), 0)
            } else {
                unimplemented!()
            }
        }
        2 => {
            if pos.1 == 49 && pos.0 >= 100 {
                // A.B -> C.R
                ((99, pos.0 - 50), 3)
            } else if pos.1 == 149 && pos.0 >= 50 && pos.0 < 100 {
                // D.B -> F.R
                ((49, pos.0 + 100), 3)
            } else if pos.1 == 199 && pos.0 < 50 {
                // F.B -> A.T
                ((pos.0 + 100, 0), 2)
            } else {
                unimplemented!()
            }
        }
        3 => {
            if pos.0 == 50 && pos.1 < 50 {
                // B.L -> E.L
                ((0, 149 - pos.1), 1)
            } else if pos.0 == 50 && pos.1 >= 50 && pos.1 < 100 {
                // C.L -> E.T
                ((pos.1 - 50, 100), 2)
            } else if pos.0 == 0 && pos.1 >= 100 && pos.1 < 150 {
                // E.L -> B.L
                ((50, 49 - (pos.1 - 100)), 1)
            } else if pos.0 == 0 && pos.1 >= 150 {
                // F.L -> B.T
                ((pos.1 - 100, 0), 2)
            } else {
                unimplemented!()
            }
        },
        _ => unimplemented!()
    }
}

fn walk2(board: &Vec<(usize, Vec<char>)>, draw_board: &mut Vec<(usize, Vec<char>)>, pos: &mut (usize, usize), ori: &mut i32, dist: i64) {
    for _ in 0..dist {
        let mut new_pos = *pos;
        let mut new_ori = *ori;
        match ori {
            0 =>  {
                draw_board[pos.1].1[pos.0] = '^';
                if pos.1 == 0 || board[pos.1 - 1].1[pos.0] == ' ' {
                    (new_pos, new_ori) = corner_walk(&pos, *ori);
                } else {
                    new_pos.1 = pos.1 - 1;
                }
                if board[new_pos.1].1[new_pos.0] == '#' {
                    break;
                }
            }
            1 => {
                draw_board[pos.1].1[pos.0] = '>';
                if board[pos.1].1.len() == pos.0 + 1 || board[pos.1].1[pos.0 + 1] == ' ' {
                    (new_pos, new_ori) = corner_walk(&pos, *ori);
                } else {
                    new_pos.0 = pos.0 + 1
                }
            }
            2 => {
                draw_board[pos.1].1[pos.0] = 'v';
                if pos.1 + 1 == board.len() || board[pos.1 + 1].1[pos.0] == ' ' {
                    (new_pos, new_ori) = corner_walk(&pos, *ori);
                } else {
                    new_pos.1 = pos.1 + 1;
                }
            }
            3 => {
                draw_board[pos.1].1[pos.0] = '<';
                if pos.0 == 0 || board[pos.1].1[pos.0 - 1] == ' ' {
                    (new_pos, new_ori) = corner_walk(&pos, *ori);
                } else {
                    new_pos.0 = pos.0 - 1;
                }
            }
            _ => unimplemented!()
        }
        if board[new_pos.1].1[new_pos.0] == '#' {
            break;
        }
        *pos = new_pos;
        *ori = new_ori;
    }
}

fn task_part_two(input_path: &str) -> Result<i64, String> {
    let mut lines: Vec<String> = parse_lines(input_path)?;
    let (dists, rots) = parse_moves(lines.last().unwrap());
    lines.pop(); lines.pop();
    let max_len = lines.iter().map(|l| l.len()).max().unwrap();
    let board = lines.into_iter()
        .map(|l| {
            let mut v = l.chars().collect_vec();
            v.resize(max_len, ' ');
            (v.iter().position(|c| *c != ' ').unwrap(), v)
        })
        .collect_vec();
    let mut draw_board = board.clone();
    let mut pos = (0, 0);
    let mut ori = 1;
    pos.0 = board[0].0;
    for (dist, rot) in dists.into_iter().zip(rots.into_iter().chain("N".chars())) {
        walk2(&board, &mut draw_board, &mut pos, &mut ori, dist);
        match rot {
            'R' => ori = (ori + 1) % 4,
            'L' => ori = (ori - 1).rem_euclid(4),
            _ => {}
        }
    }

    for (_, l) in draw_board {
        println!("{}", String::from_iter(l.into_iter()));
    }

    Ok((1004 + pos.1 * 1000 + pos.0 * 4 + (ori - 1).rem_euclid(4) as usize) as i64)
}
 
#[cfg(test)]
mod tests {
    use crate::task22::{task, task_part_two};
 
    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 6032);
    }
 
    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 5031);
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