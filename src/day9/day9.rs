use advent_of_code_2022::file_utils::read_lines;
use std::{collections::HashSet};

fn reposition_tail(head: (i32, i32), tails: &mut [(i32, i32)], tails_visited: &mut [HashSet<(i32, i32)>], index: usize) {

    let tail = &mut tails[index];
    let tail_visited = &mut tails_visited[index];

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
}

fn is_negative_int(val: i32) -> i32 {
    if val < 0 {
        -1
    } else {
        1
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
            let val: i32 = s.next().expect("Error val").parse().expect("Value parse error");
            println!("{}", line);
            match direction {
                "R" => {
                    head = (head.0 + val, head.1);
                },
                "L" => {
                    head = (head.0 - val, head.1);
                },
                "U" => {
                    head = (head.0, head.1 + val);
                },
                "D" => {
                    head = (head.0, head.1 - val);
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
            //println!("H({},{}) T({},{})", head.0, head.1, tail.0, tail.1);
        }


    }

    println!("Tail 1 positions visited: {}", tails_visited[0].len());
    println!("Tail {} positions visited: {}", N, tails_visited[N-1].len());

    for tail in tails {
        println!("H({},{}) T({},{})", head.0, head.1, tail.0, tail.1);
    }

}



