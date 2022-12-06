use advent_of_code_2022::file_utils::read_lines;


pub fn part1() {
    let file_path : String = "src/day6/1.txt".to_string();

    // I need mut here for some reason
    if let Ok(mut lines) = read_lines(file_path) {


        let line = lines.next().unwrap().unwrap();

        let mut left = 0;
        let mut right = 1;

        while right-left < 4 && right < line.len() {
            let c = line.chars().nth(right).unwrap();
            let subs = &line[left..right];
            if subs.contains(c) {
                left += subs.find(c).unwrap() + 1;
            }
            right += 1;
        }

        println!("{}", right);

    }
}


pub fn part2() {

    let file_path : String = "src/day6/1.txt".to_string();

    if let Ok(mut lines) = read_lines(file_path) {


        let line = lines.next().unwrap().unwrap();
        

        let mut left = 0;
        let mut right = 1;

        while right-left < 14 && right < line.len() {
            let c = line.chars().nth(right).unwrap();
            let subs = &line[left..right];
            if subs.contains(c) {
                left += subs.find(c).unwrap() + 1;
            }
            right += 1;
        }

        println!("{}", right);

    }

}


