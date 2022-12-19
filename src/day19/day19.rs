use advent_of_code_2022::file_utils::read_lines;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::{max,min};

enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode
}
struct Blueprint {
    id: u32,
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_cost: (u32, u32),
    geode_robot_cost: (u32, u32) 
}

struct State<'a> {
    blueprint: &'a Blueprint,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    time: u32,
    build_queue: Vec<RobotType>
}
impl<'a> State<'a> {
    pub fn new(blueprint: &'a Blueprint) -> Self {
        State {
            blueprint,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            time: 0,
            build_queue: Default::default()
        }
    }

    pub fn advance_time(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;

        for queue in self.build_queue.iter() {
            match queue {
                RobotType::Ore => self.ore_robots += 1,
                RobotType::Clay => self.clay_robots += 1,
                RobotType::Obsidian => self.obsidian_robots += 1,
                RobotType::Geode => self.geode_robots += 1
            }
        }
        self.build_queue.clear();
    }

    pub fn clone(&self) -> Self {
        State { 
            blueprint: self.blueprint, 
            ore_robots: self.ore_robots, 
            clay_robots: self.clay_robots, 
            obsidian_robots: self.obsidian_robots, 
            geode_robots: self.geode_robots, 
            ore: self.ore, 
            clay: self.clay, 
            obsidian: self.obsidian, 
            geode: self.geode, 
            time: self.time,
            build_queue: Default::default()
        }
    }

    pub fn print(&self) {
        println!("--- TIME: {}", self.time);
        println!("Ore: {}, Clay: {}, Obsidian: {}, Geode: {}", self.ore, self.clay, self.obsidian, self.geode);
        println!("ROBOTS: Ore: {}, Clay: {}, Obsidian: {}, Geode: {}", self.ore_robots, self.clay_robots, self.obsidian_robots, self.geode_robots);
    }
}

fn simulate(bp: &Blueprint, MINUTES: u32) -> u32{
    let mut q: VecDeque<State> = Default::default();
    let mut memo: HashMap<(u32, u32, u32, u32, u32, u32, u32, u32), u32> = Default::default();
    let mut memo2: HashMap<u32, u32> = Default::default();
    let mut memo3: HashMap<u32, Vec<(u32, u32, u32, u32, u32, u32, u32, u32)>> = Default::default();
    q.push_back(State::new(bp));

    let mut max_geode = 0;
    while let Some(s) = q.pop_back() {

        let memo_key = (s.ore, s.clay, s.obsidian, s.geode, s.ore_robots, s.clay_robots, s.obsidian_robots, s.geode_robots);
        if let Some(m) = memo.get(&memo_key) {
            if m <= &s.time {
                continue;
            }
        }
        memo.insert(memo_key, s.time);

        /*if let Some(m) = memo2.get(&s.time) {
            if m > &s.geode {
                continue;
            }
        }
        memo2.insert(s.time, s.geode);*/

        if let Some(mut m) = memo3.get_mut(&s.time) {
            let mut cont = false;
            for st in m.iter() {
                if s.geode <= st.0 && s.ore_robots <= st.1 && s.clay_robots <= st.2 && s.obsidian_robots <= st.3 && s.geode_robots <= st.4 && s.ore <= st.5 && s.clay <= st.6 && s.obsidian <= st.7 {
                    cont = true;
                    break;
                }
            }
            if cont {
                continue;
            }
            m.push((s.geode, s.ore_robots, s.clay_robots, s.obsidian_robots, s.geode_robots, s.ore, s.clay, s.obsidian));
        } else {
            memo3.insert(s.time, vec![(s.geode, s.ore_robots, s.clay_robots, s.obsidian_robots, s.geode_robots, s.ore, s.clay, s.obsidian)]);
        }

        let diff = MINUTES - s.time;
        let potential_geodes = (diff * (diff + 1)) / 2 + s.geode + diff * s.geode_robots;
        if potential_geodes <= max_geode {
            continue;
        }
        

        let mut states_to_add: Vec<State> = Default::default();
        // Each ore robot costs 4 ore.
        if s.ore >= s.blueprint.ore_robot_cost {
            let mut new_state = s.clone();
            new_state.ore -= s.blueprint.ore_robot_cost;
            new_state.build_queue.push(RobotType::Ore);
            states_to_add.push(new_state);
        }
        // Each clay robot costs 2 ore.
        if s.ore >= s.blueprint.clay_robot_cost {
            let mut new_state = s.clone();
            new_state.ore -= s.blueprint.clay_robot_cost;
            new_state.build_queue.push(RobotType::Clay);
            states_to_add.push(new_state);
        }
        // Each obsidian robot costs 3 ore and 14 clay.
        if s.ore >= s.blueprint.obsidian_robot_cost.0 && s.clay >= s.blueprint.obsidian_robot_cost.1 {
            let mut new_state = s.clone();
            new_state.ore -= s.blueprint.obsidian_robot_cost.0;
            new_state.clay -= s.blueprint.obsidian_robot_cost.1;
            new_state.build_queue.push(RobotType::Obsidian);
            states_to_add.push(new_state);
        }
        // Each geode robot costs 2 ore and 7 obsidian.
        if s.ore >= s.blueprint.geode_robot_cost.0 && s.obsidian >= s.blueprint.geode_robot_cost.1 {
            let mut new_state = s.clone();
            new_state.ore -= s.blueprint.geode_robot_cost.0;
            new_state.obsidian -= s.blueprint.geode_robot_cost.1;
            new_state.build_queue.push(RobotType::Geode);
            states_to_add.push(new_state);
        }

        states_to_add.push(s);
        for mut new_state in states_to_add {
            new_state.time += 1;
            if new_state.time > MINUTES {
                if max_geode <= new_state.geode {
                    max_geode = new_state.geode;
                    //new_state.print();
                }
            } else {
                new_state.advance_time();
                q.push_back(new_state);
            }
        }
    }

    max_geode

}

