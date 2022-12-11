use crate::utils::parse_lines;

#[derive(Clone)]
struct Monke {
    pub items: Vec<u64>,
    op: fn(u64) -> u64,
    div_test: u64,
    targets: [usize; 2]
}

impl Monke {
    fn round(&mut self) -> Vec<(usize, u64)> {
        let res = self.items.iter().cloned()
        .map(self.op)
        .map(|x| x / 3)
        .map(|level| {
            if level % self.div_test == 0 {
                (self.targets[0], level)
            } else {
                (self.targets[1], level)
            }
        }).collect();
        self.items.clear();
        res
    }

    fn round2(&mut self) -> Vec<(usize, u64)> {
        let res = self.items.iter().cloned()
        .map(self.op)
        // Operate on ring to keep values in range
        // INPUT: 9699690 = LCM(19, 2, 3, 17, 13, 7, 5, 11)
        // EXAMPLE: 96577 = LCM(23, 19, 13, 17)
        .map(|x| x % 9699690)
        .map(|level| {
            if level % self.div_test == 0 {
                (self.targets[0], level)
            } else {
                (self.targets[1], level)
            }
        }).collect();
        self.items.clear();
        res
    }
}


lazy_static! {
    static ref EXAMPLE: Vec<Monke> = vec![
        Monke{items: vec![79, 98], op: |x| x * 19, div_test: 23, targets: [2, 3]},
        Monke{items: vec![54, 65, 75, 74], op: |x| x + 6, div_test: 19, targets: [2, 0]},
        Monke{items: vec![79, 60, 97], op: |x| x * x, div_test: 13, targets: [1, 3]},
        Monke{items: vec![74], op: |x| x + 3, div_test: 17, targets: [0, 1]},
    ];

    static ref INPUT: Vec<Monke> = vec![
        Monke{items: vec![74, 73, 57, 77, 74], op: |x| x * 11, div_test: 19, targets: [6, 7]},
        Monke{items: vec![99, 77, 79], op: |x| x + 8, div_test: 2, targets: [6, 0]},
        Monke{items: vec![64, 67, 50, 96, 89, 82, 82], op: |x| x + 1, div_test: 3, targets: [5, 3]},
        Monke{items: vec![88], op: |x| x * 7, div_test: 17, targets: [5, 4]},
        Monke{items: vec![80, 66, 98, 83, 70, 63, 57, 66], op: |x| x + 4, div_test: 13, targets: [0, 1]},
        Monke{items: vec![81, 93, 90, 61, 62, 64], op: |x| x + 7, div_test: 7, targets: [1, 4]},
        Monke{items: vec![69, 97, 88, 93], op: |x| x * x, div_test: 5, targets: [7, 2]},
        Monke{items: vec![59, 80], op: |x| x + 6, div_test: 11, targets: [2, 3]},
    ];
}

fn task(mut monkes: Vec<Monke>) -> Result<u64, String> {
    let mut monke_score = vec![0; monkes.len()];
    for _ in 0..20 {
        for monke_idx in 0..(monkes.len()) {
            let monke_throw = monkes[monke_idx].round();
            monke_score[monke_idx] += monke_throw.len();
            for (target, item) in monke_throw {
                monkes[target].items.push(item);
            }
        }
    }
    monke_score.sort();
    Ok((monke_score[monke_score.len() - 1] * monke_score[monke_score.len() - 2]) as u64)
}

fn task_part_two(mut monkes: Vec<Monke>) -> Result<u64, String> {
    let mut monke_score = vec![0; monkes.len()];
    for _ in 0..10000 {
        for monke_idx in 0..(monkes.len()) {
            let monke_throw = monkes[monke_idx].round2();
            monke_score[monke_idx] += monke_throw.len();
            for (target, item) in monke_throw {
                monkes[target].items.push(item);
            }
        }
    }
    monke_score.sort();
    Ok((monke_score[monke_score.len() - 1] * monke_score[monke_score.len() - 2]) as u64)
}

#[cfg(test)]
mod tests {
    use crate::task11::{task, task_part_two, EXAMPLE, INPUT};

    #[test]
    fn example() {
        assert_eq!(task(EXAMPLE.to_vec()).unwrap(), 10605);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two(EXAMPLE.to_vec()).unwrap(), 2713310158);
    }

    #[test]
    fn target() {
        println!("{}", task(INPUT.to_vec()).unwrap());
    }

    #[test]
    fn target_part_two() {
        println!("{}", task_part_two(INPUT.to_vec()).unwrap());
    }
}
