use itertools::Itertools;

use crate::utils::parse_lines;

const BOARD_HEIGHT: usize = 100000; // 2022 * 13 / 5 + 1;
const BOARD_WIDTH: usize = 7 + 2;
type Board = [[bool; BOARD_WIDTH]; BOARD_HEIGHT];

fn new_board() -> Board {
    let mut board: Board = [[true; BOARD_WIDTH]; BOARD_HEIGHT];
    board[0] = [false; BOARD_WIDTH];
    for y in 1..BOARD_HEIGHT {
        board[y][0] = false;
        board[y][BOARD_WIDTH - 1] = false;
    };
    board
}

#[derive(Clone, PartialEq, Debug)]
enum Shape {
    Hline, 
    // @###
    Plus,
    //  #
    // ###
    // .#
    L,
    //   #
    //   #
    // @##
    Vline,
    // #
    // #
    // #
    // @
    Square
    // ##
    // @#
}

#[derive(Clone, Debug)]
struct Piece {
    x: usize,
    y: usize,
    shape: Shape
}

impl Piece {
    fn can_move_to(&self, board: &Board, x: usize, y: usize) -> bool {
        match self.shape {
            Shape::Hline => {
                board[y][x] && board[y][x + 1] && board[y][x + 2] && board[y][x + 3]
            },
            Shape::Plus => {
                board[y][x + 1] && board[y + 1][x] && board[y + 1][x + 1] && board[y + 1][x + 2] && board[y + 2][x + 1]
            },
            Shape::L => {
                board[y][x + 1] && board[y][x + 2] && board[y][x] && board[y + 1][x + 2] && board[y + 2][x + 2]
            },
            Shape::Vline => {
                board[y][x] && board[y + 1][x] && board[y + 2][x] && board[y + 3][x]
            },
            Shape::Square => {
                board[y][x] && board[y][x + 1] && board[y + 1][x] && board[y + 1][x + 1]
            },
        }
    }

    fn imprint(&self, board: &mut Board) {
        match self.shape {
           Shape::Hline => {
                board[self.y][self.x] = false;
                board[self.y][self.x + 1] = false;
                board[self.y][self.x + 2] = false;
                board[self.y][self.x + 3] = false;
            },
            Shape::Plus => {
                board[self.y][self.x + 1] = false;
                board[self.y + 1][self.x] = false;
                board[self.y + 1][self.x + 1] = false;
                board[self.y + 1][self.x + 2] = false;
                board[self.y + 2][self.x + 1] = false;
            },
            Shape::L => {
                board[self.y][self.x + 1] = false;
                board[self.y][self.x + 2] = false;
                board[self.y][self.x] = false;
                board[self.y + 1][self.x + 2] = false;
                board[self.y + 2][self.x + 2] = false;
            },
            Shape::Vline => {
                board[self.y][self.x] = false;
                board[self.y + 1][self.x] = false;
                board[self.y + 2][self.x] = false;
                board[self.y + 3][self.x] = false;
            },
            Shape::Square => {
                board[self.y][self.x] = false;
                board[self.y][self.x + 1] = false;
                board[self.y + 1][self.x] = false;
                board[self.y + 1][self.x + 1] = false;
            },
        }
    }

    fn max_y(&self) -> usize {
        match self.shape {
            Shape::Hline => self.y,
            Shape::Plus => self.y + 2,
            Shape::L => self.y + 2,
            Shape::Vline => self.y + 3,
            Shape::Square => self.y + 1,
        }
    }
    fn apply_wind(&mut self, board: &Board, dir: char) {
        match dir {
            '<' => if self.can_move_to(board, self.x-1, self.y) {
                        self.x -= 1;
                    },
            '>' => if self.can_move_to(board, self.x+1, self.y) {
                        self.x += 1;
                    },
            _ => panic!("Invalid wind direction.")
        }
    }

    fn try_move_down(&mut self, board: &Board) -> bool {
        if self.can_move_to(board, self.x, self.y-1) {
            self.y -= 1;
            true
        } else {
            false
        }
    }
}

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let wind_pattern = lines[0].clone().chars().into_iter().collect_vec();
    let mut board: Board = new_board();
    let shape_order = [Shape::Hline, Shape::Plus, Shape::L, Shape::Vline, Shape::Square];
    let mut max_y = 0;
    let mut wind_idx = 0;
    for block_id in 0..2022 {
        let mut piece = Piece {x: 3, y: max_y + 4, shape: shape_order[block_id % 5].clone()};
        loop {
            piece.apply_wind(&board, wind_pattern[wind_idx % wind_pattern.len()]);
            wind_idx += 1;
            if !piece.try_move_down(&board) {
                piece.imprint(&mut board);
                max_y = std::cmp::max(piece.max_y(), max_y);
                break;
            }
        }
    }

    Ok(max_y as u64)
}

fn are_pieces_series_congruent(cycle1: &[Piece], cycle2: &[Piece]) -> Option<usize> {
    if cycle1[0].y >= cycle2[0].y {
        return None
    }
    let y_distance = cycle2[0].y - cycle1[0].y;
    for (p1, p2) in cycle1.into_iter().zip(cycle2.into_iter()) {
        assert!(p1.shape == p2.shape);
        if p1.x != p2.x || p1.y + y_distance != p2.y {
            return None
        }
    };
    Some(y_distance)
}

fn has_cycle(pieces: &Vec<Piece>, min_cycle_length: usize) -> Option<((usize, usize))> {
    for cycle_len in (min_cycle_length..(pieces.len() /2)).step_by(5) {
        let cycle1 = &pieces[pieces.len() - cycle_len*2..pieces.len() - cycle_len];
        let cycle2 = &pieces[pieces.len() - cycle_len..];
        if let Some(y_distance) = are_pieces_series_congruent(cycle1, cycle2) {
            return Some((cycle_len, y_distance));
        }
    }
    None
}


fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let wind_pattern = lines[0].clone().chars().into_iter().collect_vec();
    let mut board: Board = new_board();
    let shape_order = [Shape::Hline, Shape::Plus, Shape::L, Shape::Vline, Shape::Square];
    let mut max_y = 0;
    let mut wind_idx = 0;
    let mut pieces_placed = vec![];
    for block_id in 0.. {
        let mut piece = Piece {x: 3, y: max_y + 4, shape: shape_order[block_id % 5].clone()};
        loop {
            piece.apply_wind(&board, wind_pattern[wind_idx % wind_pattern.len()]);
            wind_idx += 1;
            if !piece.try_move_down(&board) {
                piece.imprint(&mut board);
                max_y = std::cmp::max(piece.max_y(), max_y);
                pieces_placed.push(piece);
                break;
            }
        }
        if let Some((cycle_len, cycle_y_diff)) = has_cycle(&pieces_placed, wind_pattern.len() / 15 * 5) {
            let blocks_left = 1000000000000 - block_id - 1;
            let cycles_left = blocks_left / cycle_len;
            let blocks_rest = blocks_left - cycles_left * cycle_len;
            let last_cycle_start_y = pieces_placed[pieces_placed.len() - cycle_len].max_y();
            let rest_y_diff = pieces_placed[pieces_placed.len() - cycle_len + blocks_rest].max_y() - last_cycle_start_y;
            let res = max_y + cycles_left * cycle_y_diff + rest_y_diff;
            return Ok(res as u64)
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::task17::{task, task_part_two};

    #[test]
    fn exdample() {
        assert_eq!(task("example.txt").unwrap(), 3068);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 1514285714288);
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
