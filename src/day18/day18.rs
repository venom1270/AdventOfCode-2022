use advent_of_code_2022::file_utils::read_lines;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::{max,min};

fn parse_input(file_path: String) -> Vec<(i32, i32, i32)> {

    let mut points: Vec<(i32, i32, i32)> = Default::default(); 

    if let Ok(mut lines) = read_lines(file_path) {
        
        for line in lines {
            let l = line.expect("Error getting line");
            let mut s = l.split(',').fuse();
            let x: i32 = s.next().expect("Error getting x").parse().expect("Error parsing x");
            let y: i32 = s.next().expect("Error getting y").parse().expect("Error parsing y");
            let z: i32 = s.next().expect("Error getting z").parse().expect("Error parsing z");
    
            points.push((x, y, z));
        }
        
    }

    points

}


fn get_surface_area(points: &Vec<(i32, i32, i32)>) -> (i32, HashSet<(i32, i32, i32)>) {

    let mut x_map: HashMap<i32, Vec<(i32, i32)>> = Default::default();
    let mut y_map: HashMap<i32, Vec<(i32, i32)>> = Default::default();
    let mut z_map: HashMap<i32, Vec<(i32, i32)>> = Default::default();
    let mut neighbouring_spaces: HashSet<(i32, i32, i32)> = Default::default();

    let mut surfaces = 0;

    for point in points {

        let x: i32 = point.0;
        let y: i32 = point.1;
        let z: i32 = point.2;

        surfaces += 6;

        let mut hs: HashSet<(i32, i32, i32)> = Default::default();

        if let Some(vals) = x_map.get_mut(&x) {
            for val in vals.iter() {
                if hs.contains(&(x, val.0, val.1)) {
                    continue;
                }
                if val.0 == y && (val.1 - z).abs() == 1 || val.1 == z && (val.0 - y).abs() == 1 {
                    surfaces -= 2;
                    hs.insert((x, val.0, val.1));
                }
            }
            vals.push((y, z));
        } else {
            x_map.insert(x, vec![(y, z)]);
        }
        if let Some(vals) = y_map.get_mut(&y) {
            for val in vals.iter() {
                if hs.contains(&(val.0, y, val.1)) {
                    continue;
                }
                if val.0 == x && (val.1 - z).abs() == 1 || val.1 == z && (val.0 - x).abs() == 1 {
                    surfaces -= 2;
                    hs.insert((val.0, y, val.1));
                }
            }
            vals.push((x, z));
        } else {
            y_map.insert(y, vec![(x, z)]);
        }
        if let Some(vals) = z_map.get_mut(&z) {
            for val in vals.iter() {
                if hs.contains(&(val.0, val.1, z)) {
                    continue;
                }
                if val.0 == x && (val.1 - y).abs() == 1 || val.1 == y && (val.0 - x).abs() == 1 {
                    surfaces -= 2;
                    hs.insert((val.0, val.1, z));
                }
            }
            vals.push((x, y));
        } else {
            z_map.insert(z, vec![(x, y)]);
        }

        // Points/spaces to check in part 2
        if neighbouring_spaces.contains(point) {
            neighbouring_spaces.remove(point);
        }
        for k in [-1, 1] {
            let p1 = (point.0+k, point.1, point.2);
            let p2 = (point.0, point.1+k, point.2);
            let p3 = (point.0, point.1, point.2+k);
            if !points.contains(&p1) {
                neighbouring_spaces.insert(p1);
            }
            if !points.contains(&p2) {
                neighbouring_spaces.insert(p2);
            }
            if !points.contains(&p2) {
                neighbouring_spaces.insert(p3);
            }
        }
    
    }

    (surfaces, neighbouring_spaces)

}

const SEARCH_LIMIT: i32 = 20;
fn air_bubble_surface(points: &Vec<(i32, i32, i32)>, visited: &mut HashSet<(i32, i32, i32)>, point: (i32, i32, i32)) -> i32 {
    let mut q: VecDeque<(i32, i32, i32)> = Default::default();
    let mut v: HashSet<(i32, i32, i32)> = Default::default(); // Visited locally
    let mut air_points: Vec<(i32, i32, i32)> = Default::default();
    q.push_back(point);
    v.insert(point);
    let mut size = 0;
    while let Some(el) = q.pop_front() {
        air_points.push(el);
        size += 6;
        //println!("---- {} {} {}", el.0, el.1, el.2);
        for k in [-1, 1] {
            let p1 = (el.0+k, el.1, el.2);
            let p2 = (el.0, el.1+k, el.2);
            let p3 = (el.0, el.1, el.2+k);
            if !points.contains(&p1) && !v.contains(&p1) {
                q.push_front(p1);
                v.insert(p1);
                size -= 2;
            }
            if !points.contains(&p2) && !v.contains(&p2) {
                q.push_front(p2);
                v.insert(p2);
                size -= 2;
            }
            if !points.contains(&p3) && !v.contains(&p3) {
                q.push_front(p3);
                v.insert(p3);
                size -= 2;
            }
        }
        size += 0;
        if el.0 > SEARCH_LIMIT || el.0 < 0 || el.1 > SEARCH_LIMIT || el.1 < 0 || el.2 > SEARCH_LIMIT || el.2 < 0 {
            visited.extend(v.iter());
            return 0;
        }
    }
    visited.extend(v.iter());

    (size, _) = get_surface_area(&air_points);

    size
}

pub fn solution() {
    let file_path = String::from("src/day18/1.txt");

    let points = parse_input(file_path);

    let (surfaces, neighboring_spaces) = get_surface_area(&points);

    println!("Surfaces: {}", surfaces);

    let mut visited: HashSet<(i32, i32, i32)> = Default::default();
    

    let mut size = 0;
    for point in neighboring_spaces {
        //println!("Point {} {} {}", point.0, point.1, point.2);
        if !visited.contains(&point) && !points.contains(&point) {
            //println!("Searching...");
            let s = air_bubble_surface(&points, &mut visited, point);            
            size += s;
        }
    }

    println!("Size of trapped air: {}", size);
    println!("Exterior surface area: {}", surfaces - size);


}



