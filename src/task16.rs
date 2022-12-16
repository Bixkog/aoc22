use core::time;
use std::{iter::FromIterator, collections::{HashMap, BinaryHeap}};

use itertools::Itertools;
use regex::Regex;

use crate::utils::parse_lines;

#[derive(Debug, Clone)]
struct Room {
    name: String,
    valve_pressure: u64,
    connected_to: HashMap<String, u64>,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? ((?:(?:, )?\w\w)+)").unwrap();
}

fn parse_room(line: String) -> Result<Room, String> {
    let captures = RE.captures_iter(line.as_str()).next().ok_or("Can't parse line".to_string())?;
    let mut c_iter = captures.iter().skip(1);
    println!("{:?}", captures);
    Ok(Room { 
        name: c_iter
            .next()
            .flatten()
            .ok_or("Name capture error.".to_string())?
            .as_str()
            .to_string(), 
        valve_pressure: c_iter
            .next()
            .flatten()
            .ok_or("Valve pressure capture error.".to_string())?
            .as_str()
            .parse::<u64>()
            .or(Err("Can't parse flow rate.".to_string()))?, 
        connected_to: c_iter
            .next()
            .flatten()
            .ok_or("Tunnels captures error.".to_string())?
            .as_str().to_string().split(", ")
            .map(|s| (String::from(s), 1))
            .collect()
    })
}

fn clear_0_pressure_rooms(rooms: Vec<Room>) -> HashMap<String, Room> {
    let mut rooms_map: HashMap<String, Room> = rooms.into_iter().map(|r| (r.name.clone(), r)).collect();
    for room_name in rooms_map.keys().cloned().collect_vec() {
        if rooms_map[&room_name].valve_pressure > 0 || room_name == "AA" { continue }
        let room0 = rooms_map.remove(&room_name).unwrap();
        for (_, c_room) in rooms_map.iter_mut() {
            if c_room.connected_to.contains_key(&room0.name) {
                let c_room_to_room0_dist = *c_room.connected_to.get(&room0.name).unwrap();
                for (new_tunnel, new_distance) in room0.connected_to.iter() {
                    if (!c_room.connected_to.contains_key(new_tunnel) && c_room.name != *new_tunnel) ||
                       (c_room.connected_to.contains_key(new_tunnel) && *c_room.connected_to.get(new_tunnel).unwrap() > new_distance + c_room_to_room0_dist) {
                        c_room.connected_to.insert(new_tunnel.to_string(), *new_distance + c_room_to_room0_dist);
                       }
                }
                c_room.connected_to.remove(&room0.name);
            }
        }
    }
    rooms_map
}

fn fill_rooms_distances(rooms: &HashMap<String, Room>, mut t_room: Room) -> Room {
    let mut new_distances = HashMap::from_iter(rooms.iter().map(|(name, _)| (name.clone(), u64::MAX)));
    let mut pq = BinaryHeap::from(vec![(0 as i64, t_room.name.clone())]);
    while let Some((d, t_room_name)) = pq.pop() {
        let d = -d as u64;
        let old_d = new_distances.get(&t_room_name).unwrap();
        if d > *old_d {continue};
        for (neighbour_name, n_d) in rooms.get(&t_room_name).unwrap().connected_to.iter() {
            if d + *n_d < *new_distances.get(neighbour_name).unwrap() {
                pq.push((-((d + *n_d) as i64), neighbour_name.clone()));
                *new_distances.get_mut(neighbour_name).unwrap() = d + *n_d;
            }
            
        }
    }
    t_room.connected_to = new_distances;
    t_room
}

fn fill_all_rooms_distances(rooms: HashMap<String, Room>) -> HashMap<String, Room> {
    rooms.iter().map(|(k, v)| (k.clone(), fill_rooms_distances(&rooms, v.clone()))).collect()
}


static mut best_score: u64 = 0;
static mut max_pressure: u64 = 0;

fn walk(rooms: &HashMap<String, Room>, current_room: &String, time_left: u64, released_pressure: u64, opened_valves: Vec<String>, score: u64) -> u64 {
    let mut max_score = score;
    for (target_room_name, distance) in rooms.get(current_room).unwrap()
            .connected_to.iter()
            .filter(|(_, d)| **d < time_left)
            .sorted_by_key(|t| (rooms.get(t.0).unwrap().valve_pressure as f64 / *t.1 as f64) as u64)
            .rev(){
        if !opened_valves.contains(target_room_name) {
            let new_released_pressure = released_pressure + rooms.get(target_room_name).unwrap().valve_pressure;
            unsafe {
                if score + released_pressure * (distance + 1) + 2 * new_released_pressure + max_pressure * (time_left - distance) >= best_score {
                     let walk_score = walk(rooms, 
                                                target_room_name, 
                                                time_left - distance - 1, 
                                                new_released_pressure,
                                                opened_valves.clone().into_iter().chain(vec![target_room_name.clone()]).collect(),
                                                score + released_pressure * (distance + 1));
                    if max_score < walk_score {
                        max_score = walk_score;
                    }
                }
            }
        }
    }
    if max_score == score {
        max_score += time_left * released_pressure;
    }
    unsafe {
        if max_score > best_score {
            best_score = max_score;
        }
    }
    max_score
}

