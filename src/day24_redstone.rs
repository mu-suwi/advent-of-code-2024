// day 24 -- redstone

// time to simulate a bunch of logic gates yayyyy

use std::collections::HashMap;

use itertools::Itertools;

type WireMap<'a> = HashMap<&'a str, Option<u8>>;

#[derive(Clone, Copy, Debug)]
struct Function<'a> {
    a: &'a str,
    b: &'a str,
    gate: &'a str,
    output: &'a str,
}

fn propogate<'a>(
    constants: &WireMap<'a>,
    functions: &[Function<'a>],
) -> WireMap<'a> {
    let mut wires: WireMap = constants.clone();
    for fun in functions.iter() {
        for wire in [fun.a, fun.b, fun.output] {
            if wires.contains_key(&wire) {
                continue;
            }
            wires.insert(wire, None);
        }
    }

    while !wires
        .iter()
        .filter(|(w, _)| w.starts_with('z'))
        .all(|(_, v)| v.is_some())
    {
        let funcs: Vec<_> = functions
            .iter()
            .filter(|f| {
                let [a, b, o] = [f.a, f.b, f.output];
                wires.get(&a).unwrap().is_some()
                    && wires.get(&b).unwrap().is_some()
                    && wires.get(&o).unwrap().is_none()
            })
            .collect();

        for fun in funcs {
            let Some(Some(a)) = wires.get(&fun.a) else {
                panic!();
            };
            let Some(Some(b)) = wires.get(&fun.b) else {
                panic!();
            };
            let out = match fun.gate {
                "AND" => a & b,
                "OR" => a | b,
                "XOR" => a ^ b,
                _ => 0,
            };
            wires.insert(fun.output, Some(out));
        }
    }

    wires
}

fn pins_to_u64(c: char, wires: &WireMap) -> u64 {
    let pins: Vec<_> = wires.iter().filter(|(k, _)| k.starts_with(c)).collect();
    let str: String = pins
        .iter()
        .sorted_unstable()
        .rev()
        .map(|(_k, v)| v.unwrap().to_string())
        .collect();
    u64::from_str_radix(&str, 2).unwrap()
}

pub fn main(input: &str) {
    let (constants, functions) = input.split_once("\n\n").unwrap();

    let constants: WireMap = constants
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|pair| {
            let (a, b) = pair;
            match b {
                "1" => (a, Some(1)),
                "0" => (a, Some(0)),
                _ => (a, None),
            }
        })
        .collect();

    let functions: Vec<Function> = functions
        .lines()
        .map(|line| {
            let params: Vec<_> = line.split(" ").collect();
            Function {
                a: params[0],
                b: params[2],
                gate: params[1],
                output: params[4],
            }
        })
        .collect();

    let wires = propogate(&constants, &functions);

    let part1_output = pins_to_u64('z', &wires);

    println!("total wires: {}", wires.len());

    println!("{part1_output:b}");
    println!("{part1_output}");

    // part 2

    // let mut permutations = repeat_n(0..functions.len(), 4)
    //     .multi_cartesian_product()
    //     .filter(|v| v.iter().all_unique());

    // it turns out checking 222 to the 4th possible permutations is not
    // how this puzzle was meant to be solved LMAO

    let (x, y) = (pins_to_u64('x', &constants), pins_to_u64('y', &constants));

    let z_should_be = x + y;

    println!("{part1_output:b}\n{z_should_be:b}");

    unimplemented!() // i ended up solving it by hand.

    // it turns out having experience with redstone comes in handy when
    // the puzzle involves poking around in the guts of a binary adder!

    //        carry in _________
    //                    ______[XOR]_________ z04
    //           /[XOR]__/
    //   x04 ___//       \___[ & ]__
    //   y04 ___X____              _[ OR ]__ carry out
    //          \____[ & ]________/

}