fn parse_input(file_path: String) -> Vec<Blueprint> {

    let mut blueprints: Vec<Blueprint> = Default::default(); 

    if let Ok(lines) = read_lines(file_path) {
        
        for line in lines {
            let l = line.expect("Error getting line");
            let mut s = l.split(' ').fuse();
            
            s.next();
            let id_s = s.next().expect("Error getting ID");
            let id = id_s[..id_s.len()-1].parse().expect("Error parsing ID");
            for _ in 0..4 {
                s.next();
            }
            let ore_robot_cost: u32 = s.next().expect("Error getting ore").parse().expect("Error parsing ore");
            for _ in 0..5 {
                s.next();
            }
            let clay_robot_cost: u32 = s.next().expect("Error getting clay").parse().expect("Error parsing clay");
            for _ in 0..5 {
                s.next();
            }
            let o1: u32 = s.next().expect("Error getting o1").parse().expect("Error parsing o1");
            for _ in 0..2 {
                s.next();
            }
            let o2: u32 = s.next().expect("Error getting o2").parse().expect("Error parsing o2");
            let obsidian_robot_cost = (o1, o2);
            for _ in 0..5 {
                s.next();
            }
            let g1: u32 = s.next().expect("Error getting g1").parse().expect("Error parsing g1");
            for _ in 0..2 {
                s.next();
            }
            let g2: u32 = s.next().expect("Error getting g2").parse().expect("Error parsing g2");
            let geode_robot_cost = (g1, g2);
            
            blueprints.push(Blueprint { id, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost });

        }
        
    }

    blueprints

}



pub fn solution() {
    let file_path = String::from("src/day19/1.txt");

    let blueprints = parse_input(file_path);
    let mut s = 0;
    
    
    println!("Running all blueprints with 24 minutes...");
    for blueprint in blueprints.iter() {
        //println!("BP {}, Costs: {} {} ({},{}), ({},{})", blueprint.id, blueprint.ore_robot_cost, blueprint.clay_robot_cost, blueprint.obsidian_robot_cost.0, blueprint.obsidian_robot_cost.1, blueprint.geode_robot_cost.0, blueprint.geode_robot_cost.1);
        let max_geodes = simulate(&blueprint, 24);
        let quality = max_geodes * blueprint.id;
        println!("Blueprint {} | Max geodes {} | Quality {}", blueprint.id, max_geodes, quality);
        s += quality;
    }

    println!("Total quality of blueprints: {}", s);

    s = 1;
    println!("Running first three blueprints with 32 minutes...");
    for i in 0..3 {
        //println!("BP {}, Costs: {} {} ({},{}), ({},{})", blueprint.id, blueprint.ore_robot_cost, blueprint.clay_robot_cost, blueprint.obsidian_robot_cost.0, blueprint.obsidian_robot_cost.1, blueprint.geode_robot_cost.0, blueprint.geode_robot_cost.1);
        let blueprint = blueprints.get(i).expect("Error getting blueprint");
        let max_geodes = simulate(&blueprint, 32);
        println!("Blueprint {} | Max geodes {}", blueprint.id, max_geodes);
        s *= max_geodes;
    }

    println!("Multiplied max geodes: {}", s);
    

}



