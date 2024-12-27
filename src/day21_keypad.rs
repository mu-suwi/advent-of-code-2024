// day 21 - game inside a game inside a game

// in this puzzle, you are trying to enter a code into a keypad, by
// piloting a robot that can only move left, right, up or down 1 key at a time.
// movement keys for this robot are being manipulated by another robot which
// works the same way. and that robot has to be piloted by a third robot,
// which is controlled by you.

// numpad < robot < arrowkeys < robot < arrowkeys < robot < direct input

use crate::vec2;
use crate::vec2::Vec2;
use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
struct ButtonPusher {
    position: char, // the button it is currently standing on
    keypad: HashMap<char, Vec2>, // map of all keys it has access to
}

#[derive(Clone)]
struct MemoryCore {
    recursions: HashMap<(Vec<char>, u64), u64>,
    expansions: HashMap<Vec<char>, Vec<char>>,
    motions: HashMap<(char, char), Vec<char>>,
}

impl ButtonPusher {
    fn press(&mut self, button: char) -> Vec<char> {
        let here = self.keypad.get(&self.position).unwrap();
        let dest = self.keypad.get(&button).expect("inaccessible button");
        let diff = *dest - *here;

        let mut moves = VecDeque::new();

        match diff.y {
            1.. => {
                for _ in 0..diff.y {
                    moves.push_back('v');
                }
            }
            (..=-1) => {
                for _ in diff.y..0 {
                    moves.push_back('^');
                }
            }
            _ => {}
        }

        match diff.x {
            1.. => {
                for _ in 0..diff.x {
                    if here.x == 0 && dest.y == 3 {
                        moves.push_front('>');
                    } else {
                        moves.push_back('>');
                    }
                }
            }
            (..=-1) => {
                for _ in diff.x..0 {
                    if here.y == 3 && dest.x == 0 {
                        moves.push_back('<');
                    } else {
                        moves.push_front('<');
                    }
                }
            }
            0 => {}
        }

        self.position = button;
        moves.push_back('A');

        moves.into_iter().collect()
    }
}

fn numpad_motion(orig: char, dest: char) -> Vec<char> {
    #[rustfmt::skip]
    let numpad = HashMap::from([
        ('7', vec2!(0, 0)), ('8', vec2!(1, 0)), ('9', vec2!(2, 0)),
        ('4', vec2!(0, 1)), ('5', vec2!(1, 1)), ('6', vec2!(2, 1)),
        ('1', vec2!(0, 2)), ('2', vec2!(1, 2)), ('3', vec2!(2, 2)),
                            ('0', vec2!(1, 3)), ('A', vec2!(2, 3)),
    ]);

    let mut tiny_kong = ButtonPusher {
        position: orig,
        keypad: numpad,
    };

    tiny_kong.press(dest)
}

fn arrow_motion(orig: char, dest: char) -> Vec<char> {
    #[rustfmt::skip]
    let arrowkeys = HashMap::from([
                            ('^', vec2!(1, 3)), ('A', vec2!(2, 3)),
        ('<', vec2!(0, 4)), ('v', vec2!(1, 4)), ('>', vec2!(2, 4)),
    ]);

    let mut diddy_kong = ButtonPusher {
        position: orig,
        keypad: arrowkeys,
    };

    diddy_kong.press(dest)
}

fn build_motion_cache() -> HashMap<(char, char), Vec<char>> {
    let mut cache = HashMap::new();

    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'];
    for a in digits {
        for b in digits {
            let value = numpad_motion(a, b);
            cache.insert((a, b), value);
        }
    }

    let inputs = ['^', '<', 'v', '>', 'A'];
    for a in inputs {
        for b in inputs {
            let value = arrow_motion(a, b);
            cache.insert((a, b), value);
        }
    }

    cache
}

fn expand(
    motion: &[char],
    cache: &mut MemoryCore,
) -> Vec<char> {
    if let Some(expanded) = cache.expansions.get(motion) {
        return expanded.to_vec();
    }

    let mut moves = Vec::new();

    let mut movelist = vec![('A', motion[0])];

    if motion.len() > 1 {
        for i in 0..(motion.len() - 1) {
            movelist.push((motion[i], motion[i + 1]));
        }
    }

    for pair in movelist {
        let (a, b) = pair;
        let mut mov = cache.motions.get(&(a, b)).unwrap().clone();
        moves.append(&mut mov);
    }

    cache.expansions.insert(motion.to_vec(), moves.clone());
    moves
}

fn explore(
    sequence: &[char],
    cuil: u64,
    cache: &mut MemoryCore,
) -> u64 {
    if cuil == 0 {
        return sequence.len() as u64;
    }

    if let Some(len) = cache.recursions.get(&(sequence.to_vec(), cuil)) {
        return *len;
    }

    let mut length = 0;
    let inner = expand(sequence, cache);
    for chonk in inner.chunk_by(|a, _b| *a != 'A') {
        length += explore(chonk, cuil - 1, cache);
    }

    cache.recursions.insert((sequence.to_vec(), cuil), length);
    length
}

fn comp_sum(codes: &[Vec<char>], cuil: u64) -> u64 {
    let mut complexity_sum = 0;

    let mut cache = MemoryCore {
        recursions: HashMap::new(),
        expansions: HashMap::new(),
        motions: build_motion_cache(),
    };

    for code in codes {
        let simian_slam = explore(
            code,
            cuil + 1,
            &mut cache,
        );

        let code_value: u32 = code
            .iter()
            .filter(|c| **c != 'A')
            .map(|c| c.to_digit(10).unwrap_or(0))
            .fold(0, |acc, x| acc * 10 + x);

        complexity_sum += code_value as u64 * simian_slam;
        println!("complexity for {code:?}: {code_value} + {simian_slam}",);
    }
    complexity_sum
}

pub fn main(input: &str) {
    let codes: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let complexity_sum = comp_sum(&codes, 2);
    println!("sum of code complexities (2 cuil): {complexity_sum}");

    // part 2

    let complex_complexity_sum = comp_sum(&codes, 25);
    println!("sum of code complexities (25 cuil): {complex_complexity_sum}");
}