fn walk2(rooms: &HashMap<String, Room>, 
        state: [(&String, u64); 2], 
        time_left: u64, 
        released_pressure: u64,
        opened_valves: Vec<String>, 
        score: u64
    ) -> u64 {
    let mut max_score = score;
    for i in 0..2 {
        if state[i].1 == time_left {
            for (target_room_name, distance) in rooms.get(state[i].0).unwrap()
                    .connected_to.iter()
                    .filter(|(_, d)| **d < time_left)
                    .filter(|(name, _)| !opened_valves.contains(name))
                    .sorted_by_key(|t| (rooms.get(t.0).unwrap().valve_pressure as f64 / *t.1 as f64) as u64)
                    .rev() {
                let min_distance;
                let new_released_pressure;
                if time_left - *distance - 1 > state[(i+1)%2].1 {
                    min_distance = *distance + 1;
                    new_released_pressure = released_pressure + rooms.get(target_room_name).unwrap().valve_pressure;
                } else {
                    min_distance = time_left - state[(i+1)%2].1;
                    new_released_pressure = released_pressure + rooms.get(state[(i+1)%2].0).unwrap().valve_pressure;
                }
                unsafe {
                if score + new_released_pressure * min_distance + released_pressure + max_pressure * (time_left - min_distance - 1) >= best_score {
                    let mut new_state = state.clone();
                    new_state[i] = (target_room_name, time_left - distance - 1);
                    let valve_score = walk2(rooms, 
                                    new_state,
                                    time_left - min_distance,
                                    new_released_pressure,
                                    opened_valves.clone().into_iter().chain(vec![target_room_name.clone()]).collect(),
                                    score + released_pressure * min_distance);
                    if max_score < valve_score {
                        max_score = valve_score;
                    }
                }
                }
            }
            break;
        }
    }
    if max_score == score {
        max_score += time_left * released_pressure;
        
    }
    unsafe {
        if max_score > best_score {
            best_score = max_score;
        }
    }
    max_score
}


fn task(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let rooms = lines.into_iter().map(parse_room).collect::<Result<Vec<_>, _>>()?;
    println!("{}", rooms.len());
    for r in rooms.iter() {
        println!("{:?}", r);
    }
    let rooms = clear_0_pressure_rooms(rooms);
    println!();
    println!("{}", rooms.len());
    for r in rooms.iter() {
        println!("{:?}", r);
    }

    let rooms = fill_all_rooms_distances(rooms);
    println!();
    println!("{}", rooms.len());
    for r in rooms.iter() {
        println!("{:?}", r);
    }
    unsafe{
        max_pressure = rooms.values().map(|r| r.valve_pressure).sum();
    }
    Ok(walk(&rooms, &"AA".to_string(), 30, 0, vec!["AA".to_string()], 0))
}

fn task_part_two(input_path: &str) -> Result<u64, String> {
    let lines: Vec<String> = parse_lines(input_path)?;
    let rooms = lines.into_iter().map(parse_room).collect::<Result<Vec<_>, _>>()?;
    println!("{}", rooms.len());
    for r in rooms.iter() {
        println!("{:?}", r);
    }
    let rooms = clear_0_pressure_rooms(rooms);
    println!();
    println!("{}", rooms.len());
    for r in rooms.iter() {
        println!("{:?}", r);
    }

    let rooms = fill_all_rooms_distances(rooms);
    println!();
    println!("{}", rooms.len());
    for r in rooms.iter() {
        println!("{:?}", r);
    }
    unsafe{
        max_pressure = rooms.values().map(|r| r.valve_pressure).sum();
    }
    Ok(walk2(&rooms, [(&"AA".to_string(), 26), (&"AA".to_string(), 26)], 26, 0, vec!["AA".to_string()], 0))
}

#[cfg(test)]
mod tests {
    use crate::task16::{task, task_part_two};

    #[test]
    fn example() {
        assert_eq!(task("example.txt").unwrap(), 1651);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(task_part_two("example.txt").unwrap(), 1707);
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
