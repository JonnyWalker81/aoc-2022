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

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
enum Robot {
    #[default]
    Noop,
    Ore(i32),
    Clay(i32),
    Obsidian(i32, i32),
    Geode(i32, i32),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Blueprint {
    id: i32,
    ore: Robot,
    clay: Robot,
    obsidian: Robot,
    geode: Robot,
}

impl Blueprint {
    fn ore_cost(&self) -> i32 {
        match self.ore {
            Robot::Ore(c) => c,
            _ => 0,
        }
    }

    fn clay_cost(&self) -> i32 {
        match self.clay {
            Robot::Clay(c) => c,
            _ => 0,
        }
    }

    fn obsidian_cost(&self) -> (i32, i32) {
        match self.obsidian {
            Robot::Obsidian(o, c) => (o, c),
            _ => (0, 0),
        }
    }

    fn obsidian_ore_cost(&self) -> i32 {
        match self.obsidian {
            Robot::Obsidian(o, _) => o,
            _ => 0,
        }
    }

    fn obsidian_clay_cost(&self) -> i32 {
        match self.obsidian {
            Robot::Obsidian(_, c) => c,
            _ => 0,
        }
    }

    fn geode_cost(&self) -> (i32, i32) {
        match self.geode {
            Robot::Geode(ore, obs) => (ore, obs),
            _ => (0, 0),
        }
    }

    fn geode_ore_cost(&self) -> i32 {
        match self.geode {
            Robot::Geode(ore, _) => ore,
            _ => 0,
        }
    }

    fn geode_obsidian_cost(&self) -> i32 {
        match self.geode {
            Robot::Geode(_, obs) => obs,
            _ => 0,
        }
    }

    fn enough_for_ore_robot(&self, ore: i32) -> bool {
        ore >= self.ore_cost()
    }

    fn enough_for_clay_robot(&self, clay: i32) -> bool {
        clay >= self.clay_cost()
    }

    fn enough_for_obsidian_robot(&self, ore: i32, clay: i32) -> bool {
        let (ore_cost, clay_cost) = self.obsidian_cost();
        if ore_cost > 0 && clay_cost > 0 {
            ore >= ore_cost && clay >= clay_cost
        } else {
            false
        }
    }

    fn enough_for_geoode_robot(&self, ore: i32, obsidian: i32) -> bool {
        let (ore_cost, obsidian_cost) = self.geode_cost();
        if ore_cost > 0 && obsidian_cost > 0 {
            ore >= ore_cost && obsidian >= obsidian_cost
        } else {
            false
        }
    }
}

impl From<&str> for Blueprint {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split_whitespace().collect();
        let num: i32 = parts[1].trim_end_matches(":").parse().unwrap();
        let ore_cost: i32 = parts[6].parse().unwrap();
        let clay_cost: i32 = parts[12].parse().unwrap();
        let obs_ore_cost: i32 = parts[18].parse().unwrap();
        let obs_clay_cost: i32 = parts[21].parse().unwrap();
        let geode_ore_cost: i32 = parts[27].parse().unwrap();
        let geode_clay_cost: i32 = parts[30].parse().unwrap();

