use advent_of_code_2022::file_utils::read_lines;

fn parse_input(file_path: String) -> Vec<i32> {

    let mut numbers: Vec<i32> = Default::default(); 

    if let Ok(lines) = read_lines(file_path) {
        
        for line in lines {
            let l = line.expect("Error getting line");
            numbers.push(l.parse().expect("Error parsing number"));
        }
    }

    numbers

}

fn find_index(numbers: &Vec<(usize, i32)>, i: usize) -> usize {
    for j in 0..numbers.len() {
        let (ni, _) = numbers[j];
        if ni == i {
            return j;
        }
    }
    println!("Error find_index");
    return 0;
}

fn print(numbers: &Vec<(usize, i32)>) {
    for (_, i) in numbers {
        print!("{}, ", i);
    }
    println!();
}

pub fn solution() {
    let file_path = String::from("src/day20/1.txt");

    let original_numbers = parse_input(file_path);
    let len = original_numbers.len();
    let mut numbers: Vec<(usize, i32)> = Default::default();
    for i in 0..len {
        numbers.push((i, original_numbers[i]));
    }
    
    for i in 0..len {
        let mut n = original_numbers[i];
        let mut ni = find_index(&numbers, i);

        if n > 0 {
            while n > 0 {
                numbers.swap(ni % len, (ni + 1) % len);
                ni += 1;
                n -= 1;
            }
        } else if n < 0 {
            while n < 0 {
                numbers.swap(ni % len, (ni - 1) % len);
                ni += 1;
                n -= 1;
            }
        }
        
        //print(&numbers);
    }   

    println!("LEN: {}", len);
    
    let mut grove_coordinates = 0;
    for (i, n) in numbers.iter() {
        if n == &0 {
            println!("{} {} {}", numbers[(i+1000-1) % len].1, numbers[(i+2000-1) % len].1, numbers[(i+3000-1) % len].1);
            grove_coordinates = numbers[(i+1000-1) % len].1 + numbers[(i + 2000-1) % len].1 + numbers[(i + 3000-1) % len].1;
            break;
        }
    }

    println!("Grove coordinates: {}", grove_coordinates);
    // wrong: -11289, -3075, -12770, 11365, -700, -2819

}



