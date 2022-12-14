use advent_of_code_2022::file_utils::read_lines;

enum Element {
    Number(i32),
    List(Vec<Element>)
}

fn parse_input(file_path: String) -> Vec<(Vec<Element>, Vec<Element>)> {

    let mut result: Vec<(Vec<Element>, Vec<Element>)> = Default::default();
    let mut parsed: Vec<Vec<Element>> = Default::default();

    if let Ok(mut lines) = read_lines(file_path) {
        
        

        // Parse monkey id
        while let Some(s) = lines.next() {

            let line = s.expect("Error line");
            if line.len() == 0 {
                let r = (parsed.pop().unwrap(), parsed.pop().unwrap());
                result.push((r.1, r.0));
                continue;
            }
            let s = line.split(',').fuse();
            let mut current_list: Vec<Element> = Default::default();
            let mut stack: Vec<Vec<Element>> = Default::default();


            for mut spl in s {
                
                while spl.starts_with("[") {
                    stack.push(current_list);
                    current_list = Default::default();
                    spl = &spl[1..];
                }
                let mut i = 0;
                while spl.ends_with("]") {
                    spl = &spl[..spl.len()-1];
                    i += 1;
                }

                //println!("Parsing '{}'", spl);
                if spl.len() > 0 {
                    let val: i32 = spl.parse().expect("Error parsing val");
                    current_list.push(Element::Number(val));
                }
                

                for _ in 0..i {
                    let old_list = current_list;
                    current_list = stack.pop().unwrap();
                    current_list.push(Element::List(old_list));
                }
                
            }

            println!("SIZE: {}", current_list.len());

            if let Element::List(l) = current_list.get(0).unwrap() {
                print_list(l);
                println!();
                parsed.push(current_list);
            }

            
        }

        let r = (parsed.pop().unwrap(), parsed.pop().unwrap());
        result.push((r.1, r.0));
    }

    result

}

fn print_list(list: &Vec<Element>) {
    print!("[");
    let mut first = true;
    for el in list {
        if first {
            first = false;
        } else {
            print!(",");
        }
        match el {
            Element::Number(n) => print!("{}", n),
            Element::List(l) => print_list(l)
        }
    }
    print!("]");
}


fn check_lists(left: &Vec<Element>, right: &Vec<Element>) -> i32 {

    for i in 0..left.len() {
        if i >= right.len() {
            return -1;
        }

        match left.get(i).unwrap() {
            Element::Number(x) => {
                match right.get(i).unwrap() {
                    Element::Number(y) => {
                        //println!("Compare {} vs {}", x, y);
                        if x < y {
                            return 1;
                        } else if x > y {
                            return -1;
                        }
                    },
                    Element::List(ly) => {
                        //println!("Compare {} vs [{}]", x, ly.len());
                        let check = check_lists(&vec![Element::Number(*x)], ly);
                        if check != 0 {
                            return check;
                        }
                    }
                }
            },
            Element::List(lx) => {
                match right.get(i).unwrap() {
                    Element::Number(y) => {
                        //println!("Compare [{}] vs {}", lx.len(), y);
                        let check = check_lists(lx, &vec![Element::Number(*y)]);
                        if check != 0 {
                            return check;
                        }
                    },
                    Element::List(ly) => {
                        //println!("Compare [{}] vs [{}]", lx.len(), ly.len());
                        let check = check_lists(lx, ly);
                        if check != 0 {
                            return check;
                        }
                    }
                }
            }
        }
    }

    if left.len() < right.len() {
        1
    } else {
        0
    }

}

pub fn solution() {
    let file_path = String::from("src/day13/1.txt");

    let lists = parse_input(file_path);
    let mut index_sum = 0;
    let mut index = 1;

    let mut all_lists: Vec<Vec<Element>> = Default::default();

    // Part 1
    for mut l in lists {
        // Check lists
        if let Element::List(left) = l.0.pop().unwrap() {
            if let Element::List(right) = l.1.pop().unwrap() {
                let check = check_lists(&left, &right); 
                if check >= 0 {
                    index_sum += index;
                }
                println!("Pair {}: {}", index, check);
                all_lists.push(left);
                all_lists.push(right);
            }
        }
        index += 1;
    }

    println!("Index sum: {}", index_sum);



    // -------------------------------------
    // Part 2 - "Bubble sort"
    // Add two divider packets
    all_lists.push(vec![Element::List(vec![Element::Number(2)])]);
    all_lists.push(vec![Element::List(vec![Element::Number(6)])]);

    // Bubble sort list
    for i in 0..all_lists.len() {
        for j in i+1..all_lists.len() {
            let el_i = all_lists.get(i).unwrap();
            let el_j = all_lists.get(j).unwrap();
            let comparison = check_lists(el_i, el_j);
            if comparison == -1 {
                all_lists.swap(i, j);
            }
        }
    }

    // Find divider packets
    let mut key = 1;
    for i in 0..all_lists.len() {
        //print_list(all_lists.get(i).unwrap());
        //println!();
        if all_lists.get(i).unwrap().len() == 1 {
            let el = all_lists.get(i).unwrap().get(0).unwrap();
            if let Element::List(l) = el {
                if l.len() == 1 {
                    match l.get(0).unwrap() {
                        Element::Number(2) | Element::Number(6) => key *= i+1,
                        _ => () 
                    }
                }
            }
        }
    }

    println!("Divider key: {}", key);

    

}



