// day 17 - three-bit brainfuck

// this puzzle involves simulating a computer that only works with 3-bit numbers
// and has a unique limited set of instructions. you are given a starting configuration
// with some value in register A and some source code to be executed.

// for part 2, find an input to put in register A which results in

use regex::Regex;

#[derive(Debug)]
struct Memory {
    a: u64,
    b: u64,
    c: u64,
    ptr: usize,
    out: Vec<u64>,
}

fn combo(operand: u64, mem: &Memory) -> u64 {
    // some opcodes interpret their operand as a "combo operand" which can read from registers.
    // for others it's just a literal.
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

fn adv(operand: u64, mem: &mut Memory) {
    // divide register A by 2 to the X, where X is the combo operand
    mem.a /= 2u64.pow(combo(operand, mem) as u32);
    mem.ptr += 2;
}

fn bxl(operand: u64, mem: &mut Memory) {
    // register B takes the bitwise XOR of the B register and the literal operand
    mem.b ^= operand;
    mem.ptr += 2;
}

fn bst(operand: u64, mem: &mut Memory) {
    // store X % 8 in the B register
    mem.b = combo(operand, mem) % 8;
    mem.ptr += 2;
}

fn jnz(operand: u64, mem: &mut Memory) {
    // jump the instruction pointer to the index given by the literal operand unless A is zero
    if mem.a != 0 {
        mem.ptr = operand as usize;
    } else {
        mem.ptr += 2;
    }
}

#[allow(unused_variables)]
fn bxc(operand: u64, mem: &mut Memory) {
    // bitwise XOR register B with register C
    mem.b ^= mem.c;
    mem.ptr += 2;
}

fn out(operand: u64, mem: &mut Memory) {
    // output the least significant octal of the value given by the combo operand
    mem.out.push(combo(operand, mem) % 8);
    mem.ptr += 2;
}

fn bdv(operand: u64, mem: &mut Memory) {
    // same as adv, but stored in the B register
    mem.b = mem.a / 2u64.pow(combo(operand, mem) as u32);
    mem.ptr += 2;
}

fn cdv(operand: u64, mem: &mut Memory) {
    // same as adv, but stored in the C register
    mem.c = mem.a / 2u64.pow(combo(operand, mem) as u32);
    mem.ptr += 2;
}

fn execute(program: &[u64], input: u64) -> Vec<u64> {
    let mut reg = Memory {
        a: input,
        b: 0,
        c: 0,
        ptr: 0,
        out: Vec::new(),
    };

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

    reg.out
}

fn rec_search_octals(program: &[u64], stack: u64) -> Option<u64> {
    // part 2: finding an output that matches our program, using a recursive search
    // starting from the last digit and climbing up the tree if it appears to match so far.

    // the program operates on the input in 1-octal chunks, but each octal of output is
    // also affected by other bits in the A register.
    // testing two octals (6 bits) at a time avoids some (all?) false negatives.

    for i in 0b000000..0b111111 {
        let stacc = (stack << 6) + i;
        let test = execute(program, stacc);

        // if we return Some() we exit all this recursion immediately
        if test == program {
            println!("{stacc:b}\n{test:?}");
            println!("match found! {stacc}");
            return Some(stacc);
        } else {
            // don't ask me why we need to rev() this twice
            let reference: Vec<_> = program
                .iter()
                .rev()
                .take(test.len())
                .rev()
                .cloned()
                .collect();

            if test == reference {
                println!("{stacc:b}");
                if let Some(quine) = rec_search_octals(program, stacc) {
                    return Some(quine);
                }
            }
        }
    }

    // if the for loop ends, this particular recursive branch is exhausted
    None
}

pub fn main(input: &str) {
    let (input_mem, input_prog) = input.split_once("\n\n").unwrap();

    let re = Regex::new(r"[0-9]+").unwrap();

    let mem: Vec<_> = re
        .find_iter(input_mem)
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();

    let program: Vec<u64> = re
        .find_iter(input_prog)
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();

    println!("{mem:?}");
    println!("{program:?}");
    println!("---");

    let output_part1 = execute(&program, mem[0]);
    println!("from starting memory: {output_part1:?}");

    // part 2!

    println!("looking for input that outputs this:");
    println!("{:?}", program);

    let Some(_output_part2) = rec_search_octals(&program, 0) else {
        panic!("weh............");
    };
}
