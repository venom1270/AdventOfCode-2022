use advent_of_code_2022::file_utils::read_lines;
use std::collections::HashSet;
use std::cmp::{max,min};

struct Sensor {
    position: (i32, i32),
    beacon: (i32, i32)
}

fn parse_tuple(s: String) -> (i32, i32) {
    let mut split = s.trim().split(",").fuse();
    (split.next().unwrap().parse().expect("Error parsing tuple 1"), split.next().unwrap().parse().expect("Error parsing tuple 2"))
}

fn parse_input(file_path: String) -> Vec<Sensor> {

    let mut sensors: Vec<Sensor> = Default::default();

    if let Ok(mut lines) = read_lines(file_path) {
        
    
        while let Some(s) = lines.next() {

            let l = s.expect("Error line");
            let mut split = l.split(' ').fuse();
            split.next();
            split.next();
            let sx_string = split.next().expect("Parse error at sensor x");
            let sx: i32 = sx_string[2..sx_string.len()-1].parse().expect("Error parsing sx");
            let sy_string = split.next().expect("Parse error at sensor y");
            let sy: i32 = sy_string[2..sy_string.len()-1].parse().expect("Error parsing sy");
            split.next();
            split.next();
            split.next();
            split.next();
            let bx_string = split.next().expect("Parse error at beacon x");
            let bx: i32 = bx_string[2..bx_string.len()-1].parse().expect("Error parsing bx");
            let by_string = split.next().expect("Parse error at beacon y");
            let by: i32 = by_string[2..].parse().expect("Error parsing by");

            sensors.push(Sensor { position: (sx, sy), beacon: (bx, by) });
        }

    }

    sensors

}


pub fn solution() {
    let file_path = String::from("src/day15/1.txt");

    let sensors = parse_input(file_path);

    const Y_ROW: i32 = 2000000;
    let mut coverage: HashSet<(i32, i32)> = Default::default();
    for s in sensors.iter() {
        //println!("Sensor at {} {}", s.position.0, s.position.1);
        let md = (s.beacon.0 - s.position.0).abs() + (s.beacon.1 - s.position.1).abs();
        let y_diff = (s.position.1 - Y_ROW).abs();
        if y_diff <= md {
            let y = Y_ROW;
            let x_bound = md-y_diff;
            for x in -x_bound..x_bound+1 {
                //println!("Inserting {} {}", s.position.0 + x, y);
                coverage.insert((s.position.0 + x, y));
            }
        }
        /*for y in -md..md+1 {
            let x_bound = md-y.abs();
            for x in -x_bound..x_bound+1 {
                //println!("Inserting {} {}", s.position.0 + x, s.position.1 + y);
                coverage.insert((s.position.0 + x, s.position.1 + y));
            }
        }*/
    }

    // Remove beacons from coverage
    for s in sensors.iter() {
        coverage.remove(&s.beacon);
    }
    // Count row
    let row_count = coverage.iter().filter(|p| p.1 == Y_ROW).count();
    println!("Coverage at row y={} is {}", Y_ROW, row_count);

    // Part 2
    const SIZE: usize = 4000000+1; 
    //let mut intervals: [Vec<(i32, i32)>; SIZE] = [(); SIZE].map(|_| Vec::new());
    let mut intervals = vec![Vec::new(); SIZE];
    for s in sensors.iter() {
        let md = (s.beacon.0 - s.position.0).abs() + (s.beacon.1 - s.position.1).abs();
        let y_bounds = (max(0, s.position.1 - md), min(SIZE as i32, s.position.1 + md));
        //println!("SENSONR {} {} with md {} | bounds {} {}", s.position.0, s.position.1, md, y_bounds.0, y_bounds.1);
        for y in y_bounds.0..y_bounds.1 {
            let mut x_bound = 0;
            if s.position.1 < y {
                x_bound = md - (y - s.position.1);
            } else {
                x_bound = md - (s.position.1 - y);
            }
            //println!("y {} | x bound  {}", y, x_bound);
            let mut x_interval = (s.position.0 - x_bound, s.position.0 + x_bound);
            let mut int_vec = &intervals[y as usize];
            let mut updated_vec: Vec<(i32, i32)> = Vec::new();
            let mut done: bool = false;
            for i in int_vec.iter() {
                if done {
                    updated_vec.push(*i);
                } else if x_interval.1 < i.0 {
                    // x_interval left of i - put to new vec
                    updated_vec.push(x_interval);
                    updated_vec.push(*i);
                    done = true;
                } else if x_interval.0 < i.0 && x_interval.1 >= i.0 && x_interval.1 <= i.1 {
                    //updated_vec.push((x_interval.0, i.1));
                    //done = true;
                    // x_interval overlaps on left side
                    x_interval.1 = i.1;
                } else if x_interval.0 >= i.0 && x_interval.0 <= i.1 && x_interval.1 >= i.1 {
                    x_interval.0 = i.0;
                } else if x_interval.0 >= i.0 && x_interval.1 <= i.1 {
                    // x_interval inside i
                    updated_vec.push(*i);
                    done = true;
                } else if x_interval.0 > i.1 {
                    // x_interval right of i
                    updated_vec.push(*i);
                } else {
                    //println!("{}, {} vs {},{}", x_interval.0, x_interval.1, i.0, i.1);
                    // x_interval is bigger than i
                }
            }
            if !done {
                updated_vec.push(x_interval);
            }
            intervals[y as usize] = updated_vec;
        }
    }
    for i in 0..intervals.len() {
        //println!("Interval {}", i);
        let mut last = -1;
        for int in &intervals[i] {
            //println!("({},{})", int.0, int.1);
            if last != -1 && last + 1 != int.0 {
                println!("Distress beacon found! x={} y={} | Tuning frequency: {}", last+1, i, (last+1) as u128 * 4000000 + i as u128);
            }
            last = int.1;
        }
    }


}



