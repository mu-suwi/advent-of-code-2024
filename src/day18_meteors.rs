// day 18 - the sky is falling!

// in this puzzle, you are trapped in some sort of space that is slowly caving in and
// must pathfind to the exit while dodging the debris!

// for part 1, we just simulate 100 meteors and then pathfind through the result.
// i'm kind of dreading whatever part 2 has in store...

use std::{
    collections::{HashMap, HashSet, VecDeque},
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
    // populate nodes
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
                    nodes.insert((here, dir), usize::MAX);
                }
            }
        }
    }

    nodes
}

fn render(obstacles: HashSet<Vec2>, path: HashSet<Vec2>) {
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

    let (kb, _) = meteors.split_at(1024);
    let obstacles: HashSet<Vec2> = kb.iter().copied().collect();
    let mut nodes = populate_nodes(kb);

    let starter_rat = Rat {
        pos: Vec2::ZERO,
        score: 0,
        path: vec![Vec2::ZERO],
    };

    let mut rat_stack: VecDeque<Rat> = VecDeque::new();
    rat_stack.push_back(starter_rat);

    let goal = Vec2 { x: 70, y: 70 };

    let mut min_steps = usize::MAX;
    let mut found_path: HashSet<_> = HashSet::new();

    // dyke's traversal.
    while let Some(rat) = rat_stack.pop_front() {
        if rat.pos == goal {
            min_steps = rat.score;
            found_path = rat.path.iter().copied().collect();
            break;
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

    render(obstacles, found_path);

    println!("minimum steps after 1024 fallen: {min_steps}");
}
