use advent_of_code_2022::file_utils::read_lines;
use std::collections::{HashSet, HashMap, VecDeque, BinaryHeap};
use std::cmp::{min,max, Ordering};

#[derive(Clone, Copy)]
struct Blizzard {
    x: u32,
    y: u32,
    direction: char
}

#[derive(Eq, PartialEq)]
struct State {
    time: u32,
    expedition: (u32, u32),
    cost: u32
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.time.cmp(&other.time))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(file_path: String) -> (Vec<Vec<char>>, Vec<Blizzard>) {
    let mut blizzards: Vec<Blizzard> = Default::default();
    let mut map: Vec<Vec<char>> = Default::default();
    if let Ok(lines) = read_lines(file_path) {
        let mut y = 0;
        for line in lines {
            let l = line.expect("Error getting line");
            let mut x = 0;
            let mut row: Vec<char> = Default::default();
            for c in l.chars() {
                if ['<', '>', '^', 'v'].contains(&c) {
                    blizzards.push(Blizzard { x, y, direction: c });
                }
                row.push(c);
                x += 1;
            }
            map.push(row);
            y += 1;
        }
    }
    (map, blizzards)
}

fn calculate_cost(position: (u32, u32), goal: (u32, u32), time: u32) -> u32 {
    //println!("{},{} | {},{}", position.0, position.1, goal.0, goal.1);
    //((goal.0 - position.0) + (goal.1 - position.1)) * 31 + time * 6
    ((goal.0 as i32 - position.0 as i32).abs() + (goal.1 as i32 - position.1 as i32).abs()) as u32 * 1 + time * 1
    //time
}

fn solve(blizzards_cache: &Vec<(Vec<Blizzard>, HashMap<(u32,u32), u32>)>, map: &Vec<Vec<char>>, start: (u32, u32), end: (u32, u32), initial_time: u32) -> u32 {
    let n = map.len();
    let m = map[0].len();
    let mut visited: HashSet<(u32, u32, u32)> = Default::default();  
    
    let state = State { time: initial_time, expedition: start, cost: calculate_cost(start, end, 0)};
    //let mut q: VecDeque<State> = Default::default();
    let mut pq = BinaryHeap::new();
    //q.push_back(state);
    pq.push(state);

    let mut old_time = 0;


    let mut qwe = 0;
    let mut solution: i32 = -1;
    //while let Some(s) = q.pop_front() {
    while let Some(s) = pq.pop() {

        //println!("{}, {}", s.expedition.0, s.expedition.1);

        if solution != -1 && s.time as i32 + (end.0 as i32 - s.expedition.0 as i32).abs() + (end.1 as i32 - s.expedition.1 as i32).abs() >= solution {
            continue;
        }

        qwe += 1;
        if qwe % 1000000 == 0 {
            println!("({},{}) | {} | {} | SIZE: {}", s.expedition.0, s.expedition.1, s.time, s.cost, pq.len());
        }

        if s.expedition == end {
            if solution == -1 {
                solution = s.time as i32;
                println!("NEW SOLUTION: {}", solution);
            }
            //return s.time;
        }

        if old_time < s.time {
            //println!("New time: {}", s.time);
            old_time = s.time;
        }

        /*if let Some(x) = memo.get(&s.expedition) {
            if s.time as i32 - (max(n, m)) as i32 > *x as i32 {
                continue;
            } else {
                memo.insert(s.expedition, s.time);
            }
        } else {
            memo.insert(s.expedition, s.time);
        }*/

        if visited.contains(&(s.expedition.0, s.expedition.1, s.time)) {
            continue;
        }
        visited.insert((s.expedition.0, s.expedition.1, s.time));

        let (_, blizzard_map) = &blizzards_cache[s.time as usize + 1];

        // Check possible positions to move
        // Check up
        let mut check_pos = s.expedition;
        if check_pos.1 > 0 {
            check_pos.1 -= 1;
            if map[check_pos.1 as usize][check_pos.0 as usize] != '#' && !blizzard_map.contains_key(&check_pos) {
                //q.push_front(State { time: s.time+1, expedition: check_pos, cost: calculate_cost(check_pos, end) });
                pq.push(State { time: s.time+1, expedition: check_pos, cost: calculate_cost(check_pos, end, s.time+1) });
            }
        }
        // Check left
        check_pos = s.expedition;
        if check_pos.0 > 0 {
            check_pos.0 -= 1;
            if map[check_pos.1 as usize][check_pos.0 as usize] != '#' && !blizzard_map.contains_key(&check_pos) {
                //q.push_front(State { time: s.time+1, expedition: check_pos, cost: calculate_cost(check_pos, end) });
                pq.push(State { time: s.time+1, expedition: check_pos, cost: calculate_cost(check_pos, end, s.time+1) });
            }
        }
        // Check down
        check_pos = s.expedition;
        check_pos.1 += 1;
        if check_pos.1 < n as u32 && map[check_pos.1 as usize][check_pos.0 as usize] != '#' && !blizzard_map.contains_key(&check_pos) {
            //q.push_front(State { time: s.time+1, expedition: check_pos, cost: calculate_cost(check_pos, end) });
            pq.push(State { time: s.time+1, expedition: check_pos, cost: calculate_cost(check_pos, end, s.time+1) });
        }
        // Check right
        check_pos = s.expedition;
        check_pos.0 += 1;
        if check_pos.0 < m as u32 && map[check_pos.1 as usize][check_pos.0 as usize] != '#' && !blizzard_map.contains_key(&check_pos) {
            //q.push_front(State { time: s.time+1, expedition: check_pos, cost: calculate_cost(check_pos, end) });
            pq.push(State { time: s.time+1, expedition: check_pos, cost: calculate_cost(check_pos, end, s.time+1) });
        }
        // Check current
        if !blizzard_map.contains_key(&s.expedition) {
            //q.push_front(State { time: s.time+1, expedition: s.expedition, cost: s.cost });
            pq.push(State { time: s.time+1, expedition: s.expedition, cost: calculate_cost(s.expedition, end, s.time+1) });
        }
        
    }

    solution as u32

}

