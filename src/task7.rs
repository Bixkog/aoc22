use itertools::Itertools;
use std::{fs::read_to_string, collections::HashMap};

use crate::task7_parsing::{Command, parse_commands, LsRow};

fn get_dir_sizes(commands: Vec<Command>) -> Result<HashMap<String, u64>, String> {
    let mut c_path: Vec<String> = vec![];
    let mut dir_size: HashMap<String, u64> = HashMap::new();
    dir_size.insert(String::new(), 0);
    for command in commands {
        match command {
            Command::Cd { arg } => {
                match arg.as_str() {
                    "/" => c_path.clear(),
                    ".." => drop(c_path.pop().ok_or("Can't pop on empty c_path.")?),
                    dir => c_path.push(dir.to_string()),
                    
                };
            }
            Command::Ls { output } => {
                for ls_row in output {
                    match ls_row {
                        LsRow::File { file_size, file_name: _ } => {
                            let mut dir_path = String::new();
                            *dir_size.get_mut(&dir_path).unwrap() += file_size;
                            for dir in c_path.iter() {
                                dir_path = dir_path + "/" + dir;
                                if !dir_size.contains_key(&dir_path) {
                                    dir_size.insert(dir_path.clone(), 0);
                                }
                                *dir_size.get_mut(&dir_path).unwrap() += file_size;
                            }
                            
                        },
                        LsRow::Dir { dir_name: _ } => {},
                    }
                }
            }
        }
    }
    Ok(dir_size)
}

fn task(input_path: &str) -> Result<u64, String> {
    let lines: String = read_to_string(input_path).or(Err("Can't read input file.".to_string()))?;
    let (_, commands) = parse_commands(lines.as_str()).unwrap();

    let dir_sizes = get_dir_sizes(commands)?;
    Ok(dir_sizes.into_values().fold(0, |s, v| {
        if v <= 100000 {
            s + v
        } else {
            s
        }
    }))
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: String = read_to_string(input_path).or(Err("Can't read input file.".to_string()))?;
    let (_, commands) = parse_commands(lines.as_str()).unwrap();

    let dir_sizes = get_dir_sizes(commands)?;
    let root_size = dir_sizes.get(&String::new()).ok_or("Root dir not found in dir sizes.".to_string())?;

    let total_space = 7 * u64::pow(10, 7);
    let free_space_needed = 3 * u64::pow(10, 7);

    let min_dir_to_delete_size = free_space_needed - (total_space - root_size);

    dir_sizes.into_values().filter(|v| *v > min_dir_to_delete_size)
        .min().ok_or("Could not find dir to delete.".to_string())
}

#[cfg(test)]
mod tests {
    use crate::task7::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 95437);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 24933642);
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
