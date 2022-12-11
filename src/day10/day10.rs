use advent_of_code_2022::file_utils::read_lines;

pub fn solution() {
    let file_path = "src/day10/1.txt";
   
    let mut x = [0; 250];
    let mut cycle = 0;
    x[0] = 1;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            let line = line.expect("Parse error");
            let mut s = line.split(' ').fuse();

            let command = s.next().expect("Error command");
            match command {
                "addx" => {
                    let add: i32 = s.next().expect("Error add").parse().expect("Error add parse");
                    x[cycle+1] = x[cycle];
                    x[cycle+2] = x[cycle] + add;
                    cycle += 2;
                },
                "noop" => {
                    x[cycle+1] = x[cycle];
                    cycle += 1;
                },
                _ => {
                    panic!("Invalid input");
                }
            }

            if cycle >= 240 {
                break;
            }
        }
    }

    let mut signal_strength = 0;

    for i in [20, 60, 100, 140, 180, 220] {
        signal_strength += x[i-1] * i as i32;
        //println!("{} | {} {} {} {}", x[i] * i as i32, x[i-1], x[i], x[i+1], x[i+2]);
    }

    println!("Signal strength: {}", signal_strength);

    println!("CRT image:");

    for i in 0..240 {
        //println!("{}", x[i]);
        if (x[i]-1..x[i]+2).contains(&(i as i32 % 40)) {
            print!("#");
            //println!("# {} {}", i, x[i]);
        } else {
            print!(".");
            //println!(". {} {}", i, x[i]);
        }
        if (i+1) % 40 == 0 {
            println!();
        }
    }

}



