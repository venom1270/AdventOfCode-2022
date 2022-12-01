use advent_of_code_2022::file_utils::read_lines;

pub fn part1() {

    //let file_path : String = env::current_dir().unwrap().display().to_string() + "\\src\\day1\\1.txt";
    let file_path : String = "src/day1/1.txt".to_string();

    //println!("In file {}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String

        let mut max_calories = 0;
        let mut curr_calories = 0;

        for line in lines {
            if let Ok(l) = line {
                match l.parse::<i32>() {
                    Ok(n) => {
                        curr_calories += n; 
                    },
                    Err(_) => {
                        if max_calories < curr_calories {
                            max_calories = curr_calories;
                        } 
                        curr_calories = 0;
                    }
                }
            }
        }

        println!("{}", max_calories);

    }

}


pub fn part2() {

    let file_path : String = "src/day1/1.txt".to_string();

    //println!("In file {}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String

        let mut elf1 = 0;
        let mut elf2 = 0;
        let mut elf3 = 0;
        let mut curr_calories = 0;

        for line in lines {
            if let Ok(l) = line {
                match l.parse::<i32>() {
                    Ok(n) => {
                        curr_calories += n; 
                    },
                    Err(_) => {
                        
                        if elf3 < curr_calories {
                            if elf2 < curr_calories {
                                elf3 = elf2;
                                if elf1 < curr_calories {
                                    elf2 = elf1;
                                    elf1 = curr_calories;
                                } else {
                                    elf2 = curr_calories;
                                }
                            } else {
                                elf3 = curr_calories;
                            }
                        }

                        curr_calories = 0;
                    }
                }
            }
        }

        let top_elves_calories = elf1 + elf2 + elf3;
        println!("{}", top_elves_calories);

    }

}


