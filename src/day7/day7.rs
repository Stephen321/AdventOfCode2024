use std::{fs, process::exit};

fn main() {
    let input_file_path = "./src/day7/input.txt";
    let contents = fs::read_to_string(input_file_path).unwrap();

    let test = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[derive(Debug, Clone, Eq, Hash, PartialEq)]
    enum Op {
        Add,
        Mult,
    }

    fn apply_op(a: u64, b: u64, op: &Op) -> u64 {
        match &op {
            Op::Add => a + b,
            Op::Mult => a * b,
        }
    }

    fn try_op(target: u64, values: &[u64], acc: u64, op: Op) -> bool {
        if values.is_empty() {
            //println!("values is empty for op {:?}, returning false", op);
            return false;
        }
        //println!(
        //    "acc is {} and values {:?} in try ops with op {:?}",
        //    acc, values, op
        //);
        let acc = apply_op(acc, values[0], &op);
        //print!("\t - apply op gave new acc of {}\n", acc);
        if acc > target {
            //println!("acc greater than target, return false");
            return false;
        } else if acc == target {
            //println!("acc is equaltarget, return true");
            return true;
        }
        //println!("cotinue search, calling try ops again for add and mult on next value");
        try_next_ops(target, values, acc)
    }

    fn try_next_ops(target: u64, values: &[u64], acc: u64) -> bool {
        try_op(target, &values[1..], acc, Op::Add) || try_op(target, &values[1..], acc, Op::Mult)
    }

    fn is_valid(target: u64, values: &[u64]) -> bool {
        println!(
            "is_valid called with values {:?}\tTarget: {:?}",
            values, target
        );

        let ops = [Op::Add, Op::Mult];

        let acc = values[0];
        println!("acc starts as {}", acc);
        try_next_ops(target, values, acc)
    }

    let count: u64 = contents
        .lines()
        .map(str::split_whitespace)
        .map(|mut split| {
            let target = split.next().unwrap();
            let target: u64 = target[..target.len() - 1].parse().unwrap();

            let values: Vec<u64> = split.map(str::parse).map(Result::unwrap).collect();
            let len = values.len();

            println!("\ntarget {} with values {:?}", target, values);

            let mut acc = values[0];
            if is_valid(target, &values) {
                println!("is valid!!");
                return target;
            }
            println!("is not valid!!");
            0
        })
        .sum();
    //println!("{:?}", data);

    //let rows = data.len();
    //let cols = data[0].len();
    //println!("rows {:?} cols {:?}", rows, cols);

    // Part 1
    println!("Part 1 {:?}", count);

    // Part 2
    println!("Part 2 {:?}", "");
}
