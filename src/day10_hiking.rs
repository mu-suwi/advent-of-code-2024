// day 10 - fright fortress parkour challenge

// the input for this puzzle represents an elevation map.
// every 0 is a starting point - give each start point a score
// based on how many 9's it can reach by making single 1 block vertical jumps
// from one digit to an orthogonally adjacent digit exactly 1 block higher.
// then sum the scores of all start points.

use crate::tools2d::{Vec2, COMPASS};
use std::collections::HashSet;

struct Fortress {
    grid: Vec<Vec<u8>>,
    bounds: Vec2,
}

impl Fortress {
    fn get_all(&self, digit: u8) -> HashSet<Vec2> {
        let mut results = HashSet::new();
        for (i, line) in self.grid.iter().enumerate() {
            for (j, _d) in line.iter().enumerate().filter(|(_j, d)| **d == digit) {
                let coords = Vec2 {
                    x: j as isize,
                    y: i as isize,
                };
                results.insert(coords);
            }
        }
        results
    }

    fn get_tile(&self, index: Vec2) -> Option<u8> {
        if !(0..self.bounds.x).contains(&index.x) | !(0..self.bounds.y).contains(&index.y) {
            return None;
        }
        let (x, y) = (index.x as usize, index.y as usize);
        self.grid.get(y)?.get(x).copied()
    }
}

// can i really do it? recursion?
fn recursive_search(
    world: &Fortress,
    jump_from: Vec2,
    flags: &mut HashSet<Vec2>,
    visits: &mut usize,
) {
    // let's get our bearings...
    let here = world.get_tile(jump_from).unwrap();
    print!("{here}->");
    for dir in COMPASS {
        // glad i wrote that tools2d module, it makes adding Vec2s like this sooo intuitive
        let jump_to = jump_from + dir;
        let Some(tile) = world.get_tile(jump_to) else {
            // must be out of bounds
            continue;
        };
        if tile == here + 1 {
            if tile == 9 {
                // that's the top!
                print!("9!");
                println!();
                *visits += 1;
                flags.insert(jump_to);
            } else {
                // onward... -->
                recursive_search(world, jump_to, flags, visits);
                // <-- and back
            }
        }
    }
    // <-- back down the path
}

pub fn main(input: &str) {
    let world = {
        let grid: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("unknown digit??") as u8)
                    .collect()
            })
            .collect();

        let bounds = Vec2 {
            x: grid[0].len() as isize,
            y: grid.len() as isize,
        };

        Fortress { grid, bounds }
    };

    let zeroes = world.get_all(0);
    let nines = world.get_all(9);

    println!("number of zeroes: {}", zeroes.len());
    println!("number of nines: {}", nines.len());

    let mut total_flags = 0;
    let mut total_visits = 0;
    for z in zeroes {
        let mut flags: HashSet<Vec2> = HashSet::new();
        let mut visits = 0;

        recursive_search(&world, z, &mut flags, &mut visits);

        println!(
            "\nscore for {0},{1}: {2} flags, {3} visits",
            z.x,
            z.y,
            flags.len(),
            visits
        );
        total_flags += flags.len();
        total_visits += visits;

        // reality check: how many nines are less than 9 tiles away from you?
        // (this is mostly useless information)
        let close_nines = {
            nines
                .iter()
                .filter(|n| {
                    let distance = **n - z;
                    distance.taxi_dist() < 9
                })
                .count()
        };

        println!("total nines within range: {}", close_nines);
        println!();
    }

    println!("total reachability score: {}", total_flags);
    println!("total distinct paths: {}", total_visits);
}
