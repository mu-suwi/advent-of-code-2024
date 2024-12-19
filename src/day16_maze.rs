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
                    score_submissions.insert(rat.score);
                    rats.swap_remove(i);
                    continue;
                }
                '#' => {
                    rats.swap_remove(i);
                    continue;
                }
                _ if visited.contains(&(rat.pos, rat.dir)) => {
                    rats.swap_remove(i);
                    continue;
                }
                _ if rat.score > 100_000 => {
                    rats.swap_remove(i);
                    continue;
                }
                _ => {}
            }

            if rat.sleep_countdown > 0 {
                rats[i].sleep_countdown -= 1;
                rats[i].score += 1;
                continue;
            }

            if get_tile(&maze, rat.pos + rat.dir.rotate_ccw()) == '.' {
                let new_rat = Rat {
                    pos: rat.pos + rat.dir.rotate_ccw(),
                    dir: rat.dir.rotate_ccw(),
                    score: rat.score,
                    sleep_countdown: 1001,
                };
                rats.push(new_rat);
                visited.insert((rat.pos, rat.dir));
            }

            if get_tile(&maze, rat.pos + rat.dir.rotate_cw()) == '.' {
                let new_rat = Rat {
                    pos: rat.pos + rat.dir.rotate_cw(),
                    dir: rat.dir.rotate_cw(),
                    score: rat.score,
                    sleep_countdown: 1001,
                };
                rats.push(new_rat);
                visited.insert((rat.pos, rat.dir));
            }

            rats[i].pos = rat.pos + rat.dir;
            rats[i].score += 1;
        }
    }

    for score in score_submissions {
        println!("{score}");
    }
}
