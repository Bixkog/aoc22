use std::cmp::Ordering;
use itertools::Itertools;
use crate::utils::parse_lines;

type Json = serde_json::Value;

fn parse_packet(s: &str) -> Result<Json, String> {
    serde_json::from_str::<Json>(s).or(Err(format!("Can't parse: {}", s).to_string()))
}

fn packet_cmp(p1: &Json, p2: &Json) -> Ordering {
    match (p1, p2) {
        (Json::Number(n1), Json::Number(n2)) => 
            n1.as_u64().unwrap().cmp(&n2.as_u64().unwrap()) 
        ,
        (Json::Array(l1), Json::Array(l2)) => {
                let list_comp = l1.into_iter()
                    .zip(l2.into_iter())
                    .map(|(l1_p, l2_p)| packet_cmp(l1_p, l2_p))
                    .find_or_last(|res| *res != Ordering::Equal).or(Some(Ordering::Equal)).unwrap();
                match list_comp {
                    Ordering::Equal => l1.len().cmp(&l2.len()),
                    ord => ord
                }
            }
        ,
        (Json::Number(_), Json::Array(_)) => 
            packet_cmp(&Json::Array(vec![p1.clone()]), p2),
        (Json::Array(_), Json::Number(_)) => 
            packet_cmp(p1, &Json::Array(vec![p2.clone()])),
        _ => panic!("Invalid json object.")
    }
}

fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    Ok(lines.split(|s| s.is_empty())
        .map(|ps| Ok((
            parse_packet(ps[0].as_str())?,
            parse_packet(ps[1].as_str())?,
        )))
        .map_ok(|(p1, p2)| packet_cmp(&p1, &p2))
        .collect::<Result<Vec<_>, String>>()?.into_iter()
        .zip(1..)
        .filter_map(|(order, pos)| if order == Ordering::Less {Some(pos)} else {None})
        .sum()
    )
}

fn is_divider_packet(p: &Json) -> bool {
    *p == Json::Array(vec![Json::Array(vec![Json::Number(serde_json::Number::from(2))])])
    || *p == Json::Array(vec![Json::Array(vec![Json::Number(serde_json::Number::from(6))])])
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    Ok(lines
        .into_iter()
        .filter(|s| s.len() > 1)
        .map(|s| 
            parse_packet(s.as_str()).unwrap(),
        )
        .sorted_by(packet_cmp)
        .zip(1..)
        .filter(|(v, _)| is_divider_packet(v))
        .map(|t| t.1)
        .product())
        
}

#[cfg(test)]
mod tests {
    use crate::task13::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 13);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 140);
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
