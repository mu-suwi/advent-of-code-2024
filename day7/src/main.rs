use itertools::Itertools;
use std::iter::repeat_n;

const OPERATORS: [char; 3] = ['+', '*', '|'];

fn main() {
    // read the input as a list of pairs (test value, number sequence)
    let inputs: Vec<_> = std::fs::read_to_string("input.txt")
        .expect("where input?")
        .lines()
        .map(|line| line.split_once(':').expect("line contained no :"))
        .map(|(test, list)| {
            (
                test.parse::<u128>().expect("parse error on test value"),
                list.split_whitespace()
                    .map(|s| s.parse::<u128>().expect("parse error on operands"))
                    .collect::<Vec<u128>>(),
            )
        })
        .collect();

    // can't use a HashSet here because some test values are equal to each other!
    let mut matches = Vec::new();

    let opys = OPERATORS;

    for (test, equation) in &inputs {
        print!("\n{:?}", equation);

        // getting all permutations using Itertools::multi_cartesian_product
        let permutations: Vec<_> = repeat_n(opys.iter(), equation.len() - 1)
            .multi_cartesian_product()
            .collect();

        let mut yay = false;

        // test every permutation against the test value
        for op_sequence in permutations.iter() {
            print!("\n{test}: {}", equation[0]);
            let mut ops = op_sequence.clone();

            // i wanted to use iter().fold() here, but couldn't coax it to do operations LTR
            let mut acc = equation[0];
            for x in equation.iter().skip(1) {
                let operator = ops.pop();
                match operator {
                    Some('+') => {
                        // addition!
                        print!("+{}", x);
                        acc += x;
                    }
                    Some('*') => {
                        // multiplication!
                        print!("*{}", x);
                        acc *= x;
                    }
                    Some('|') => {
                        // concatenation nya
                        print!(" {}", x);
                        let cat = acc.to_string() + &x.to_string();
                        acc = cat.parse::<u128>().unwrap();
                    }
                    _ => {}
                }
            }
            let result = acc;

            print!(" = {}", result);
            if &result == test {
                print!(" = {} yayyyyyyyyyyyyyyy!!", test);
                yay = true;
            }
        }
        if yay {
            matches.push(*test);
        }

        // printing all those print!() macros takes for fucking ever but it's so pretty to watch
    }

    let output: u128 = matches.iter().sum();

    println!(
        "\n sum of all test values with matching permutations: {}",
        output
    );
}
