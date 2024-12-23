// day 18 - the sky is falling!

// in this puzzle, you are trapped in some sort of space that is slowly caving in and
// must pathfind to the exit while dodging the debris!

// for part 1, we just simulate 100 meteors and then pathfind through the result.
// ~~ i'm kind of dreading whatever part 2 has in store... ~~

// for part 2 return the coordinates of the first meteor that completely blocks the exit.

use std::{
    collections::{HashMap, VecDeque},
    ops::RangeInclusive,
};

use crate::vec2::Vec2;

const BOUNDS: RangeInclusive<isize> = 0..=70;

#[derive(Clone)]
struct Rat {
    pos: Vec2,
    score: usize,
    path: Vec<Vec2>,
}

fn populate_nodes(obstacles: &[Vec2]) -> HashMap<(Vec2, Vec2), usize> {
    let mut nodes = HashMap::new();
    // draw a line between every node and every other node.
    for y in BOUNDS {
        for x in BOUNDS {
            let here = Vec2 { x, y };
            if obstacles.contains(&here) {
                continue;
            }

            for dir in Vec2::COMPASS {
                let there = here + dir;
                if !obstacles.contains(&there)
                    & BOUNDS.contains(&there.x)
                    & BOUNDS.contains(&there.y)
                {
                    // and give that line a very big number to hold also
                    nodes.insert((here, dir), usize::MAX);
                }
            }
        }
    }

    nodes
}

// oh yeah optional function arguments? rust has those
fn render(obstacles: &[Vec2], path: Option<Vec<Vec2>>) {
    std::process::Command::new("clear").status().unwrap();

    let path = path.unwrap_or_default();

    for y in BOUNDS {
        println!();
        for x in BOUNDS {
            let here = Vec2 { x, y };
            match here {
                _ if obstacles.contains(&here) => {
                    print!("░░");
                }
                _ if path.contains(&here) => {
                    print!("🐁");
                }
                _ => {
                    print!("  ");
                }
            }
        }
    }
    println!();
}

fn find_exit(obstacles: &[Vec2]) -> Option<Vec<Vec2>> {
    let starter_rat = Rat {
        pos: Vec2::ZERO,
        score: 0,
        path: vec![Vec2::ZERO],
    };

    let mut rat_stack: VecDeque<Rat> = VecDeque::new();
    rat_stack.push_back(starter_rat);

    let goal = Vec2 { x: 70, y: 70 };

    let mut nodes = populate_nodes(obstacles);

    // dyke's traversal.
    while let Some(rat) = rat_stack.pop_front() {
        if rat.pos == goal {
            return Some(rat.path);
        }

        for dir in Vec2::COMPASS {
            let edge = (rat.pos, dir);
            let Some(record) = nodes.get(&edge) else {
                continue;
            };
            if rat.score < *record {
                nodes.insert(edge, rat.score);
                let mut new_rat = Rat {
                    pos: rat.pos + dir,
                    score: rat.score + 1,
                    path: rat.path.clone(),
                };
                new_rat.path.push(rat.pos + dir);
                rat_stack.push_back(new_rat);
            }
        }
    }
    None
}

pub fn main(input: &str) {
    let meteors: Vec<Vec2> = input
        .lines()
        .map(|a| {
            let (x, y) = a.split_once(",").unwrap();
            Vec2 {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let mut obstacles = Vec::new();
    let mut blockage = Vec2::ZERO;
    let mut fastest_path = 0;

    for meteor in meteors {
        obstacles.push(meteor);
        let found_path = find_exit(&obstacles);

        render(&obstacles, found_path.clone());

        // part 1 goal
        if found_path.is_some() && obstacles.len() == 1024 {
            fastest_path = found_path.unwrap().len();
        } else
        // part 2 goal
        if found_path.is_none() {
            blockage = meteor;
            break;
        };
    }

    println!("shortest path after 1024 falls: {fastest_path}");
    println!(
        "exit blocked by {blockage:?} after {} falls",
        obstacles.len()
    );
}
