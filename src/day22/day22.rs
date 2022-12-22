use advent_of_code_2022::file_utils::read_lines;
use std::{collections::HashMap};
use std::cmp::{min,max};

enum Tile {
    Open,
    Wall
}

fn parse_input(file_path: String) -> (HashMap<(u32, u32), Tile>, Vec<(u32, i32)>, [(u32, u32, u32, u32); 6]) {

    let mut map: HashMap<(u32, u32), Tile> = Default::default();
    let mut directions: Vec<(u32, i32)> = Default::default();
    let mut areas = [(0,0,0,0); 6];
    let mut areas_index = 0;

    if let Ok(lines) = read_lines(file_path) {
        
        let mut row = 1;

        let mut min_area = 0;
        let mut max_area = 0;
        let mut last_row = 1;

        let mut next_min = 0;
        let mut next_max = 0;

        for line in lines {
            let l = line.expect("Error getting line");
        
            if l.starts_with(" ") || l.starts_with("#") || l.starts_with(".") {
                let mut column = 0;
                next_min = 0;
                next_max = 0;
                for c in l.chars() {
                    column += 1;
                    let mut tile = Tile::Open;
                    match c {
                        '#' => tile = Tile::Wall,
                        '.' => tile = Tile::Open,
                        _ => continue
                    }
                    //println!("Inserting {}, {} | {}", row, column, c);
                    map.insert((row, column), tile);
                    if next_min == 0 {
                        next_min = column;
                    }
                    next_max = column;
                }
            } else if l.len() > 1 {
                let mut num_s: String = String::from("");
                let mut turn: i32 = 0;
                for c in l.chars() {
                    if ['L', 'R'].contains(&c) {
                        match c {
                            'L' => turn = -1,
                            'R' => turn = 1,
                            _ => println!("Error")
                        }
                        directions.push((num_s.parse().expect("Error parsing num_s"), turn));
                        num_s.clear();
                    } else {
                        num_s.push(c);
                    }
                }
                directions.push((num_s.parse().expect("Error parsing last num_s"), 0));
            }


            if next_max != max_area || next_min != min_area {
                if min_area == 0 {
                    min_area = next_min;
                } else if next_min != min_area {
                    // Add area START..min_area
                    println!("Found area1 {} {} | {} {}", last_row, min_area, row-1, max_area);
                    areas[areas_index] = (last_row, min_area, row-1, max_area);
                    areas_index += 1;
                    min_area = next_min;
                    last_row = row;
                }
                if max_area == 0 {
                    max_area = next_max;
                } else if next_max != max_area {
                    // Add area START..min_area
                    println!("Found area2-1 {} {} | {} {}", last_row, min_area, row-1, next_max);
                    println!("Found area2-2 {} {} | {} {}", last_row, next_max+1, row-1, max_area);
                    areas[areas_index] = (last_row, min_area, row-1, next_max);
                    areas_index += 1;
                    areas[areas_index] = (last_row, next_max+1, row-1, max_area);
                    areas_index += 1;
                    max_area = next_max;
                    last_row = row;
                }
            }

            row += 1;
        }
        
        println!("Found area1 {} {} | {} {}", last_row, min_area, row, max_area);
        areas[areas_index] = (last_row, min_area, row-1, max_area);
        
    }

    (map, directions, areas)

}

fn find_start_position(map: &HashMap<(u32, u32), Tile>) -> (u32, u32) {
    for i in 1..1000 {
        for j in 1..1000 {
            if let Some(_) = map.get(&(i, j)) {
                return (i, j);
            }
        }
    }
    return (0, 0)
}

fn find_wrap_around(map: &HashMap<(u32, u32), Tile>, mut current_position: (u32, u32), direction: u32) -> (u32, u32) {
    //println!("Readjusted from {}, {}", current_position.0, current_position.1);
    while let Some(_) = map.get(&current_position) {
        match direction {
            0 => current_position.1 += 1,
            1 => current_position.0 += 1,
            2 => current_position.1 -= 1,
            3 => current_position.0 -= 1,
            _ => println!("Erorr match find_wrap_around...")
        }
    }
    // Readjust position by 1
    match direction {
        0 => current_position.1 -= 1,
        1 => current_position.0 -= 1,
        2 => current_position.1 += 1,
        3 => current_position.0 += 1,
        _ => ()
    }
    //println!("Readjusted to {}, {}", current_position.0, current_position.1);
    return current_position;
}

fn get_area(areas: [(u32, u32, u32, u32); 6], current_position: (u32, u32)) -> u32 {
    let (x, y) = current_position;
    let mut i = 0;
    for (x1, y1, x2, y2) in areas {
        if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
            return i;
        }
        i += 1;
    }
    return 10;
}

