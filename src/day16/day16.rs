use advent_of_code_2022::file_utils::read_lines;
use std::collections::HashMap;
use std::cmp::{max,min};

struct Valve {
    name: String,
    flow_rate: u32,
    neighbours: Vec<String>
}

fn parse_input(file_path: String) -> HashMap<String, Valve> {

    let mut valves_map: HashMap<String, Valve> = Default::default();

    if let Ok(mut lines) = read_lines(file_path) {
        
    
        while let Some(s) = lines.next() {

            let l = s.expect("Error line");
            let mut split = l.split(' ').fuse();
            split.next();
            let valve_name = split.next().expect("Parse error at valve name");
            split.next();
            split.next();
            let flow_rate_string = split.next().expect("Parse error at flow rate");
            let flow_rate: u32 = flow_rate_string[5..flow_rate_string.len()-1].parse().expect("Error parsing flow_rate_string");
            split.next();
            split.next();
            split.next();
            split.next();
            let mut neighbours: Vec<String> = Default::default(); 
            for n in split {
                if n.ends_with(",") {
                    neighbours.push(n[..n.len()-1].to_string());
                } else {
                    neighbours.push(n.to_string());
                }
            }

            valves_map.insert(valve_name.to_string(), Valve { name: valve_name.to_string(), flow_rate, neighbours });
        }

    }

    valves_map

}


fn release_pressure(valves: &HashMap<String, Valve>, opened: &mut HashMap<String, bool>, current: String, time: i32, mut pressure: u32, memo: &mut HashMap<(String, i32), u32>) -> (u32, HashMap<String, bool>) {
    if time == 0 {
        return (pressure, opened.clone());
    }
    if time < 0 {
        return (0, Default::default());
    }
    if let Some(val) = memo.get(&(current.to_string(), time)) {
        if val >= &pressure {
            return (0, Default::default());
        }
    }
    memo.insert((current.to_string(), time), pressure);
    //println!("{}", current);

    let current_valve = valves.get(&current).unwrap();
    let mut max_pressure = 0;
    let mut final_opened = Default::default();
    
    // Don't open current valve
    for neighbour in current_valve.neighbours.iter() {
        let (mp, fo) = release_pressure(valves, opened, neighbour.to_string(), time-1, pressure, memo);
        //max_pressure = max(max_pressure, mp);
        if mp > max_pressure {
            max_pressure = mp;
            final_opened = fo;
        }
    }
    
    // Open current valve
    if !opened.get(&current).unwrap() {
        //println!("OPENENING {}", current);
        opened.insert(current.to_string(), true);
        let valve_pressure = (time-1) as u32 * current_valve.flow_rate;
        for neighbour in &current_valve.neighbours {
            //max_pressure = max(max_pressure, release_pressure(valves, opened, neighbour.to_string(), time-2, pressure+valve_pressure, memo));
            let (mp, fo) = release_pressure(valves, opened, neighbour.to_string(), time-2, pressure+valve_pressure, memo);
            if mp > max_pressure {
                max_pressure = mp;
                final_opened = fo;
            }
            /*max_pressure = max(max_pressure, mp);
            final_opened = fo;*/
        }
        opened.insert(current, false);
    } 

    return (max_pressure, final_opened);

}

