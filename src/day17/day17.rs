use advent_of_code_2022::file_utils::read_lines;
use std::collections::HashMap;
use std::cmp::{max,min};

enum Direction {
    Left,
    RIght
}

#[derive(Copy, Clone)]
enum ShapeType {
    LineVertical,
    LineHorizontal,
    Square,
    Plus,
    L,
}

struct Shape {
    shape_type: ShapeType,
    coords: Vec<(i32, i32)>
}
impl Shape {
    pub fn new(shape_type: ShapeType, bottom_left_coord: (i32, i32)) -> Self {
        let mut coords: Vec<(i32, i32)> = Default::default();
        match shape_type {
            ShapeType::LineHorizontal => {
                coords.push(bottom_left_coord);
                coords.push((bottom_left_coord.0+1, bottom_left_coord.1));
                coords.push((bottom_left_coord.0+2, bottom_left_coord.1));
                coords.push((bottom_left_coord.0+3, bottom_left_coord.1));
                Shape {
                    shape_type,
                    coords
                }
            },
            ShapeType::LineVertical => {
                coords.push(bottom_left_coord);
                coords.push((bottom_left_coord.0, bottom_left_coord.1+1));
                coords.push((bottom_left_coord.0, bottom_left_coord.1+2));
                coords.push((bottom_left_coord.0, bottom_left_coord.1+3));
                Shape {
                    shape_type,
                    coords
                }
            },
            ShapeType::Square => {
                coords.push(bottom_left_coord);
                coords.push((bottom_left_coord.0, bottom_left_coord.1+1));
                coords.push((bottom_left_coord.0+1, bottom_left_coord.1));
                coords.push((bottom_left_coord.0+1, bottom_left_coord.1+1));
                Shape {
                    shape_type,
                    coords
                }
            },
            ShapeType::L => {
                coords.push(bottom_left_coord);
                coords.push((bottom_left_coord.0+1, bottom_left_coord.1));
                coords.push((bottom_left_coord.0+2, bottom_left_coord.1));
                coords.push((bottom_left_coord.0+2, bottom_left_coord.1+1));
                coords.push((bottom_left_coord.0+2, bottom_left_coord.1+2));
                Shape {
                    shape_type,
                    coords
                }
            },
            ShapeType::Plus => {
                coords.push((bottom_left_coord.0+1, bottom_left_coord.1+1));
                coords.push((bottom_left_coord.0+1, bottom_left_coord.1));
                coords.push((bottom_left_coord.0, bottom_left_coord.1+1));
                coords.push((bottom_left_coord.0+2, bottom_left_coord.1+1));
                coords.push((bottom_left_coord.0+1, bottom_left_coord.1+2));
                Shape {
                    shape_type,
                    coords
                }
            }
        }
    }

    fn can_move(&self, dir: &Direction, board: &[[u32; 6000]]) -> bool {
        let mut coord_adjust = 0;
        match dir {
            Direction::Left => coord_adjust = -1,
            Direction::RIght=> coord_adjust = 1
        }
        for c in self.coords.iter() {
            let x = c.0 + coord_adjust;
            if x < 0 || x >= 7 {
                return false;
            }
            if board[x as usize][c.1 as usize] != 0 {
                return false
            }
        }

        true
    }

    pub fn move_shape(&mut self, dir: &Direction, board: &[[u32; 6000]]) -> bool {
        if self.can_move(&dir, board) {
            let mut coord_adjust = 0;
            match dir {
                Direction::Left => coord_adjust = -1,
                Direction::RIght=> coord_adjust = 1
            }
            for c in self.coords.iter_mut() {
                c.0 += coord_adjust;
            }
            return true;
        }
        
        false
    }

    pub fn fall(&mut self, board: &mut [[u32; 6000]]) -> bool {
        let mut can_fall = true;
        for c in self.coords.iter() {
            if c.1 - 1 < 0 || board[c.0 as usize][c.1 as usize-1] != 0 {
                can_fall = false;
                break;
            }
        }
        if can_fall {
            for c in self.coords.iter_mut() {
                c.1 = c.1 - 1;
            }
        } else {
            for c in self.coords.iter() {
                board[c.0 as usize][c.1 as usize] = 1;
            }
        }

        can_fall
        
    }

    pub fn print_coords(&self) {
        for c in self.coords.iter() {
            println!("{} {}", c.0, c.1);
        }
    }
}

fn parse_input(file_path: String) -> Vec<Direction> {

    let mut directions: Vec<Direction> = Default::default();

    if let Ok(mut lines) = read_lines(file_path) {
        
        let line = lines.next().expect("Error getting line").expect("Error getting line 2");

        for c in line.chars() {
            match c {
                '<' => directions.push(Direction::Left),
                '>' => directions.push(Direction::RIght),
                _ => println!("Unknown input character!"),
            }
        }

    }

    directions

}


pub fn solution() {
    let file_path = String::from("src/day17/1.txt");

    let directions = parse_input(file_path);

    const SHAPE_TYPE_ARRAY: [ShapeType; 5] = [ShapeType::LineHorizontal, ShapeType::Plus, ShapeType::L, ShapeType::LineVertical, ShapeType::Square];

    const AREA_WIDTH: usize = 7;
    const AREA_HEIGHT: usize = 6000;
    const NUM_SHAPES: u32 = 2022;
    let mut board: [[u32; AREA_HEIGHT]; AREA_WIDTH] = [[0; AREA_HEIGHT]; AREA_WIDTH];
    let mut top: [i32; AREA_WIDTH] = [-1; AREA_WIDTH];
    let mut directions_index = 0;

    for i in 0..NUM_SHAPES {
        let top_y = top.iter().max_by(|x, y| x.cmp(y)).unwrap() + 4;
        let mut shape = Shape::new(SHAPE_TYPE_ARRAY[(i % SHAPE_TYPE_ARRAY.len() as u32) as usize], (2, top_y as i32));

        // Move and fall shape
        loop {
            let dir = directions.get(directions_index).unwrap();
            shape.move_shape(dir, &board);
            /*match dir {
                Direction::Left => println!("Moving shape left"),
                Direction::RIght => println!("Moving shape right")
            }
            shape.print_coords();*/
            directions_index = (directions_index + 1) % directions.len();
            if !shape.fall(&mut board) {
                break;
            }
        }
        // Memorize top
        for c in shape.coords {
            //println!("TOP {} {}", c.0, c.1);
            if top[c.0 as usize] < c.1 {
                top[c.0 as usize] = c.1;
            }
        }

        print!("Top: ");
        for t in top {
            print!("{} ", t);
        }
        println!();

    }

    let tower_height = top.iter().max_by(|x,y| x.cmp(y)).unwrap() + 1;

    println!("Tower height: {}", tower_height);


}



