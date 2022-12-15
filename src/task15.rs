use itertools::Itertools;
use regex::Regex;
use itertools::FoldWhile::{Continue, Done};
use crate::utils::parse_lines;
use tqdm::tqdm;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(-?\d+)").unwrap();
}

struct Sensor {
    x: i64, y: i64,
    b_x: i64, b_y: i64
}

fn parse_sensor(line: String) -> Result<Sensor, String> {
    let positions = RE.captures_iter(line.as_str())
        .map(|capt| capt[0].parse::<i64>().or(Err("Can't parse position.".to_string())))
        .collect::<Result<Vec<i64>, String>>()?;
    Ok(Sensor {
        x: positions[0],
        y: positions[1],
        b_x: positions[2],
        b_y: positions[3],
    })
}

fn get_range(sensor: &Sensor, query_y: i64, with_beacon: bool) -> Result<(i64, i64), String> {
    let distance = (sensor.x-sensor.b_x).abs() + (sensor.y-sensor.b_y).abs();
    let left = sensor.x - (distance - (sensor.y - query_y).abs());
    let right = sensor.x + (distance - (sensor.y - query_y).abs() + 1);
    if !with_beacon {
        if sensor.b_x == left && sensor.b_y == query_y {
            return Ok((left + 1,  right))
        } else if sensor.b_x == right - 1 && sensor.b_y == query_y {
            return Ok((left, right - 1))
        }
    }
    Ok((left, right))
}

fn task(input_path: &str, query_y: i64) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let sensors = lines.into_iter().map(parse_sensor).collect::<Result<Vec<Sensor>, String>>()?;
    Ok(sensors.iter()
        .map(|sensor| get_range(sensor, query_y, false))
        .collect::<Result<Vec<(i64, i64)>, String>>()?.into_iter()
        .filter(|(l, r)| l < r)
        .sorted()
        .chain(vec![(i64::MAX, i64::MAX)])
        .fold((0, (i64::MIN, i64::MIN)), |(sum, prev_range), range| {
            if prev_range.1 <= range.0 {
                (sum + prev_range.1 - prev_range.0, range)
            } else {
                (sum, (prev_range.0, std::cmp::max(prev_range.1, range.1)))
            }
        }).0 as u64)
    
}

fn task_part_two(input_path: &str, size: i64) -> Result<i64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let sensors = lines.into_iter().map(parse_sensor).collect::<Result<Vec<Sensor>, String>>()?;
    tqdm(0..size)
        .map(|query_y| 
            Ok::<(i64, i64), String>((
                sensors
                .iter()
                .map(|sensor| get_range(sensor, query_y, true))
                .collect::<Result<Vec<(i64, i64)>, String>>()?.into_iter()
                .filter(|(l, r)| l < r)
                .sorted()
                .chain(vec![(size+1, size+1)])
                .fold_while((-1, 0), |(empty_x, prev_right), range| {
                    if prev_right < range.0 {
                        Done((prev_right, prev_right))
                    } else {
                        Continue((empty_x, std::cmp::max(prev_right, range.1)))
                    }
                }).into_inner().0, query_y))
        )
        .filter_ok(|(x, _)| *x != -1)
        .map_ok(|(x, y)| x * 4000000 + y)
        .next().ok_or("Can't find empty slot.".to_string())?
    
}

#[cfg(test)]
mod tests {
    use crate::task15::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt", 10).unwrap(), 26);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt", 20).unwrap(), 56000011);
    }

    #[test]
    fn target() {
        println!("{}", task("input.txt", 2000000).unwrap());
    }

    #[test]
    fn target_part_two() {
        println!("{}", task_part_two("input.txt", 4000000).unwrap());
    }
}
