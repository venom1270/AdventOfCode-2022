use advent_of_code_2022::file_utils::read_lines;
use std::cmp::{max, min};

fn parse_input(file_path: &str) -> Vec<Vec<u32>> {

    let mut grid: Vec<Vec<u32>> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            let line = line.expect("Parse error");
            let mut row: Vec<u32> = Vec::new();
            for c in line.chars() {
                row.push(c.to_string().parse().unwrap());
            }
            grid.push(row);
        }
    }

    grid
}

fn tree_visible(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {

    let height = grid.get(i).unwrap().get(j).unwrap();
    let n = grid.len();
    let m = grid.get(0).unwrap().len();

    // First and third loop have to have x: i32 as usize (which is default) is unsigned

    // Check up
    let mut x: i32 = i as i32 - 1;
    while x >= 0 && grid.get(x as usize).unwrap().get(j).unwrap() < height { x -= 1; }
    if x < 0 { return true; }
    // Check down
    let mut x = i + 1;
    while x < n && grid.get(x).unwrap().get(j).unwrap() < height { x += 1; }
    if x >= n { return true; }
    // Check left
    let mut x: i32 = j as i32 - 1;
    while x >= 0 && grid.get(i).unwrap().get(x as usize).unwrap() < height { x -= 1; }
    if x < 0 { return true; }
    // Check right
    let mut x = j + 1;
    while x < m && grid.get(i).unwrap().get(x).unwrap() < height { x += 1; }
    if x >= m { return true; }

    false
}


fn tree_score(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {

    let height = grid.get(i).unwrap().get(j).unwrap();
    let n = grid.len();
    let m = grid.get(0).unwrap().len();
    let mut score: usize = 1;

    // First and third loop have to have x: i32 as usize (which is default) is unsigned

    // Check up
    let mut x: i32 = i as i32 - 1;
    while x >= 0 && grid.get(x as usize).unwrap().get(j).unwrap() < height { x -= 1; }
    x = max(0, x);
    score *= i - x as usize;
    // Check down
    let mut x = i + 1;
    while x < n && grid.get(x).unwrap().get(j).unwrap() < height { x += 1; }
    x = min(n-1, x);
    score *= x - i;
    // Check left
    let mut x: i32 = j as i32 - 1;
    while x >= 0 && grid.get(i).unwrap().get(x as usize).unwrap() < height { x -= 1; }
    x = max(0, x);
    score *= j - x as usize;
    // Check right
    let mut x = j + 1;
    while x < m && grid.get(i).unwrap().get(x).unwrap() < height { x += 1; }
    x = min(m-1, x);
    score *= x - j;

    score
}

pub fn solution() {
    let grid = parse_input("src/day8/1.txt");
    let n = grid.len();
    let m = grid.get(0).unwrap().len();

    let mut trees_visible = 2*m + 2*n - 4;
    let mut max_tree_score = 0;

    for i in 1..n-1 {
        for j in 1..m-1 {
            max_tree_score = max(max_tree_score, tree_score(&grid, i, j));
            if tree_visible(&grid, i, j) {
                trees_visible += 1;
            }
        }
    }

    println!("Trees visible: {}", trees_visible);
    println!("Max tree score visible: {}", max_tree_score);

}



