use std::{env, process::exit};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
    let trimmed = input.trim();

    match day {
        1 => day1::main(trimmed),
        2 => day2::main(trimmed),
        3 => day3::main(trimmed),
        4 => day4::main(trimmed),
        5 => day5::main(trimmed),
        6 => day6::main(trimmed),
        7 => day7::main(trimmed),
        8 => day8::main(trimmed),
        9 => day9::main(trimmed),
        // 10 => day10::main(trimmed),
        // 11 => day11::main(trimmed),
        // 12 => day12::main(trimmed),
        // 13 => day13::main(trimmed),
        // 14 => day14::main(trimmed),
        // 15 => day15::main(trimmed),
        // 16 => day16::main(trimmed),
        // 17 => day17::main(trimmed),
        // 18 => day18::main(trimmed),
        // 19 => day19::main(trimmed),
        // 20 => day20::main(trimmed),
        // 21 => day21::main(trimmed),
        // 22 => day22::main(trimmed),
        // 23 => day23::main(trimmed),
        // 24 => day24::main(trimmed),
        // 25 => day25::main(trimmed),
        _ => {
            println!("day solution not yet implemented");
            exit(1);
        }
    }
}
