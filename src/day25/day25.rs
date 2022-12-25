use advent_of_code_2022::file_utils::read_lines;

fn parse_input(file_path: String) -> Vec<String> {
    let mut numbers: Vec<String> = Default::default();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            numbers.push(line.unwrap());
        }
    }
    numbers
}

fn snafu_to_decimal(number: String) -> i64 {
    let mut power: i64 = 1; // I don't know how it's called properly in English xD
    let mut decimal_number = 0;
    for c in number.chars().rev() {
        match c {
            '2' => decimal_number += 2*power,
            '1' => decimal_number += 1*power,
            '-' => decimal_number -= 1*power,
            '=' => decimal_number -= 2*power,
            '0' => (),
            _ => unreachable!("Invalid SNAFU digit!")
        }
        power *= 5;
    }
    decimal_number
}

fn decimal_to_snafu(number: i64) -> String {
    let mut snafu_number = String::from("");
    let mut max_number: [u64; 20] = [2; 20];
    for i in 1..20 {
        max_number[i] = max_number[i-1] + 2*5_u64.pow(i as u32);
    }

    let mut num_places = 0;
    while number as u64 > max_number[num_places] {
        num_places += 1;
    }
    num_places += 1;

    let mut current_decimal = 0;
    let options = [(2, '2'), (1, '1'), (0, '0'), (-1, '-'), (-2, '=')];
    for i in (0..num_places).rev() {
        let pow = 5_i64.pow(i as u32);
        let mut c = '2';
        let mut best = i64::MAX;
        let mut diff = i64::MAX;

        // Check all options and choose one which gives result closest to target number
        for (val, ch) in options {
            let value = current_decimal + val*pow;
            let d = (value - number as i64).abs();
            if d < diff {
                best = value;
                c = ch;
                diff = d;
            }
        }
        current_decimal = best;
        snafu_number.push(c);
    }
    snafu_number
}

pub fn solution() {
    let file_path = String::from("src/day25/1.txt");
    let numbers = parse_input(file_path);
    let mut sum_decimal = 0;
    for n in numbers {
        sum_decimal += snafu_to_decimal(n);
    }
    println!("Decimal SUM of numbers: {}", sum_decimal);
    let sum_snafu = decimal_to_snafu(sum_decimal);
    println!("SNAFU SUM of numbers: {}", sum_snafu);
       
    /*decimal_to_snafu(1);
    decimal_to_snafu(2);
    decimal_to_snafu(3);
    decimal_to_snafu(4);
    decimal_to_snafu(5);
    decimal_to_snafu(20);
    decimal_to_snafu(2022);
    decimal_to_snafu(12345);*/
}



