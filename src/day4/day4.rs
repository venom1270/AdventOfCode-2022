use advent_of_code_2022::file_utils::read_lines;

pub fn part1() {
    let file_path : String = "src/day4/1.txt".to_string();

    if let Ok(lines) = read_lines(file_path) {

        let mut overlaps = 0;

        for line in lines {
            if let Ok(l) = line {              
                
                let s: Vec<&str> = l.split(',').collect();
                let s1: Vec<&str> = s[0].split('-').collect();
                let s2: Vec<&str> = s[1].split('-').collect();

                let x1: i32 = s1[0].parse().expect("Error x1");
                let y1: i32 = s1[1].parse().expect("Error y1");
                let x2: i32 = s2[0].parse().expect("Error x2");
                let y2: i32 = s2[1].parse().expect("Error y2");

                if (x1 >= x2 && y1 <= y2) || (x1 <= x2 && y1 >= y2) {
                    overlaps += 1;
                }
                
            }
        }

        println!("{}", overlaps);

    }

}


pub fn part2() {

    let file_path : String = "src/day4/1.txt".to_string();

    if let Ok(lines) = read_lines(file_path) {

        let mut overlaps = 0;

        for line in lines {
            if let Ok(l) = line {              
                
                let s: Vec<&str> = l.split(',').collect();
                let s1: Vec<&str> = s[0].split('-').collect();
                let s2: Vec<&str> = s[1].split('-').collect();

                let x1: i32 = s1[0].parse().expect("Error x1");
                let y1: i32 = s1[1].parse().expect("Error y1");
                let x2: i32 = s2[0].parse().expect("Error x2");
                let y2: i32 = s2[1].parse().expect("Error y2");

                if !(y2 < x1 || x2 > y1) {
                    overlaps += 1;
                }
                
            }
        }

        println!("{}", overlaps);

    }

}


