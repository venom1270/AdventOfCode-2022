use advent_of_code_2022::file_utils::read_lines;
use std::collections::{HashSet, HashMap};
use std::cmp::{min,max};

struct Elf {
    x: i32,
    y: i32,
    proposed_x: i32,
    proposed_y: i32
}

enum Direction {
    North,
    South,
    West,
    East
}

fn parse_input(file_path: String) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Default::default();
    if let Ok(lines) = read_lines(file_path) {
        let mut y = 0;
        for line in lines {
            let l = line.expect("Error getting line");
            let mut x = 0;
            for c in l.chars() {
                if c == '#' {
                    elves.push(Elf {x, y, proposed_x: x, proposed_y: y});
                }
                x += 1;
            }
            y += 1;
        }
    }
    elves
}

fn get_rectangle_size(elves: &Vec<Elf>) -> i32 {
    let mut x1 = std::i32::MAX;
    let mut y1 = std::i32::MAX;
    let mut x2 = std::i32::MIN;
    let mut y2 = std::i32::MIN;

    for elf in elves {
        //println!("Elf {},{}", elf.x, elf.y);
        x1 = min(x1, elf.x); 
        y1 = min(y1, elf.y);
        x2 = max(x2, elf.x);
        y2 = max(y2, elf.y);
    }

    println!("{},{} | {},{}", x1, y1, x2, y2);

    (x2 - x1 + 1) * (y2 - y1 + 1)
}

fn print_map(elves: &Vec<Elf>) {
    let mut elves_map: HashMap<(i32, i32), usize> = Default::default();

    let mut x1 = std::i32::MAX;
    let mut y1 = std::i32::MAX;
    let mut x2 = std::i32::MIN;
    let mut y2 = std::i32::MIN;

    // Create elves map
    for (i, elf) in elves.iter().enumerate() {
        elves_map.insert((elf.x, elf.y), i);
        x1 = min(x1, elf.x); 
        y1 = min(y1, elf.y);
        x2 = max(x2, elf.x);
        y2 = max(y2, elf.y);
    }

    println!("-----");
    for i in y1..y2+1 {
        for j in x1..x2+1 {
            if elves_map.contains_key(&(j, i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn propose(elves_map: &HashMap<(i32, i32), usize>, elf: &Elf, dir: &Direction) -> Option<(i32, i32)> {
    
    let mut elf_adjacent = false;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            if elves_map.contains_key(&(elf.x+i, elf.y+j)) {
                elf_adjacent = true;
                break;
            }
        }
    }

    if !elf_adjacent {
        return None
    }
    
    let mut k1 = (elf.x, elf.y);
    let mut k2 = (elf.x, elf.y);
    let mut k3 = (elf.x, elf.y);
    match dir {
        Direction::North => {
            k1.1 -= 1;
            k2 = (elf.x-1, elf.y-1);
            k3 = (elf.x+1, elf.y-1);
        },
        Direction::South => {
            k1.1 += 1;
            k2 = (elf.x-1, elf.y+1);
            k3 = (elf.x+1, elf.y+1);
        },
        Direction::West => {
            k1.0 -= 1;
            k2 = (elf.x-1, elf.y+1);
            k3 = (elf.x-1, elf.y-1);
        },
        Direction::East => {
            k1.0 += 1;
            k2 = (elf.x+1, elf.y+1);
            k3 = (elf.x+1, elf.y-1);
        }
    }
    if elves_map.contains_key(&k1) || elves_map.contains_key(&k2) || elves_map.contains_key(&k3) {
        None
    } else {
        Some(k1)
    }
}

fn solve(elves: &mut Vec<Elf>) -> i32 {

    let mut proposals: HashMap<(i32, i32), u32> = Default::default();
    let mut elves_map: HashMap<(i32, i32), usize> = Default::default();
    let directions = [Direction::North, Direction::South, Direction::West, Direction::East];
    let mut directions_index = 0;

    const NUM_ROUNDS: u32 = 100000;

    // Create elves map
    for (i, elf) in elves.iter().enumerate() {
        elves_map.insert((elf.x, elf.y), i);
    }

    // Start rounds
    for r in 0..NUM_ROUNDS {
        proposals.clear();
        // First phase - make proposals
        for elf in elves.iter_mut() {
            for i in 0..directions.len() {
                let dir = &directions[(directions_index + i) % directions.len()];
                if let Some(proposal) = propose(&elves_map, elf, dir) {
                    // If adjacent positions do not contain elf, propose
                    if let Some(x) = proposals.get(&proposal) {
                        proposals.insert(proposal, x+1);
                    } else {
                        proposals.insert(proposal, 1);
                    }
                    elf.proposed_x = proposal.0;
                    elf.proposed_y = proposal.1;
                    break;
                }
            }
        }

        // Second phase - execute proposals
        let mut elf_moved = false;
        for (i, elf) in elves.iter_mut().enumerate() {
            // Check if elf made a proposal
            if let Some(num_proposals) = proposals.get(&(elf.proposed_x, elf.proposed_y)) {
                // Check if the position was proposed only by one elf (this one)
                if *num_proposals == 1 {
                    // Execute proposal
                    elves_map.remove(&(elf.x, elf.y));
                    elf.x = elf.proposed_x;
                    elf.y = elf.proposed_y;
                    elves_map.insert((elf.x, elf.y), i);
                    elf_moved = true;
                } else {
                    elf.proposed_x = elf.x;
                    elf.proposed_y = elf.y;
                }
            }
        }

        if !elf_moved {
            println!("No elf moved! Round: {}", r+1);
            break;
        }

        //print_map(&elves);

        // Advence direction
        directions_index = (directions_index + 1) % directions.len();
    }

    

    get_rectangle_size(elves)
}

pub fn solution() {
    let file_path = String::from("src/day23/1.txt");
    let mut elves = parse_input(file_path);

    let rectangle_size = solve(&mut elves);
    print_map(&elves);

    let empty_tiles = rectangle_size - elves.len() as i32;
    println!("Rectangle size: {} | Empty tiles: {}", rectangle_size, empty_tiles);

}



