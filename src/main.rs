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
mod day13_theclaw;
mod day14_omorashi;
mod day15_sokobot;
mod day16_maze;
mod day17_computer;
mod day18_meteors;
mod day19_hotspring;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

mod vec2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: advent-of-code-2024 [day] [input.txt]");
        exit(1);
    }
    let day: usize = str::parse(&args[1]).expect("couldn't parse day as number");
    let input = std::fs::read_to_string(&args[2]).expect("couldn't read input text");
    let input: &str = input.trim();

    match day {
        1 => day01_lists::main(input),
        2 => day02_reactor::main(input),
        3 => day03_muls::main(input),
        4 => day04_xmas::main(input),
        5 => day05_rules::main(input),
        6 => day06_chuchu::main(input),
        7 => day07_operators::main(input),
        8 => day08_antennas::main(input),
        9 => day09_defrag::main(input),
        10 => day10_hiking::main(input),
        11 => day11_stones::main(input),
        12 => day12_flowers::main(input),
        13 => day13_theclaw::main(input),
        14 => day14_omorashi::main(input),
        15 => day15_sokobot::main(input),
        16 => day16_maze::main(input),
        17 => day17_computer::main(input),
        18 => day18_meteors::main(input),
        19 => day19_hotspring::main(input),
        // 20 => day20::main(input),
        // 21 => day21::main(input),
        // 22 => day22::main(input),
        // 23 => day23::main(input),
        // 24 => day24::main(input),
        // 25 => day25::main(input),
        _ => unimplemented!(),
    }
}