fn find_wrap_around_cube(map: &HashMap<(u32, u32), Tile>, mut current_position: (u32, u32), mut direction: u32, areas: [(u32, u32, u32, u32); 6]) -> (u32, u32, u32) {
    
    let current_area = get_area(areas, current_position);
    println!("Readjusted from {}, {} | {}", current_position.0, current_position.1, direction);
    match current_area {
        0 => {
            match direction {
                2 => {
                    let a = areas[3];
                    // 1,51 -> 1,150
                    // 10,51 -> 1,141
                    current_position = (1, 151 - current_position.0);
                    direction = 0;
                },
                3 => {
                    let a = areas[5];
                    // 1,51 -> 151,1
                    // 1,60 -> 160,1
                    current_position = (current_position.1 + 100, 1);
                    direction = 0;
                },
                _ => unreachable!("ERROR 0")
            }
        },
        1 => {
            match direction {
                0 => {
                    let a = areas[4];
                    // 1,150 -> 150,100
                    // 10,150 -> 141,100
                    current_position = (151 - current_position.0, 100);
                    direction = 2;
                },
                1 => {
                    let a = areas[2];
                    // 50,150 -> 100,100
                    // 50,140 -> 90,100
                    current_position = (current_position.1-50, 100);
                    direction = 2;
                },
                3 => {
                    let a = areas[5];
                    // 1,101 -> 200,50
                    // 1,110 -> 200,41
                    current_position = (200, 151 - current_position.1);
                    direction = 3;
                },
                _ => unreachable!("ERROR 0")
            }
        },
        2 => {
            match direction {
                0 => {
                    let a = areas[1];
                    // 51,100 -> 50, 101
                    // 60,100 -> 50, 110
                    current_position = (50, current_position.0 + 50);
                    direction = 3;
                },
                2 => {
                    let a = areas[3];
                    // 51,51 -> 101,1
                    // 60,51 -> 101,10
                    current_position = (101, current_position.0 - 50);
                    direction = 1;
                },
                _ => unreachable!("ERROR 0")
            }
        },
        3 => {
            match direction {
                2 => {
                    let a = areas[0];
                    // 101,1 -> 50,51
                    // 110,1 -> 41,51
                    current_position = (151-current_position.0, 51);
                    direction = 0;
                },
                3 => {
                    let a = areas[2];
                    // 101,1 -> 51,51
                    // 101,10 -> 60,51
                    current_position = (50+current_position.1, 51);
                    direction = 0;
                },
                _ => unreachable!("ERROR 0")
            }
        },
        4 => {
            match direction {
                0 => {
                    let a = areas[1];
                    // 101,100 -> 50,150
                    // 110,100 -> 41,150
                    current_position = (151-current_position.0, 150);
                    direction = 2;
                },
                1 => {
                    let a = areas[5];
                    // 150,51 -> 151,50
                    // 150,60 -> 160,50
                    current_position = (100 + current_position.1, 50);
                    direction = 2;
                },
                _ => unreachable!("ERROR 0")
            }
        },
        5 => {
            match direction {
                0 => {
                    let a = areas[4];
                    // 151,50 -> 150,51
                    // 160,50 -> 150,60
                    current_position = (150, current_position.0 - 100);
                    direction = 3;
                },
                1 => {
                    let a = areas[1];
                    // 200,1 -> 1,150
                    // 200,10 -> 1,141

                    // 200,1 -> 1,101
                    // 200,10 -> 1,110
                    //current_position = (1, 151-current_position.1);
                    current_position = (1, 100+current_position.1);
                    direction = 1;
                },
                2 => {
                    let a = areas[0];
                    // 151,1 -> 1,51
                    // 160,1 -> 1,60
                    //current_position = (1, 50+current_position.1);
                    current_position = (1, current_position.0 - 100);
                    direction = 1;
                },
                _ => unreachable!("ERROR 0")
            }
        },
        _ => ()
    }

    println!("Readjusted to {}, {} | {}", current_position.0, current_position.1, direction);
    return (current_position.0, current_position.1, direction);
}

pub fn solution() {
    let file_path = String::from("src/day22/1.txt");
    let (map, directions, areas) = parse_input(file_path);

    let mut current_position = find_start_position(&map);
    let mut current_direction: i32 = 0;
    let mut next_direction = 0;

    for (val, turn) in directions {

        // println!("{} {}", val, turn);
        // Make 'val' steps
        for _ in 0..val {

            let mut next_position = current_position; 
            match current_direction {
                0 => next_position.1 += 1,
                1 => next_position.0 += 1,
                2 => next_position.1 -= 1,
                3 => next_position.0 -= 1,
                _ => println!("Erorr match current_direction...")
            }

            if let None = map.get(&next_position) {
                // Wrap around...
                //next_position = find_wrap_around(&map, current_position, (current_direction + 2) as u32 % 4);

                (next_position.0, next_position.1, next_direction) = find_wrap_around_cube(&map, current_position, current_direction as u32, areas);
            }

            if let Some(pos) = map.get(&next_position) {
                if matches!(pos, Tile::Wall) {
                    break;
                } else {
                    current_position = next_position;

                    current_direction = next_direction as i32;
                }
            } else {
                println!("Error let Some next position");
            }

        }

        current_direction = (current_direction + turn) % 4;
        if current_direction < 0 {
            current_direction += 4;
        }
        next_direction = current_direction as u32;

    }

    let final_password = 1000 * current_position.0 + 4 * current_position.1 + current_direction as u32;
    println!("Final password (part 1): {}", final_password);
    // 3360, 132088, 141173 - too low | not right: 157062

}



