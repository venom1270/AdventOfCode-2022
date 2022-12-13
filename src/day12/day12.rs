use advent_of_code_2022::file_utils::read_lines;
use std::collections::{HashSet, VecDeque};

fn parse_input(file_path: String) -> ((u32, u32), (u32, u32), Vec<Vec<u32>>) {

    let mut grid: Vec<Vec<u32>> = Default::default();
    let mut start: (u32, u32) = (0, 0);
    let mut end: (u32, u32) = (0, 0);

    if let Ok(mut lines) = read_lines(file_path) {
        
        let mut i = 0;
        let mut j = 0;

        // Parse monkey id
        while let Some(s) = lines.next() {
            let mut grid_line: Vec<u32> = Default::default();
            let line = s.expect("Error line");
            
            j = 0;
            for c in line.chars() {
                match c {
                    'S' => {
                        grid_line.push('a' as u32 - 97);
                        start = (i, j);
                    },
                    'E' => {
                        grid_line.push('z' as u32 - 97);
                        end = (i, j);
                    },
                    x => grid_line.push(x as u32 - 97)
                }
                j += 1;
                
            }

            grid.push(grid_line);
            i += 1;
        }

    }

    (start, end, grid)

}

fn part1(start: (u32, u32), end: (u32, u32), grid: &Vec<Vec<u32>>) {
    
    // Iterative breadth first search
    let mut visited: HashSet<(u32, u32)> = Default::default();
    let mut queue: VecDeque<(u32, u32, u32)> = Default::default();
    let N = grid.len() as u32;
    let M = grid.get(0).unwrap().len() as u32;
    queue.push_back((start.0, start.1, 0));


    let mut steps = 0;

    while let Some((i, j, current_steps)) = queue.pop_front() {
        if (i, j) == end {
            steps = current_steps;
            break;
        }
        if visited.contains(&(i, j)) {
            continue;
        }
        visited.insert((i, j));

        //println!("Evaluating ({}, {}, {})", i, j, current_steps);

        let val = grid.get(i as usize).unwrap().get(j as usize).unwrap();
        // Next steps
        if i as i32-1 >= 0 && grid.get((i-1) as usize).unwrap().get(j as usize).unwrap() <= &(val+1) {
            queue.push_back((i-1, j, current_steps+1));
        }
        if j as i32 -1 >= 0 && grid.get(i as usize).unwrap().get((j-1) as usize).unwrap() <= &(val+1) {
            queue.push_back((i, j-1, current_steps+1));
        }
        if i+1 < N && grid.get((i+1) as usize).unwrap().get(j as usize).unwrap() <= &(val+1) {
            queue.push_back((i+1, j, current_steps+1));
        }
        if j+1 < M && grid.get(i as usize).unwrap().get((j+1) as usize).unwrap() <= &(val+1) {
            queue.push_back((i, j+1, current_steps+1));
        }
    }

    println!("Minumum number of steps: {}", steps);

}

fn part2(start: (u32, u32), end: (u32, u32), grid: &Vec<Vec<u32>>) {
    
    // Iterative breadth first search
    let mut visited: HashSet<(u32, u32)> = Default::default();
    let mut queue: VecDeque<(u32, u32, u32)> = Default::default();
    let N = grid.len() as u32;
    let M = grid.get(0).unwrap().len() as u32;
    queue.push_back((end.0, end.1, 0));


    let mut steps = 0;

    while let Some((i, j, current_steps)) = queue.pop_front() {

        let val = *grid.get(i as usize).unwrap().get(j as usize).unwrap();

        //println!("{}", val);

        if val == 0 {
            steps = current_steps;
            break;
        }
        if visited.contains(&(i, j)) {
            continue;
        }
        visited.insert((i, j));

        //println!("Evaluating ({}, {}, {})", i, j, current_steps);

        // Next steps
        if i as i32-1 >= 0 {
            let next_val = *grid.get((i-1) as usize).unwrap().get(j as usize).unwrap() as i32;
            if  next_val <= val as i32 && val as i32 - next_val <= 1 || next_val > val as i32 { 
                queue.push_back((i-1, j, current_steps+1));
            }
        } 
        if j as i32-1 >= 0 {
            let next_val = *grid.get(i as usize).unwrap().get((j-1) as usize).unwrap() as i32;
            if  next_val <= val as i32 && val as i32 - next_val <= 1 || next_val > val as i32 { 
                queue.push_back((i, j-1, current_steps+1));
            }
        } 
        if  i+1 < N {
            let next_val = *grid.get((i+1) as usize).unwrap().get(j as usize).unwrap() as i32;
            if  next_val <= val as i32 && val as i32 - next_val <= 1 || next_val > val as i32 { 
                queue.push_back((i+1, j, current_steps+1));
            }
        } 
        if j+1 < M {
            let next_val = *grid.get(i as usize).unwrap().get((j+1) as usize).unwrap() as i32;
            if  next_val <= val as i32 && val as i32 - next_val <= 1 || next_val > val as i32 { 
                queue.push_back((i, j+1, current_steps+1));
            }
        } 
    }

    println!("Minumum number of steps on most optimal start: {}", steps);

}

pub fn solution() {
    let file_path = String::from("src/day12/nsk4.txt");

    let (start, end, grid) = parse_input(file_path);

    part1(start, end, &grid);
    part2(start, end, &grid);

}



