use advent_of_code_2022::file_utils::read_lines;
use std::collections::HashSet;

pub fn part1() {
    let file_path : String = "src/day3/1.txt".to_string();

    if let Ok(lines) = read_lines(file_path) {

        let mut priority = 0;

        for line in lines {
            if let Ok(l) = line {              
                let compartment_split = l.len() / 2;
                let comp1 = &l[..compartment_split];
                let comp2 = &l[compartment_split..];
                let mut found_chars = HashSet::new();
                //println!("{} Comparing {} and {}", compartment_split, comp1, comp2);
                for c in comp1.chars() {
                    if !found_chars.contains(&c) && comp2.contains(c) {
                        //println!("{}", c);
                        found_chars.insert(c);
                        if c <= 'Z' {
                            priority += (c as u32 - 65 + 27);
                        } else {
                            priority += (c as u32 - 97 + 1);
                        }
                    }
                }

                
            }
        }

        println!("{}", priority);

    }

}


pub fn part2() {

    let file_path : String = "src/day3/1.txt".to_string();

    if let Ok(lines) = read_lines(file_path) {

        let mut priority = 0;

        // We have to convert to "peekable" to be able to "peek()" without consiming iteratior (moving it forward), and it has to be "mut"
        let mut lines = lines.peekable();

        while lines.peek().is_some() {


            let comp1 = lines.next().expect("Error1").expect("Error1");
            let comp2 = lines.next().expect("Error2").expect("Error2");
            let comp3 = lines.next().expect("Error3").expect("Error3");

            //println!("{} | {} | {}", comp1, comp2, comp3);

            for c in comp1.chars() {
                if comp2.contains(c) && comp3.contains(c) {
                    //println!("{}", c);
                    if c <= 'Z' {
                        priority += c as u32 - 65 + 27;
                    } else {
                        priority += c as u32 - 97 + 1;
                    }
                    break;
                }
            }


        }

        println!("{}", priority);

    }

}


