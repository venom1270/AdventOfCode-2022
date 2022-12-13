use advent_of_code_2022::file_utils::read_lines;
use std::{collections::HashSet};
use std::cmp::{min, max};

fn reposition_tail(head: (i32, i32), tails: &mut [(i32, i32)], tails_visited: &mut [HashSet<(i32, i32)>], index: usize) {

    let tail = &mut tails[index];
    let tail_visited = &mut tails_visited[index];

    let tail_visited_len = tail_visited.len();

    let mut x_diff = head.0 - tail.0;
    let mut y_diff = head.1 - tail.1;

    // Check diagonal first
    while x_diff.abs() > 1 && y_diff.abs() >= 1 || x_diff.abs() >= 1 && y_diff.abs() > 1 {
        if x_diff > 0 { tail.0 += 1; x_diff -= 1; }
        else { tail.0 -= 1; x_diff += 1; }
        if y_diff > 0 { tail.1 += 1; y_diff -= 1; }
        else { tail.1 -= 1; y_diff += 1; }
        tail_visited.insert(*tail); // * - "Dereferencing the borrow"
    }

    // Check non-diagonal directions and add to set
    let old_tail = tail.clone();
    if x_diff.abs() > 1 { 
        tail.0 += x_diff - is_negative_int(x_diff); 
        let mut loop_range = tail.0..old_tail.0+1;
        if x_diff > 0 {
            loop_range = old_tail.0..tail.0+1;
        }
        for i in loop_range {
            tail_visited.insert((i, tail.1));
        }
    }
    if y_diff.abs() > 1 { 
        tail.1 += y_diff - is_negative_int(y_diff); 
        let mut loop_range = tail.1..old_tail.1+1;
        if y_diff > 0 {
            loop_range = old_tail.1..tail.1+1;
        }
        for i in loop_range {
            tail_visited.insert((tail.0, i));
        }
    }

    //println!("Tail {}: Added {} new visited", index+1, tail_visited.len()-tail_visited_len);

}

fn is_negative_int(val: i32) -> i32 {
    if val < 0 {
        -1
    } else {
        1
    }
}

fn print_map(head: (i32, i32), tails: &[(i32, i32)]) {
    let mut up = 0;
    let mut down = 0;
    let mut right = 0;
    let mut left = 0;
    left = min(left, head.0);
    right = max(right, head.0);
    up = max(up, head.1);
    down = min(down, head.1);
    for (i, j) in tails {
        left = min(left, *i);
        right = max(right, *i);
        up = max(up, *j);
        down = min(down, *j);
    }
    println!("{} {} {} {}", up, down, left, right);
    for i in (down..up+1).rev() {
        for j in left..right+1 {
            let mut p: String = ".".to_string();
            if (j,i) == head {
                p = "H".to_string();
            }
            for k in 0..tails.len() {
                if (j, i) == tails[k] {
                    //println!("FOUND");
                    p = (k+1).to_string();
                }
            }
            print!("{}", p);
        }
        println!();
    }
}

fn print_visited(visited: &HashSet<(i32, i32)>) {
    let mut up = 0;
    let mut down = 0;
    let mut right = 0;
    let mut left = 0;
    for (i, j) in visited {
        left = min(left, *i);
        right = max(right, *i);
        up = max(up, *j);
        down = min(down, *j);
    }
    println!("{} {} {} {}", up, down, left, right);
    for i in (down..up+1).rev() {
        for j in left..right+1 {
            let mut p: String = ".".to_string();
            if visited.contains(&(i,j)) {
                p = "#".to_string();
            }
            print!("{}", p);
        }
        println!();
    }
}

pub fn solution() {
    let file_path = "src/day9/1.txt";
   
    const N: usize = 9;

    let mut head: (i32, i32) = (0, 0);
    let mut tails: [(i32, i32); N] = [(0, 0); N];
    let mut tails_visited: [HashSet<(i32, i32)>; N] = Default::default();

    
    for mut hs in tails_visited.iter_mut() {
        hs.insert((0, 0));
    }

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            let line = line.expect("Parse error");
        
            let mut s = line.split(' ').fuse();
            let direction = s.next().expect("Error direction");
            let mut val: i32 = s.next().expect("Error val").parse().expect("Value parse error");
            println!("{}", line);
            
            while val > 0 {
                val -= 1;
                match direction {
                    "R" => {
                        head = (head.0 + 1, head.1);
                    },
                    "L" => {
                        head = (head.0 - 1, head.1);
                    },
                    "U" => {
                        head = (head.0, head.1 + 1);
                    },
                    "D" => {
                        head = (head.0, head.1 - 1);
                    },
                    _ => panic!("Invalid direction")
                }
                for i in 0..N {
                    let mut h = head;
                    if i > 0 {
                        h = tails[i-1];
                    }           
                    reposition_tail(h, &mut tails, &mut tails_visited, i);
                }
            }
            //print_map(head, &tails);
            //println!("--------");
            //println!("H({},{}) T({},{})", head.0, head.1, tail.0, tail.1);
        }


    }

    println!("Tail 1 positions visited: {}", tails_visited[0].len());
    //println!("Tail {} positions visited: {}", N, tails_visited[N-1].len());

    for i in 0..N {
        println!("Tail {} positions visited: {}", i+1, tails_visited[i].len());
    }

    /*for tail in tails {
        println!("H({},{}) T({},{})", head.0, head.1, tail.0, tail.1);
    }*/

    //print_map(head, &tails);

    //print_visited(&tails_visited[8]);

}



/* 

Number of visited spots: 6090
Number of visited spots: 6845

Number of visited spots: 6090
Number of visited spots: 6090

Number of visited spots: 6090
Number of visited spots: 5433

Number of visited spots: 6090
Number of visited spots: 4837

Number of visited spots: 6090
Number of visited spots: 4292

Number of visited spots: 6090
Number of visited spots: 3804

Number of visited spots: 6090
Number of visited spots: 3396

Number of visited spots: 6090
Number of visited spots: 3053

Number of visited spots: 6090
Number of visited spots: 2806

Number of visited spots: 6090
Number of visited spots: 2566


*/