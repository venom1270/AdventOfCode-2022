use advent_of_code_2022::file_utils::read_lines;
use std::collections::HashSet;
use std::cmp::max;

fn parse_tuple(s: String) -> (i32, i32) {
    let mut split = s.trim().split(",").fuse();
    (split.next().unwrap().parse().expect("Error parsing tuple 1"), split.next().unwrap().parse().expect("Error parsing tuple 2"))
}

fn parse_input(file_path: String) -> (i32, HashSet<(i32, i32)>) {

    let mut hs: HashSet<(i32, i32)> = Default::default();
    let mut abyss = 0;

    if let Ok(mut lines) = read_lines(file_path) {
        
    
        while let Some(s) = lines.next() {

            let l = s.expect("Error line");
            let mut split = l.trim().split("->").fuse();
            
            let (mut old_x, mut old_y) = parse_tuple(split.next().unwrap().to_string());
            abyss = max(abyss, old_y);
            for st in split {
                let (new_x, new_y) = parse_tuple(st.to_string());
                if new_x - old_x != 0 {
                    if new_x > old_x {
                        for i in old_x..new_x+1 {
                            hs.insert((i, new_y));
                        }
                    } else {
                        for i in new_x..old_x+1 {
                            hs.insert((i, new_y));
                        }
                    }
                } else {
                    if new_y > old_y {
                        for i in old_y..new_y+1 {
                            hs.insert((new_x, i));
                        }
                    } else {
                        for i in new_y..old_y+1 {
                            hs.insert((new_x, i));
                        }
                    }
                }
                old_x = new_x;
                old_y = new_y;
                abyss = max(abyss, new_y);
            }

            
        }

    }

    (abyss, hs)

}


pub fn solution() {
    let file_path = String::from("src/day14/1.txt");

    let (abyss, mut hs) = parse_input(file_path);

    let mut sand_pos = (500, 0);
    let mut sand_count_abyss = 0;
    let mut sand_count_floor = 0;
    let mut hs_floor = hs.clone();
    let floor = abyss + 2;

    println!("Abyss y: {}", abyss);
    println!("Floor y: {}", floor);

    // Do until sand does not reach the abyss
    'outer: while sand_pos.1 < abyss {
        sand_pos = (500, 0);
        let mut can_move = true;
        while can_move {
            if sand_pos.1 > abyss {
                break 'outer;
            } else if !hs.contains(&(sand_pos.0, sand_pos.1+1)) {
                sand_pos.1 += 1;
            } else if !hs.contains(&(sand_pos.0-1, sand_pos.1+1)) {
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if !hs.contains(&(sand_pos.0+1, sand_pos.1+1)) {
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                hs.insert(sand_pos);
                //println!("Sand landed at {} {}", sand_pos.0, sand_pos.1);
                sand_count_abyss += 1;
                can_move = false;
            }
        }
    }

    while sand_pos != (500, 0) {
        sand_pos = (500, 0);
        let mut can_move = true;
        while can_move {
            if sand_pos.1+1 == floor {
                can_move = false;
            } else if !hs_floor.contains(&(sand_pos.0, sand_pos.1+1)) {
                sand_pos.1 += 1;
            } else if !hs_floor.contains(&(sand_pos.0-1, sand_pos.1+1)) {
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if !hs_floor.contains(&(sand_pos.0+1, sand_pos.1+1)) {
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                can_move = false;
            }
        }
        hs_floor.insert(sand_pos);
        sand_count_floor += 1;
    }

    println!("Sand count with abyss: {}", sand_count_abyss);
    println!("Sand count with floor: {}", sand_count_floor);

    

}



