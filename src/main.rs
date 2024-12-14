use std::{env, process::exit};

mod day01_lists;
mod day02_reactor;
mod day03_muls;
mod day04_xmas;
mod day05_rules;
mod day06_chuchu;
mod day07_operators;
mod day08_antennas;
mod day09_defrag;
mod day10_hiking;
mod day11_stones;
mod day12_flowers;
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

mod tools2d;

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
        1 => day01_lists::main(trimmed),
        2 => day02_reactor::main(trimmed),
        3 => day03_muls::main(trimmed),
        4 => day04_xmas::main(trimmed),
        5 => day05_rules::main(trimmed),
        6 => day06_chuchu::main(trimmed),
        7 => day07_operators::main(trimmed),
        8 => day08_antennas::main(trimmed),
        9 => day09_defrag::main(trimmed),
        10 => day10_hiking::main(trimmed),
        11 => day11_stones::main(trimmed),
        12 => day12_flowers::main(trimmed),
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
