use std::{str::FromStr, collections::HashMap};

use crate::utils::parse_lines;

#[derive(Clone, Debug)]
enum Node {
    Op(String, String, String),
    Num(i64)
}

fn evaluate(monkeys: &mut HashMap<String, Node>, name: String) -> i64 {
    match monkeys.get(&name).cloned().unwrap() {
        Node::Op(r1, op, r2) => {
            let n1 = evaluate(monkeys, r1);
            let n2 = evaluate(monkeys, r2);
            let res = match op.as_str() {
                "+" => n1 + n2,
                "-" => n1 - n2,
                "/" => n1 / n2,
                "*" => n1 * n2,
                _ => unimplemented!()
            };
            monkeys.insert(name.clone(), Node::Num(res));
            res
        },
        Node::Num(n) => n,
    }
}

fn task(input_path: &str) -> Result<i64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let mut monkeys: HashMap<String, Node> = lines
        .into_iter()
        .map(|s| s.split(" ").map(String::from_str).collect::<Result<Vec<String>, _>>().unwrap())
        .map(|mut v| {
            v[0].pop();
            match v.len() {
                2 => (v[0].clone(), Node::Num(v[1].parse().unwrap())),
                4 => (v[0].clone(), Node::Op(v[1].clone(), v[2].clone(), v[3].clone())),
                _ => unimplemented!()
            }
        }
        )
        .collect();
    Ok(evaluate(&mut monkeys, "root".to_string()))
}

fn evaluate2(monkeys: &HashMap<String, Node>, name: String, x: f64) -> f64 {
    if name == "humn" {
        return x
    }
    match monkeys.get(&name).cloned().unwrap() {
        Node::Op(r1, op, r2) => {
            let n1 = evaluate2(monkeys, r1, x);
            let n2 = evaluate2(monkeys, r2, x);
            let res = match op.as_str() {
                "+" => n1 + n2,
                "-" => n1 - n2,
                "/" => n1 / n2,
                "*" => n1 * n2,
                _ => unimplemented!()
            };
            res
        },
        Node::Num(n) => n as f64,
    }
}

fn task_part_two(input_path: &str) -> Result<i64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let mut monkeys: HashMap<String, Node> = lines
        .into_iter()
        .map(|s| s.split(" ").map(String::from_str).collect::<Result<Vec<String>, _>>().unwrap())
        .map(|mut v| {
            v[0].pop();
            match v.len() {
                2 => (v[0].clone(), Node::Num(v[1].parse().unwrap())),
                4 => (v[0].clone(), Node::Op(v[1].clone(), v[2].clone(), v[3].clone())),
                _ => unimplemented!()
            }
        }
        )
        .collect();
    if let Node::Op(c1, _, c2) = monkeys.get("root").unwrap() {
        monkeys.insert("root".to_string(), Node::Op(c1.clone(), "-".to_string(), c2.clone()));
    } else {
        return Err("Could not change root.".to_string());
    }
    let mut a = 0;
    let mut b = 3617613952376 * 2;
    while a + 1 != b {
        let p = (a + b) / 2;
        let res = evaluate2(&monkeys, "root".to_string(), p as f64);
        if res > 0. {
            a = p;
        } else if res < 0. {
            b = p;
        } else {
            a = p;
            break;
        }
    }
    println!("{}", evaluate2(&monkeys, "root".to_string(), a as f64));
    Ok(a)
}
 
#[cfg(test)]
mod tests {
    use crate::task21::{task, task_part_two};
 
    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 152);
    }
 
    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 301);
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