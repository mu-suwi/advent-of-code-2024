// day 16 - the maze

// this puzzle is simple - the input is a maze. you start at S and face east.
// going straight incurs 1 point and turning incurs 1000. find the lowest possible score!

use std::collections::HashSet;

use crate::tools2d::Vec2;

// breadth-first search seems like a good way to go about this...
// let's try making a maze-running rat that duplicates itself at every branch.

#[derive(Clone)]
struct Rat {
    pos: Vec2,
    dir: Vec2,
    score: usize,
    sleep_countdown: usize,
}

fn get_tile(world: &[Vec<char>], coords: Vec2) -> char {
    let x = coords.x as usize;
    let y = coords.y as usize;
    world[y][x]
}

pub fn main(input: &str) {
    let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // find 'S' in the grid
    let start: Vec2 = maze
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_x, c)| **c == 'S')
                .map(|(x, _c)| Vec2 {
                    x: x as isize,
                    y: y as isize,
                })
        })
        .unwrap();

    // all the mutable state in one place, thx
    let mut rats: Vec<Rat> = Vec::new();
    let mut visited: HashSet<(Vec2, Vec2)> = HashSet::new();
    let mut score_submissions: HashSet<usize> = HashSet::new();

    let starter_rat = Rat {
        pos: start,
        dir: Vec2::RIGHT,
        score: 0,
        sleep_countdown: 0,
    };

    rats.push(starter_rat);

    while !&rats.is_empty() {
        for (i, rat) in rats.clone().into_iter().enumerate().rev() {
            match get_tile(&maze, rat.pos) {
                'E' => {
                    // we reached the goal!
                    score_submissions.insert(rat.score);
                    rats.swap_remove(i);
                    continue;
                }
                '#' => {
                    // reached the end of a hallway
                    rats.swap_remove(i);
                    continue;
                }
                _ if visited.contains(&(rat.pos, rat.dir)) => {
                    // another rat got here first
                    rats.swap_remove(i);
                    continue;
                }
                _ if rat.score > 100_000 => {
                    // you must want to go home...
                    rats.swap_remove(i);
                    continue;
                }
                _ => {}
            }

            // after turning a corner, you'll sleep for 1000 ticks
            // so that rats with lower scores stay in the lead.
            if rat.sleep_countdown > 0 {
                rats[i].sleep_countdown -= 1;
                rats[i].score += 1;
                continue;
            }

            // every time the path branches left or right, spawn a new rat facing that direction

            if get_tile(&maze, rat.pos + rat.dir.rotate_ccw()) == '.' {
                let new_rat = Rat {
                    pos: rat.pos + rat.dir.rotate_ccw(),
                    dir: rat.dir.rotate_ccw(),
                    score: rat.score,
                    sleep_countdown: 1001, // 1 more to acct for newborn rat being 1 step ahead
                };
                rats.push(new_rat);
                visited.insert((rat.pos, rat.dir));
            }

            if get_tile(&maze, rat.pos + rat.dir.rotate_cw()) == '.' {
                let new_rat = Rat {
                    pos: rat.pos + rat.dir.rotate_cw(),
                    dir: rat.dir.rotate_cw(),
                    score: rat.score,
                    sleep_countdown: 1001, // 1 more to acct for newborn rat being 1 step ahead
                };
                rats.push(new_rat);
                visited.insert((rat.pos, rat.dir));
            }

            // after all that, step forward as normal
            rats[i].pos = rat.pos + rat.dir;
            rats[i].score += 1;
        }
    }

    for score in score_submissions {
        println!("{score}");
    }
}
