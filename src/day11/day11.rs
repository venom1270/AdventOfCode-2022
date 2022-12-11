use advent_of_code_2022::file_utils::read_lines;

enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract
}

enum OperationValue {
    Number(u32),
    OldValue
}

struct Monkey {
    id: u32,
    items: Vec<u32>,
    inspections: u32,
    operation: (Operator, OperationValue),
    test_division: u32,
    test_true: u32,
    test_false: u32
}

impl Monkey {
    pub fn new(id: u32, items: Vec<u32>, operation: (Operator, OperationValue), test_division: u32, test_true: u32, test_false: u32) -> Self {
        Monkey {
            id,
            items,
            inspections: 0,
            operation,
            test_division,
            test_true,
            test_false,
        }
    }
    
    fn do_operation(&self, x: u32, lcm: u32) -> u32 {
        let mut val = 0;
        match self.operation.1 {
            OperationValue::Number(y) => val = y,
            OperationValue::OldValue => val = x
        }
        match self.operation.0 {
            Operator::Add => {
                return x + val % lcm;
            },
            Operator::Subtract => {
                return x - val;
            },
            Operator::Multiply => {
                return ((x as u64) * (val as u64) % lcm as u64) as u32;
            },
            Operator::Divide => {
                return x / val;
            }
        }
    }

    pub fn inspect_part1(&mut self, lcm: u32) {
        self.items = self.items.iter().map(|x| self.do_operation(*x, lcm) / 3).collect(); // PART 1
        self.inspections += self.items.len() as u32;
    }

    pub fn inspect_part2(&mut self, lcm: u32) {
        self.items = self.items.iter().map(|x| self.do_operation(*x, lcm)).collect(); // PART 2
        self.inspections += self.items.len() as u32;
    }

    pub fn test(&self, i: u32) -> u32 {
        match i % self.test_division == 0 {
            true => self.test_true,
            false => self.test_false
        }
    }

}

fn parse_input(file_path: String) -> Vec<Monkey> {

    let mut monkeys: Vec<Monkey> = Default::default();

    if let Ok(mut lines) = read_lines(file_path) {
        
        // Parse monkey id
        while let Some(s) = lines.next() {
            let monkey_line = s.expect("Error monkey");
            let mut monkey_line_split = monkey_line.split(' ').fuse();
            monkey_line_split.next(); // Skip element
            let mut id_string = monkey_line_split.next().unwrap().chars();
            id_string.next_back();
            let id: u32 = id_string.as_str().parse().expect("Error parsing monkey id");

            // Parse items
            let mut items: Vec<u32> = Default::default();
            let items_line = lines.next().expect("Error parsing items line").expect("Error parsing items line 2");
            let items_line_split_tmp = items_line.replace(',', "");
            let mut items_line_split = items_line_split_tmp.split(' ').fuse();
            for _ in 0..4 {
                items_line_split.next();
            }
            while let Some(s) = items_line_split.next() {
                //println!("Parsing item: {}", s);
                items.push(s.parse().expect("Error parsing item"));
            }

            // Parse operation
            let operation_line = lines.next().expect("Error parsing operation line").expect("Error parsing operation line 2");
            let mut operation_line_split = operation_line.split(' ').fuse();
            for _ in 0..6 {
                operation_line_split.next();
            }
            let mut op: (Operator, OperationValue) = (Operator::Add, OperationValue::OldValue);
            match operation_line_split.next().unwrap() {
                "+" => op.0 = Operator::Add,
                "-" => op.0 = Operator::Subtract,
                "*" => op.0 = Operator::Multiply,
                "/" => op.0 = Operator::Divide,
                _ => panic!("Invalid opration!")
            }
            match operation_line_split.next().unwrap() {
                "old" => op.1 = OperationValue::OldValue,
                x => op.1 = OperationValue::Number(x.parse().expect("Error parsing operator value"))
            }

            // Parse test
            let test_line = lines.next().expect("Error parsing test line").expect("Error parsing test line 2");
            let mut test_line_split = test_line.split(' ').fuse();
            for _ in 0..5 {
                test_line_split.next();
            }
            let test: u32 = test_line_split.next().expect("Error test number").parse().expect("Error parsing test number");

            // Parse test true
            let test_true_line = lines.next().expect("Error parsing test true line").expect("Error parsing test true line 2");
            let mut test_true_line_split = test_true_line.split(' ').fuse();
            for i in 0..9 {
                test_true_line_split.next();
            }
            let test_true: u32 = test_true_line_split.next().expect("Error test true number").parse().expect("Error parsing test true number");

            // Parse test false
            let test_false_line = lines.next().expect("Error parsing test false line").expect("Error parsing test false line 2");
            let mut test_false_line_split = test_false_line.split(' ').fuse();
            for i in 0..9 {
                test_false_line_split.next();
            }
            let test_false: u32 = test_false_line_split.next().expect("Error test false number").parse().expect("Error parsing test false number");

            monkeys.push(Monkey::new(id, items, op, test, test_true, test_false));

            lines.next();

        }

    }

    monkeys

}

// Copied from: https://www.hackertouch.com/lowest-common-multiple-in-rust.html
fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
/////////////////////////

pub fn solution() {
    let file_path = "src/day11/1.txt";

    let mut monkeys: Vec<Monkey> = parse_input(file_path.to_string());
    let mut throws: Vec<(u32, u32)> = Default::default();

    let mut lcm_val = 1;
    for i in 0..monkeys.len() {
        lcm_val = lcm(lcm_val, monkeys.get(i).unwrap().test_division.try_into().unwrap());
    }

    println!("LCM: {}", lcm_val);

    const ROUNDS: i32 = 10000;

    for round in 0..ROUNDS {
        
        for i in 0..monkeys.len() {
            throws = Default::default();
            let m = monkeys.get_mut(i).unwrap();
            //m.inspect_part1();
            m.inspect_part2(lcm_val as u32);
            for i in m.items.iter() {
                let throw_to = m.test(*i);
                throws.push((throw_to, *i));
            }

            // Do throws
            m.items = Default::default();
            for (throw_to, item) in throws {
                monkeys.get_mut(throw_to as usize).unwrap().items.push(item);
            }
        }
    }

    let mut max1 = 0;
    let mut max2 = 0;
    for i in 0..monkeys.len() {
        let inspections = monkeys.get(i).unwrap().inspections;
        println!("Monkey {} inspects: {}", i+1, inspections);
        if inspections > max2 {
            if inspections > max1 {
                max2 = max1;
                max1 = inspections;
            } else {
                max2 = inspections;
            }
        }
    }

    println!("Monkey business level: {}", max1 as u64 * max2 as u64);

}



