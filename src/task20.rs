use itertools::Itertools;

use crate::utils::parse_lines;

fn task(input_path: &str) -> Result<i64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let numbers = lines.into_iter().map(|s| s.parse::<i64>().unwrap()).collect_vec();
    let mut mixer = numbers.clone().into_iter().enumerate().collect_vec();
    for i in 0..mixer.len() {
        let pos = mixer.iter().position(|x| x.0 == i).unwrap();
        let removed_n = mixer.remove(pos);
        let new_pos = (pos as i64 + removed_n.1).rem_euclid((mixer.len()) as i64);
        mixer.insert(new_pos as usize, removed_n);
    }
    let pos_0 = mixer.iter().position(|x| x.1 == 0).unwrap();
    Ok([1000, 2000, 3000].iter().map(|i| mixer[(pos_0 + i) % mixer.len()].1).sum())
}

fn task_part_two(input_path: &str) -> Result<i64, String> {
        let lines: Vec<String> = parse_lines(input_path)?;
    let numbers = lines.into_iter().map(|s| s.parse::<i64>().unwrap()).collect_vec();
    let mut mixer = numbers.clone().into_iter().map(|n| n * 811589153).enumerate().collect_vec();
    for _ in 0..10 {
        for i in 0..mixer.len() {
            let pos = mixer.iter().position(|x| x.0 == i).unwrap();
            let removed_n = mixer.remove(pos);
            let new_pos = (pos as i64 + removed_n.1).rem_euclid((mixer.len()) as i64);
            mixer.insert(new_pos as usize, removed_n);
        }
    }
    let pos_0 = mixer.iter().position(|x| x.1 == 0).unwrap();
    Ok([1000, 2000, 3000].iter().map(|i| mixer[(pos_0 + i) % mixer.len()].1).sum())
}
 
#[cfg(test)]
mod tests {
    use crate::task20::{task, task_part_two};
 
    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 3);
    }
 
    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 1623178306);
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