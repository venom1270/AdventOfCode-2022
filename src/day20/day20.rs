use advent_of_code_2022::file_utils::read_lines;

fn parse_input(file_path: String) -> Vec<i64> {

    let mut numbers: Vec<i64> = Default::default(); 

    if let Ok(lines) = read_lines(file_path) {
        
        for line in lines {
            let l = line.expect("Error getting line");
            numbers.push(l.parse().expect("Error parsing number"));
        }
    }

    numbers

}

fn find_index(numbers: &Vec<(usize, i64)>, i: usize) -> usize {
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

fn mix(original_numbers: &Vec<i64>, numbers: &mut Vec<(usize, i64)>) {
    let len = original_numbers.len();
    for i in 0..len {
        let n = original_numbers[i];
        if n == 0 {
            continue;
        }
        let numbers_index = find_index(&numbers, i);
        numbers.remove(numbers_index);
        let mut move_to = (n+numbers_index as i64).rem_euclid((len-1) as i64);
        if move_to == 0 {
            move_to = len as i64 - 1;
        } else if move_to == len as i64 - 1 {
            move_to = 0;
        }
        //println!("Moving {} from {} to {}", n, numbers_index, move_to);
        
        numbers.insert(move_to as usize, (i, n));
        //print(&numbers);
    }   
}

fn get_coordinates(numbers: &Vec<(usize, i64)>) -> i64 {
    let mut grove_coordinates = 0;
    let mut x = 0;
    let len = numbers.len();
    for (i, n) in numbers.iter() {
        if n == &0 {
            println!("{} {} {}", numbers[(x+1000) % len].1, numbers[(x+2000) % len].1, numbers[(x+3000) % len].1);
            grove_coordinates = numbers[(x+1000) % len].1 + numbers[(x + 2000) % len].1 + numbers[(x + 3000) % len].1;
            break;
        }
        x += 1;
    }
    grove_coordinates
}

pub fn solution() {
    let file_path = String::from("src/day20/1.txt");

    let mut original_numbers = parse_input(file_path);
    
    // Part 1
    let mut numbers: Vec<(usize, i64)> = Default::default();
    let len = original_numbers.len();
    for i in 0..len {
        numbers.push((i, original_numbers[i] as i64));
    }
    mix(&original_numbers, &mut numbers);
    let grove_coordinates = get_coordinates(&numbers);
    println!("Grove coordinates: {}", grove_coordinates);

    // Part 2
    const KEY: i64 = 811589153;
    let mut numbers: Vec<(usize, i64)> = Default::default();
    let decrypted_numbers: Vec<i64> = original_numbers.iter().map(|x| x * KEY).collect();
    let len = decrypted_numbers.len();
    for i in 0..len {
        numbers.push((i, decrypted_numbers[i] as i64));
    }
    for _ in 0..10 {
        mix(&decrypted_numbers, &mut numbers);
    }
    let grove_coordinates = get_coordinates(&numbers);
    println!("Grove coordinates with decryption: {}", grove_coordinates);


}



// 15, 2, -3, 3, -2, 0, 4   ->  15, 2, -3, 3, -2, 0, 4,| 2, -3, 3, -2, 0, 4,| 2, -3, 3, X -2, 0, 4,| 2, -3, 3, -2, 0, 4
// 12, 2, -3, 3, -2, 0, 4   ->  12, 2, -3, 3, -2, 0, 4,| 2, -3, 3, -2, 0, 4,| X 2, -3, 3, -2, 0, 4,| 2, -3, 3, -2, 0, 4
// -15, 2, -3, 3, -2, 0, 4   ->  2, -3, 3, X -2, 0, 4,| 2, -3, 3, -2, 0, 4,| 2, -3, 3, -2, 0, 4,| -15, 2, -3, 3, -2, 0, 4
// -12, 2, -3, 3, -2, 0, 4   ->  2, -3, 3, -2, 0, 4, X | 2, -3, 3, -2, 0, 4,| 2, -3, 3, -2, 0, 4,| -12, 2, -3, 3, -2, 0, 4


// 1, 1, 15, -3, 3, -2, 0   ->  1, 1, 15, -3, 3, -2, 0,| 1, 1, -3, 3, -2, 0,| 1, 1, -3, 3, -2, X 0,| 1, 1, -3, 3, -2, 0
// 1, 1, -15, -3, 3, -2, 0   ->  1, 1, -3, 3, -2, X 0,| 1, 1, -3, 3, -2, 0,| 1, 1, -3, 3, -2, 0,| 1, 1, -15, -3, 3, -2, 0
// 1, 1, 11, -3, 3, -2, 0   ->  1, 1, 11, -3, 3, -2, 0,| 1, 1, -3, 3, -2, 0,| 1, X 1, -3, 3, -2, 0,| 1, 1, -3, 3, -2, 0
// 1, 1, -11, -3, 3, -2, 0   ->  1, 1, -3, 3, -2, 0,| 1, 1, -3, X 3, -2, 0,| 1, 1, -3, 3, -2, 0,| 1, 1, -11, -3, 3, -2, 0