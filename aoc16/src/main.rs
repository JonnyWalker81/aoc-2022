use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug, Clone, Default)]
struct Valve {
    name: String,
    rate: usize,
    leads_to: Vec<String>,
}

fn shortcuts(start: &str, tunnels: &HashMap<String, Valve>) -> HashMap<String, usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut paths = HashMap::new();

    visited.insert(start);
    queue.push_back((start, 0usize));

    while let Some((node, dist)) = queue.pop_front() {
        let v = tunnels.get(node).unwrap();

        for p in &v.leads_to {
            if !visited.insert(p) {
                continue;
            }

            let room = tunnels.get(p).unwrap();
            if room.rate > 0 && room.name != start {
                paths.insert(room.name.clone(), dist + 1);
            }

            queue.push_back((&room.name, dist + 1));
        }
    }

    paths
}

#[derive(Debug, Default, Eq, Clone)]
struct Walk {
    loc: String,
    remaining_time: i32,
    visited: HashSet<String>,
    helper: bool,
}

impl PartialEq for Walk {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc
            && self.helper == other.helper
            && self.remaining_time == other.remaining_time
            && self.visited == other.visited
    }
}

impl std::hash::Hash for Walk {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.loc.hash(state);
        self.helper.hash(state);
        self.remaining_time.hash(state);
        let mut v = self.visited.iter().collect::<Vec<&String>>();
        v.sort();
        for s in v {
            s.hash(state);
        }
    }
}

#[derive(Debug, Default)]
struct Search {
    visited: HashMap<Walk, usize>,
}

impl Search {
    fn bfs(
        &mut self,
        walk: &Walk,
        tunnels: &HashMap<String, Valve>,
        shortcuts: &HashMap<String, HashMap<String, usize>>,
    ) -> usize {
        if let Some(ans) = self.visited.get(walk) {
            return *ans;
        }

        if walk.remaining_time == 0 {
            return 0;
        }

        let mut max_flow = if walk.helper {
            self.bfs(
                &Walk {
                    loc: "AA".to_string(),
                    // remaining_time: walk.remaining_time,
                    remaining_time: 26,
                    visited: walk.visited.clone(),
                    helper: false,
                },
                tunnels,
                shortcuts,
            )
        } else {
            0
        };
        if !walk.visited.contains(&walk.loc) {
            let mut visited = walk.visited.clone();
            visited.insert(walk.loc.clone());

            let flow = tunnels.get(&walk.loc).unwrap().rate * (walk.remaining_time - 1) as usize;

            max_flow = max_flow.max(
                self.bfs(
                    &Walk {
                        loc: walk.loc.clone(),
                        remaining_time: walk.remaining_time - 1,
                        visited,
                        helper: walk.helper,
                    },
                    tunnels,
                    shortcuts,
                ) + flow,
            );
        }

        // let dest = &tunnels.get(&walk.loc).unwrap();

        // let lookup = shortcuts.get(&walk.loc).unwrap();
        for (dest, cost) in shortcuts.get(&walk.loc).unwrap() {
            // let cost = lookup.get(v).unwrap();
            if *cost < walk.remaining_time as usize {
                max_flow = max_flow.max(self.bfs(
                    &Walk {
                        loc: dest.to_string(),
                        remaining_time: walk.remaining_time - *cost as i32,
                        visited: walk.visited.clone(),
                        helper: walk.helper,
                    },
                    tunnels,
                    shortcuts,
                ));
            }
        }

        self.visited.insert(walk.clone(), max_flow);

        max_flow
    }
}

fn part1(input: &str) -> Result<()> {
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let name = parts[1].to_string();
            let rate_parts: Vec<&str> = parts[4].split("=").collect();
            let rate: usize = rate_parts[1].trim_end_matches(";").parse().unwrap();
            let valve_parts: Vec<String> = parts[9..]
                .to_vec()
                .iter()
                .map(|s| s.trim_end_matches(","))
                .map(|v| v.to_string())
                .collect();

            (
                name.clone(),
                Valve {
                    name,
                    rate,
                    leads_to: valve_parts,
                },
            )
        })
        .collect();

    let mut search = Search::default();
    let walker = Walk {
        loc: "AA".to_string(),
        remaining_time: 30,
        visited: HashSet::new(),
        helper: false,
    };

    let mut cache = HashMap::new();
    for v in valves.keys() {
        // println!("{v} -> shortcuts are {:?}", shortcuts(v, &valves));
        cache.insert(v.clone(), shortcuts(v, &valves));
    }

    let max_flow = search.bfs(&walker, &valves, &cache);

    // let mut stack = vec!["AA".to_string()];
    // let mut open_values = vec![];
    // let mut timeline = vec![0; 30];
    // let mut visited = HashSet::new();
    // let mut opened: HashMap<String, Valve> = HashMap::new();
    // for t in 0..30 {
    //     if stack.is_empty() {
    //         break;
    //     }
    //     let v = stack.pop().unwrap();
    //     let valve = &valves[&v];
    //     if visited.contains(&v) {
    //         continue;
    //     }
    //
    //     if valve.rate > 0 && !opened.is_empty() {
    //         open_values.push(valve.rate);
    //         for o in &opened {
    //             open_values.push(o.1.rate);
    //         }
    //         opened.clear();
    //     } else if !opened.contains_key(&valve.name) {
    //         opened.insert(valve.name.clone(), valve.clone());
    //     }
    //
    //     visited.insert(v);
    //
    //     timeline[t] = open_values.iter().sum();
    //     for lt in &valve.leads_to {
    //         if visited.contains(lt) {
    //             continue;
    //         }
    //
    //         stack.push(lt.to_string());
    //         break;
    //     }
    // }

    // println!("{timeline:?}");
    println!("Flow: {max_flow}");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let name = parts[1].to_string();
            let rate_parts: Vec<&str> = parts[4].split("=").collect();
            let rate: usize = rate_parts[1].trim_end_matches(";").parse().unwrap();
            let valve_parts: Vec<String> = parts[9..]
                .to_vec()
                .iter()
                .map(|s| s.trim_end_matches(","))
                .map(|v| v.to_string())
                .collect();

            (
                name.clone(),
                Valve {
                    name,
                    rate,
                    leads_to: valve_parts,
                },
            )
        })
        .collect();

    let mut search = Search::default();
    let walker = Walk {
        loc: "AA".to_string(),
        remaining_time: 26,
        visited: HashSet::new(),
        helper: true,
    };

    let mut cache = HashMap::new();
    for v in valves.keys() {
        // println!("{v} -> shortcuts are {:?}", shortcuts(v, &valves));
        cache.insert(v.clone(), shortcuts(v, &valves));
    }

    let max_flow = search.bfs(&walker, &valves, &cache);

    println!("Flow: {max_flow}");

    Ok(())
}
