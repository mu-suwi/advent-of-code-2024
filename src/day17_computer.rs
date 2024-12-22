// day 17 - three-bit brainfuck

// this puzzle involves simulating a computer that only works with 3-bit numbers
// and has a unique limited set of instructions.

use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
struct Memory {
    a: u32,
    b: u32,
    c: u32,
    ptr: usize,
    out: VecDeque<u32>,
}

fn combo(operand: u32, mem: &Memory) -> u32 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => mem.a,
        5 => mem.b,
        6 => mem.c,
        _ => {
            panic!("Invalid combo operand");
        }
    }
}

fn adv(operand: u32, mem: &mut Memory) {
    mem.a /= 2u32.pow(combo(operand, mem));
    mem.ptr += 2;
}

fn bxl(operand: u32, mem: &mut Memory) {
    mem.b ^= operand;
    mem.ptr += 2;
}

fn bst(operand: u32, mem: &mut Memory) {
    mem.b = combo(operand, mem) % 8;
    mem.ptr += 2;
}

fn jnz(operand: u32, mem: &mut Memory) {
    if mem.a != 0 {
        mem.ptr = operand as usize;
    } else {
        mem.ptr += 2;
    }
}

#[allow(unused_variables)]
fn bxc(operand: u32, mem: &mut Memory) {
    mem.b ^= mem.c;
    mem.ptr += 2;
}

fn out(operand: u32, mem: &mut Memory) {
    mem.out.push_back(combo(operand, mem) % 8);
    mem.ptr += 2;
}

fn bdv(operand: u32, mem: &mut Memory) {
    mem.b = mem.a / 2u32.pow(combo(operand, mem));
    mem.ptr += 2;
}

fn cdv(operand: u32, mem: &mut Memory) {
    mem.c = mem.a / 2u32.pow(combo(operand, mem));
    mem.ptr += 2;
}

pub fn main(input: &str) {
    let (input_mem, input_prog) = input.split_once("\n\n").unwrap();

    let re = Regex::new(r"[0-9]+").unwrap();

    let mut reg = {
        let mem: Vec<_> = re
            .find_iter(input_mem)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();
        Memory {
            a: mem[0],
            b: mem[1],
            c: mem[2],
            ptr: 0,
            out: VecDeque::new(),
        }
    };

    let program: Vec<u32> = re
        .find_iter(input_prog)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();

    println!("{reg:?}");
    println!("{program:?}");
    println!("---");

    while reg.ptr < program.len() {
        let opcode = program[reg.ptr];
        let operand = program[reg.ptr + 1];
        match opcode {
            0 => {
                adv(operand, &mut reg);
            }
            1 => {
                bxl(operand, &mut reg);
            }
            2 => {
                bst(operand, &mut reg);
            }
            3 => {
                jnz(operand, &mut reg);
            }
            4 => {
                bxc(operand, &mut reg);
            }
            5 => {
                out(operand, &mut reg);
            }
            6 => {
                bdv(operand, &mut reg);
            }
            7 => {
                cdv(operand, &mut reg);
            }
            _ => {
                panic!("invalid opcode!");
            }
        }
    }

    let output = reg
        .out
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", output);
}
