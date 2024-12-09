use std::{env, process::exit};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: advent-of-code-2024 [day] [input.txt]");
        exit(1);
    }
    let day: usize = str::parse(&args[1]).expect("couldn't parse day as number");
    let input = std::fs::read_to_string(&args[2]).expect("couldn't read input text");

    match day {
        1 => day1::main(&input),
        2 => day2::main(&input),
        3 => day3::main(&input),
        4 => day4::main(&input),
        5 => day5::main(&input),
        6 => day6::main(&input),
        7 => day7::main(&input),
        8 => day8::main(&input),
        // 9 => day9::main(&input),
        // 10 => day10::main(&input),
        // 11 => day11::main(&input),
        // 12 => day12::main(&input),
        // 13 => day13::main(&input),
        // 14 => day14::main(&input),
        // 15 => day15::main(&input),
        // 16 => day16::main(&input),
        // 17 => day17::main(&input),
        // 18 => day18::main(&input),
        // 19 => day19::main(&input),
        // 20 => day20::main(&input),
        // 21 => day21::main(&input),
        // 22 => day22::main(&input),
        // 23 => day23::main(&input),
        // 24 => day24::main(&input),
        // 25 => day25::main(&input),
        _ => {
            println!("day solution not yet implemented");
            exit(1);
        }
    }
}
