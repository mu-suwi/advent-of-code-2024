// day 16 - the maze

// this puzzle is simple - the input is a maze. you start at S and face east.
// going straight incurs 1 point and turning incurs 1000. find the lowest possible score!

use std::collections::{HashMap, HashSet};

use crate::tools2d::Vec2;

// breadth-first search seems like a good way to go about this...
// let's try making a maze-running rat that duplicates itself at every branch.

#[derive(Clone)]
struct Rat {
    pos: Vec2, // location (as coords)
    dir: Vec2, // compass direction (as unit vector)
    score: usize,
    sleep_for: usize,
    stickers: Vec<Vec2>, // my solution for part 2 that makes this take 15 minutes
}

fn get_tile(world: &[Vec<char>], coords: Vec2) -> char {
    let x = coords.x as usize;
    let y = coords.y as usize;
    if (y >= world.len()) | (x >= world[0].len()) {
        return '#';
    }
    world[y][x]
}

fn update_node(nodes: &mut HashMap<(Vec2, Vec2), usize>, key: (Vec2, Vec2), score: usize) -> bool {
    if let Some(existing) = nodes.get(&key) {
        if *existing < score {
            // println!("failed to update node: {:?}", existing);
            return false;
        }
    }
    nodes.insert(key, score);
    // println!("updating node: {:?}", (key, score));
    true
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
    let mut visited: HashMap<(Vec2, Vec2), usize> = HashMap::new(); // store a score @ each node
    let mut score_submissions: HashSet<usize> = HashSet::new();
    let mut winners_area: Vec<Rat> = Vec::new();
    let mut best_so_far: usize = 999_999;

    let starter_rat = Rat {
        pos: start,
        dir: Vec2::RIGHT,
        score: 0,
        sleep_for: 0,
        stickers: Vec::new(),
    };

    rats.push(starter_rat);

    println!("mapping maze...");

    while !&rats.is_empty() {
        for (i, rat) in rats.clone().into_iter().enumerate().rev() {
            // after turning a corner, you'll sleep for 1000 ticks
            // so that rats with lower scores stay in the lead.
            if rat.sleep_for > 0 {
                rats[i].sleep_for -= 1;
                rats[i].score += 1;
                continue;
            }

            // every rat has a sticker collection containing every single tile they've seen
            rats[i].stickers.push(rat.pos);

            // every time the path branches left or right, spawn a new rat facing that direction.
            let [left, right] = [rat.dir.rotate_ccw(), rat.dir.rotate_cw()];
            for branch in [left, right] {
                if get_tile(&maze, rat.pos + branch) == '.' {
                    // every rat that passes here records their score on this node.
                    // only spawn a new rat if our score is the best so far.
                    let updated = update_node(&mut visited, (rat.pos, branch), rat.score);
                    if updated {
                        let new_rat = Rat {
                            pos: rat.pos + branch,
                            dir: branch,
                            score: rat.score + 1, // due to being 1 step ahead of parent
                            sleep_for: 1000,      // sleep timer
                            stickers: rats[i].stickers.clone(), // wehhhhhh TT~TT computer so warm
                        };
                        rats.push(new_rat);
                    }
                }
            }

            // step!
            rats[i].pos = rat.pos + rat.dir;
            rats[i].score += 1;

            match get_tile(&maze, rat.pos) {
                'E' => {
                    // we reached the goal!
                    rats[i].stickers.push(rat.pos);
                    if rat.score <= best_so_far {
                        best_so_far = rat.score;
                        score_submissions.insert(rat.score);
                        // part 2: now we need to collect stickers... i always hate this part
                        winners_area.push(rats[i].clone());
                    }
                    rats.swap_remove(i);
                }
                '#' => {
                    // reached the end of a hallway
                    rats.swap_remove(i);
                }
                _ if rat.score > 100_000 => {
                    // you must want to go home...
                    rats.swap_remove(i);
                }
                _ => {}
            }
        }
    }

    println!("finished mapping maze");

    let top_score = score_submissions
        .into_iter()
        .min()
        .expect("no score submissions");

    println!("best score: {top_score}");

    // hoo wee

    println!("processing winners...");

    let mut good_seats: HashSet<Vec2> = HashSet::new();

    for rat in winners_area {
        for sticker in rat.stickers {
            good_seats.insert(sticker);
        }
    }

    println!("good seats found: {}", good_seats.len());

    for i in 0..maze.len() {
        println!();
        for j in 0..maze[0].len() {
            let here = Vec2 {
                x: j as isize,
                y: i as isize,
            };
            if get_tile(&maze, here) == '#' {
                print!("░");
            } else if good_seats.contains(&here) {
                print!("O");
            } else {
                print!(" ");
            }
        }
    }
    println!();
}