fn release_pressure_elephant(valves: &HashMap<String, Valve>, opened: &mut HashMap<String, bool>, me: String, elephant: String, time: i32, mut pressure: u32, memo: &mut HashMap<(String, String, i32), u32>) -> u32{
    if time == 0 {
        return pressure;
    }
    if time < 0 {
        return 0;
    }
    
    if let Some(val) = memo.get(&(me.to_string(), elephant.to_string(), time)) {
        if val >= &pressure {
            return 0;
        }
    }
    let opened_count = opened.iter().filter(|o| *o.1).count();
    memo.insert((me.to_string(), elephant.to_string(), time), pressure);
    memo.insert((elephant.to_string(), me.to_string(), time), pressure);

    // Return if every valve open
    if opened_count == valves.len() {
        return pressure;
    }

    let my_valve = valves.get(&me).unwrap();
    let elephant_valve = valves.get(&elephant).unwrap();
    let mut max_pressure = 0;
    

    // We have 4 options:
    // - nobody opens valve - just move to neighbours (me and elephant)
    // - both of us open valve - me and elephant
    // - only I open valve, elephant moves
    // - only elephant opens valve, I move

    // Open current valve (both)          
    if me != elephant {
        if !opened.get(&me.to_string()).unwrap() && !opened.get(&elephant.to_string()).unwrap() && my_valve.flow_rate != 0 && elephant_valve.flow_rate != 0 {
            opened.insert(me.to_string(), true);
            opened.insert(elephant.to_string(), true);
            let pressure_increase = (time-1) as u32 * (my_valve.flow_rate + elephant_valve.flow_rate);
                
            max_pressure = max(max_pressure, release_pressure_elephant(
                valves, 
                opened, 
                me.to_string(),
                elephant.to_string(),
                time-1, 
                pressure + pressure_increase, 
                memo)
            );
    
            opened.insert(elephant.to_string(), false);
            opened.insert(me.to_string(), false);
        } 
    }

    // Don't open current valve - move to neighbours (both)
    for my_neighbour in my_valve.neighbours.iter() {
        for elephant_neighbour in elephant_valve.neighbours.iter() {

            max_pressure = max(max_pressure, release_pressure_elephant(
                valves, 
                opened, 
                my_neighbour.to_string(), 
                elephant_neighbour.to_string(), 
                time-1, 
                pressure, 
                memo)
            );
        }
    }

    // Open valve - me only - elephant moves
    let mut pressure_increase = 0;
    if my_valve.flow_rate != 0 && !opened.get(&me.to_string()).unwrap() {
        opened.insert(me.to_string(), true);
        pressure_increase = (time-1) as u32 * my_valve.flow_rate;

        for elephant_neighbour in elephant_valve.neighbours.iter() {
            max_pressure = max(max_pressure, release_pressure_elephant(
                valves, 
                opened, 
                me.to_string(), 
                elephant_neighbour.to_string(), 
                time-1, 
                pressure + pressure_increase, 
                memo)
            );
        }

        opened.insert(me.to_string(), false);
    }


    // Open valve - elephant only - I move
    let mut pressure_increase = 0;
    if elephant_valve.flow_rate != 0 && !opened.get(&elephant.to_string()).unwrap() {
        opened.insert(elephant.to_string(), true);
        pressure_increase = (time-1) as u32 * elephant_valve.flow_rate;


        for my_neighbour in my_valve.neighbours.iter() {
            max_pressure = max(max_pressure, release_pressure_elephant(
                valves, 
                opened, 
                my_neighbour.to_string(), 
                elephant.to_string(), 
                time-1, 
                pressure + pressure_increase, 
                memo)
            );
        }

        opened.insert(elephant.to_string(), false);
    }
    
    

    return max_pressure;

}

pub fn solution() {
    let file_path = String::from("src/day16/1.txt");

    let valves = parse_input(file_path);
    let mut opened: HashMap<String, bool> = Default::default();
    for v in valves.iter() {
        opened.insert(v.0.to_string(), false);
    }

    let memo = &mut Default::default();
    let (max_pressure, op) = release_pressure(&valves, &mut opened, "AA".to_string(), 30, 0, memo);
    println!("Max pressure: {}", max_pressure);

    // Reset for part 2
    memo.clear();
    opened.clear();
    for v in valves.iter() {
        opened.insert(v.0.to_string(), false);
    }
    let memo_elephant = &mut Default::default();
    println!("Max pressure with elephant help: {}", release_pressure_elephant(&valves, &mut opened, "AA".to_string(), "AA".to_string(), 26, 0, memo_elephant));
    // 2506, 2556, 2580 - too low 



}



