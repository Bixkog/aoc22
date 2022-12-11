use itertools::Itertools;
use crate::utils::parse_lines;

fn task(input_path: &str) -> Result<i64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let mut cycle = 0;
    let mut register = 1;
    let mut signal_sum = 0;
    let nop = &mut |register| {
        cycle += 1;
        if cycle % 40 == 20 {
            signal_sum += cycle * register;
        }
    };
    for line in lines {
        let splitted_line = line.split(" ").collect_vec();
        if splitted_line.len() == 1 {
            nop(register);
        } else {
            nop(register);
            nop(register);
            let add_arg: i64 = splitted_line[1].parse().or(Err("Can't parse add arg."))?;
            register += add_arg;
        }
    }
    Ok(signal_sum)
}

fn task_part_two(input_path: &str) -> Result<i64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let mut img = vec!['.'; 240];
    let mut cycle: i64 = 0;
    let mut register: i64 = 1;
    let nop = &mut |register: i64| {
        if (register - cycle % 40).abs() <= 1 {
            img[cycle as usize] = '#';
        }
        cycle += 1;

    };
    for line in lines {
        let splitted_line = line.split(" ").collect_vec();
        if splitted_line.len() == 1 {
            nop(register);
        } else {
            nop(register);
            nop(register);
            let add_arg: i64 = splitted_line[1].parse().or(Err("Can't parse add arg."))?;
            register += add_arg;
        }
    }
    for row in 0..6 {
        println!("{}", img[40*row..(40*(row+1))].into_iter().collect::<String>())
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::task10::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 13140);
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
