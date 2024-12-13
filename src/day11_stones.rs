// day 11 - psychic stones

// the scenario for this puzzle is kind of eerie -
// the input represents a row of stones with numbers (keys) engraved on them.
// the number of stones and the numbers engraved on them change every time you see them,
// according to a set of rules almost like a cellular automaton.

use itertools::Itertools;
use std::collections::HashMap;

// if i store the row of stones as a vector my computer gets warm for half an hour
// and then runs out of memory and crashes at generation 43.....
// a hashmap storing the number of instances of each key works awesome though!
fn insert_carefully(hs: &mut HashMap<usize, usize>, input: (usize, usize)) {
    let (key, amount) = input;
    if hs.contains_key(&key) {
        let existing = hs.get(&key).unwrap();
        hs.insert(key, amount + existing);
    } else {
        hs.insert(key, amount);
    }
}

fn step(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut rocks = HashMap::new();

    for (key, count) in stones {
        if key == 0 {
            insert_carefully(&mut rocks, (1, count));
        } else {
            let digits = key.to_string();
            if digits.len() % 2 == 0 {
                let (left, right) = digits.split_at(digits.len() / 2);
                let (a, b): (usize, usize) = (left.parse().unwrap(), right.parse().unwrap());
                for k in [a, b] {
                    insert_carefully(&mut rocks, (k, count));
                }
            } else {
                insert_carefully(&mut rocks, (key * 2024, count));
            }
        }
    }
    println!();

    rocks
}

pub fn main(input: &str) {
    let start: HashMap<usize, usize> = { input.split(' ').map(|x| x.parse().unwrap()).counts() };
    println!("{start:?}");

    let mut stones = start;
    println!("generation 0: {}", stones.values().sum::<usize>());

    for i in 1..=75 {
        stones = step(stones);
        println!("generation {i}: {}", stones.values().sum::<usize>());
    }
}