        Self {
            id: num,
            ore: Robot::Ore(ore_cost),
            clay: Robot::Clay(clay_cost),
            obsidian: Robot::Obsidian(obs_ore_cost, obs_clay_cost),
            geode: Robot::Geode(geode_ore_cost, geode_clay_cost),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let blueprints: Vec<Blueprint> = input.lines().map(|l| l.into()).collect();
    println!("{:?}", blueprints);
    println!("{:?}", blueprints.len());

    let mut quality = 0;
    for b in &blueprints {
        let max_geodes = work(b, 24);
        println!("{max_geodes:?}");
        quality += b.id * max_geodes
    }

    println!("Quality: {}", quality);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let blueprints: Vec<Blueprint> = input.lines().map(|l| l.into()).collect();
    println!("{:?}", blueprints);
    println!("{:?}", blueprints.len());

    let mut max = 1;
    for b in blueprints.iter().take(3) {
        let max_geodes = work(b, 32);
        println!("{max_geodes:?}");
        max *= max_geodes
    }

    println!("Quality: {}", max);
    Ok(())
}

fn work(blueprint: &Blueprint, time: i32) -> i32 {
    let sim = Simulation::new(blueprint, time);
    let mut visited = HashSet::new();
    let mut queue = vec![sim.clone()];

    let max_ore = blueprint.ore_cost().max(
        blueprint.clay_cost().max(
            blueprint
                .obsidian_ore_cost()
                .max(blueprint.geode_ore_cost()),
        ),
    );
    let max_clay = blueprint.obsidian_clay_cost();
    let max_obsidian = blueprint.geode_obsidian_cost();

    let mut best = 0;
    while let Some(state) = queue.pop() {
        if state.time == 0 {
            best = best.max(state.geode);
            continue;
        }

        if visited.contains(&state) {
            continue;
        }

        visited.insert(state.clone());

        let can_continue = state.ore_robots >= max_ore
            && state.clay_robots >= max_clay
            && state.obsidian_robots >= max_obsidian;
        // let can_continue = (state.ore_robots >= blueprint.geode_ore_cost()
        //     && state.obsidian_robots >= blueprint.geode_obsidian_cost())
        //     && (state.ore_robots >= blueprint.obsidian_ore_cost()
        //         && state.clay_robots >= blueprint.obsidian_clay_cost());
        // && state.clay_robots >= max_clay;

        if blueprint.enough_for_geoode_robot(state.ore, state.obsidian) {
            let mut c_state = state.tick();
            c_state.buy_geode_robot();
            queue.push(c_state);
            // if (state.ore_robots >= blueprint.geode_ore_cost()
            //     || state.ore_robots + state.ore >= blueprint.geode_ore_cost())
            //     && (state.obsidian_robots >= blueprint.geode_obsidian_cost()
            //         || state.obsidian_robots + state.obsidian >= blueprint.geode_obsidian_cost())
            // {
            if can_continue {
                continue;
            }
            // }
        }

        if blueprint.enough_for_obsidian_robot(state.ore, state.clay)
            && state.obsidian_robots < max_obsidian
        {
            let mut c_state = state.tick();
            c_state.buy_obsidian_robot();
            queue.push(c_state);

            // if (state.ore_robots >= blueprint.obsidian_ore_cost()
            //     || state.ore_robots + state.ore >= blueprint.obsidian_ore_cost())
            // && (state.clay_robots >= blueprint.obsidian_clay_cost()
            //     || state.clay_robots + state.clay >= blueprint.obsidian_clay_cost())
            // {
            if can_continue {
                continue;
            }
            // }
        }

        queue.push(state.tick());
        if state.blueprint.enough_for_clay_robot(state.ore) && state.clay_robots < max_clay {
            let mut c_state = state.tick();
            c_state.buy_clay_robot();
            queue.push(c_state);
        }

        if state.blueprint.enough_for_ore_robot(state.ore) && state.ore_robots < max_ore {
            let mut c_state = state.tick();
            c_state.buy_ore_robot();
            queue.push(c_state);
        }
    }

    best
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Simulation<'a> {
    blueprint: &'a Blueprint,
    time: i32,
    ore: i32,
    ore_robots: i32,
    clay: i32,
    clay_robots: i32,
    obsidian: i32,
    obsidian_robots: i32,
    geode: i32,
    geode_robots: i32,
}

impl<'a> Simulation<'a> {
    fn new(blueprint: &'a Blueprint, time: i32) -> Self {
        Self {
            blueprint,
            time,
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geode: 0,
            geode_robots: 0,
        }
    }

    // fn simulate(&mut self) {
    //     for t in 0..24 {
    //         self.tick();
    //     }
    // }

    fn tick(&self) -> Self {
        // if self
        //     .blueprint
        //     .enough_for_geoode_robot(self.ore, self.obsidian)
        // {
        //     self.buy_geode_robot()
        // } else if self
        //     .blueprint
        //     .enough_for_obsidian_robot(self.ore, self.clay)
        //     && self.obsidian < self.blueprint.obsidian_clay_cost()
        // {
        //     self.buy_obsidian_robot()
        // } else if self.blueprint.enough_for_clay_robot(self.ore)
        //     && self.clay < self.blueprint.clay_cost()
        // {
        //     self.buy_clay_robot();
        // } else if self.blueprint.enough_for_ore_robot(self.ore) {
        // }

        let mut s = self.clone();
        s.ore += self.ore_robots;
        s.clay += self.clay_robots;
        s.obsidian += self.obsidian_robots;
        s.geode += self.geode_robots;

        s.time -= 1;

        s
    }

    fn buy_ore_robot(&mut self) {
        self.ore -= self.blueprint.ore_cost();
        self.ore_robots += 1;
    }

    fn buy_clay_robot(&mut self) {
        self.ore -= self.blueprint.clay_cost();
        self.clay_robots += 1;
    }

    fn buy_obsidian_robot(&mut self) {
        let (ore, clay) = self.blueprint.obsidian_cost();
        self.ore -= ore;
        self.clay -= clay;
        self.obsidian_robots += 1;
    }

    fn buy_geode_robot(&mut self) {
        let (ore, obsidian) = self.blueprint.geode_cost();
        self.ore -= ore;
        self.obsidian -= obsidian;
        self.geode_robots += 1;
    }
}
