use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use regex::Regex;
use itertools::Itertools;

static DAY: u8 = 16;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", most_pressure(&input, false));
    println!("{DAY}b: {}", most_pressure(&input, true));
}

#[derive(PartialEq,Eq)]
struct Valve {
    name: String,
    flowrate: usize,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(input: &str) -> Valve {
        let re = Regex::new("Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();

        let caps = re.captures(input).expect("input should match regex");
        let name = String::from(&caps[1]);
        let flowrate = caps[2].parse().unwrap();
        let tunnels = caps[3].split(", ").map(String::from).collect();

        Valve { name, flowrate, tunnels }
    }
}


#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: String,
}

/* comparator for priority queue */
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/* Dijkstra implementation from https://doc.rust-lang.org/std/collections/binary_heap/index.html */
fn dijkstra(map: &HashMap<String,Valve>, from: &str, to: &str) -> usize {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    for pos in map.keys() {
        if pos != from {
            dist.insert(pos.to_string(), usize::MAX);
        }
    }

    dist.insert(from.to_string(), 0);
    heap.push(State { cost: 0, pos: from.to_string() });

    while let Some(State { cost, pos }) = heap.pop() {
        if pos == to { return cost; }
        if cost > dist[&pos] { continue; }

        for neigh in &map[&pos].tunnels {
            let next = State { cost: cost + 1, pos: neigh.to_string() };
            if next.cost < dist[neigh] {
                dist.insert(neigh.to_string(), next.cost);
                heap.push(next);
            }
        }
    }

    panic!("no path found");
}

fn pressure_from(map: &HashMap<&String,&Valve>, distances: &HashMap<(String,String),usize>, from: &str, time: usize, opened: &HashSet<String>) -> usize {
    let valves_with_flowrate = map.values()
                                  .filter(|v| v.flowrate > 0)
                                  .map(|v| v.name.clone())
                                  .collect::<Vec<_>>();

    let mut max_pressure = 0;
    for to in valves_with_flowrate {
        if to == from || opened.contains(&to) {
            /* no need to travel here if valve already open */
            continue;
        }
        let distance = distances[&(from.to_string(),to.clone())];
        if distance >= time {
            /* no time left to open another valve at destination */
            continue;
        }

        let new_from = to.clone();

        let mut time = time;
        time -= 1;  // 1 minute to open valve
        time -= distance;  // time to travel to next valve
        let mut opened = opened.clone();
        opened.insert(new_from.clone());
        let mut pressure = pressure_from(map, distances, &new_from, time, &opened);
        pressure += time * map[&new_from].flowrate;
        max_pressure = max_pressure.max(pressure);
    }

    max_pressure
}

fn most_pressure(input: &[String], with_elephant: bool) -> usize {
    let map = input.iter()
                   .map(|x| Valve::new(x))
                   .map(|x| (x.name.clone(), x))
                   .collect::<HashMap<String,Valve>>();

    let start = "AA";
    let mut distances = HashMap::new();

    let valves_with_flowrate = map.values()
                                  .filter(|v| v.flowrate > 0)
                                  .collect::<Vec<_>>();

    for valve_from in valves_with_flowrate.iter().chain([&map[start]].iter()) {
        for valve_to in &valves_with_flowrate {
            if valve_from.name == valve_to.name {
                continue;
            }
            let distance = dijkstra(&map, &valve_from.name, &valve_to.name);
            distances.insert((valve_from.name.clone(), valve_to.name.clone()), distance);
        }
    }
    let opened = HashSet::new();
    let time = match with_elephant {
        false => 30,
        true => 26,
    };
    if !with_elephant {
        let map = map.iter().collect();
        return pressure_from(&map, &distances, start, time, &opened)
    }

    let mut max_pressure = 0;
    for path in valves_with_flowrate.iter().cloned().combinations(valves_with_flowrate.len() / 2) {
        let map_you = map.iter()
                         .filter(|&(_,valve)| path.contains(&valve))
                         .collect();
        let map_elephant = map.iter()
                              .filter(|&(_,valve)| !path.contains(&valve))
                              .collect();

        let pressure_you = pressure_from(&map_you, &distances, start, time, &opened);
        let pressure_elephant = pressure_from(&map_elephant, &distances, start, time, &opened);
        max_pressure = max_pressure.max(pressure_you + pressure_elephant);
    }
    max_pressure
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(most_pressure(&input, false), 1651);
        assert_eq!(most_pressure(&input, true), 1707);
    }
}