pub fn solution() {
    let file_path = String::from("src/day24/1.txt");
    let (map, mut blizzards) = parse_input(file_path);

    let n = map.len();
    let m = map[0].len();
    let start = (1, 0);
    let end = (m as u32 - 2, n as u32 - 1);

    const BLIZZARD_CACHE_SIZE: usize = 1000;
    let mut blizzards_cache: Vec<(Vec<Blizzard>, HashMap<(u32, u32), u32>)> = Default::default();
    let mut blizzard_map: HashMap<(u32, u32), u32> = Default::default();
    blizzards_cache.push((blizzards.clone(), blizzard_map.clone()));
    for _ in 1..BLIZZARD_CACHE_SIZE {
        blizzard_map.clear();
        for b in blizzards.iter_mut() {
            let mut new_pos = (b.x, b.y);
            match b.direction {
                '>' => new_pos.0 += 1,
                '<' => new_pos.0 -= 1,
                '^' => new_pos.1 -= 1,
                'v' => new_pos.1 += 1,
                _ => unreachable!("Direction error")
            }
            if map[new_pos.1 as usize][new_pos.0 as usize] == '#' {
                match b.direction {
                    '>' => new_pos.0 = 1,
                    '<' => new_pos.0 = m as u32 - 2,
                    '^' => new_pos.1 = n as u32 - 2,
                    'v' => new_pos.1 = 1,
                    _ => unreachable!("Direction overlap error")
                }
            }
            b.x = new_pos.0;
            b.y = new_pos.1;

            if let Some(x) = blizzard_map.get(&(b.x, b.y)) {
                blizzard_map.insert((b.x, b.y), x+1);
            } else {
                blizzard_map.insert((b.x, b.y), 1);
            }
        }
        blizzards_cache.push((blizzards.clone(), blizzard_map.clone()));
    }
    

    let min_time = solve(&blizzards_cache, &map, start, end, 0);
    println!("Min time to exit: {}", min_time);
    // 316 - too low, 292, too high: 453
    let min_time = solve(&blizzards_cache, &map, end, start, min_time);
    println!("Min time back to start: {}", min_time);
    let min_time = solve(&blizzards_cache, &map, start, end, min_time);
    println!("Min time back again to exit: {}", min_time);
    

}



