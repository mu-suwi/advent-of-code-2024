use std::collections::{HashMap, VecDeque};

use crate::vec2::Vec2;

fn get_tile(coords: Vec2, world: &[Vec<char>]) -> Option<char> {
    let x = coords.x as usize;
    let y = coords.y as usize;
    if y >= world.len() || x >= world[0].len() {
        return None;
    }
    Some(world[y][x])
}

fn find_tile(tile: char, world: &[Vec<char>]) -> Option<Vec2> {
    for x in 0..world.len() {
        for y in 0..world[0].len() {
            let here = Vec2 {
                x: x as isize,
                y: y as isize,
            };

            if get_tile(here, world) == Some(tile) {
                return Some(here);
            }
        }
    }
    None
}

fn get_path(world: &[Vec<char>]) -> HashMap<Vec2, usize> {
    // creates a hashmap where every location in the maze is
    // associated with its position on the linear path.
    let start = find_tile('S', world).expect("couldn't find starting line");
    let end = find_tile('E', world).expect("couldn't find goal");

    let mut path = HashMap::new();
    path.insert(start, 0);
    let mut mouse = start;
    let mut score = 0;

    // can make lots of assumptions with this one since the path is extremely linear
    'journey: while mouse != end {
        for dir in Vec2::COMPASS {
            let there = mouse + dir;
            let tile = get_tile(there, world).expect("tried to go out of bounds!");
            if (tile == '.' || tile == 'E') && !path.contains_key(&there) {
                mouse = there;
                score += 1;
                path.insert(there, score);
                continue 'journey;
            }
        }
    }

    path
}

// this is not a dijkstra's algorithm puzzle! don't be fooled!!
fn find_clips(path: &HashMap<Vec2, usize>) -> Vec<usize> {
    let mut saves = Vec::new();
    for (here, our_pos) in path {
        for dir in Vec2::COMPASS {
            let there = *here + dir * 2;
            let Some(new_pos) = path.get(&there) else {
                continue;
            };
            if new_pos > our_pos {
                let time_save = new_pos - our_pos - (dir * 2).taxi_dist();
                saves.push(time_save);
            }
        }
    }
    saves
}

fn all_in_distance(mouse: Vec2, distance: usize) -> Vec<Vec2> {
    // adapts my funny floodfill to get all vec2s within a certain taxi distance
    let mut seen: Vec<Vec2> = Vec::new();
    let mut queue: VecDeque<(Vec2, usize)> = VecDeque::new();
    queue.push_back((mouse, distance - 1));

    while let Some((here, to_go)) = queue.pop_front() {
        if !seen.contains(&here) && to_go > 0 {
            seen.push(here);
            for dir in Vec2::COMPASS {
                let there = here + dir;
                queue.push_back((there, to_go - 1));
            }
        }
    }

    seen
}

fn find_warps(path: &HashMap<Vec2, usize>) -> Vec<usize> {
    let mut saves = Vec::new();
    let mut tiles_to_go = path.len();

    for (here, position) in path {
        println!("calculating - {tiles_to_go} to go...");
        tiles_to_go -= 1;

        let warps = all_in_distance(*here, 20);
        for warp in warps {
            let Some(new_pos) = path.get(&warp) else {
                continue;
            };
            let time_cost = position + (warp - *here).taxi_dist();
            if new_pos > &time_cost {
                let time_save = new_pos - time_cost;
                saves.push(time_save);
            }
        }
    }

    saves
}

pub fn main(input: &str) {
    let world: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    println!("calculating glitchless route...");
    let glitchless = get_path(&world);

    println!("finding time saves...");
    let skips = find_clips(&glitchless);

    println!("finding time saves with warp...");
    let warps = find_warps(&glitchless);

    let big_skips: Vec<_> = skips.iter().filter(|save| **save >= 100).collect();
    println!(
        "wall clips that save at least 100 ticks: {}",
        big_skips.len()
    );

    let big_warps: Vec<_> = warps.iter().filter(|save| **save >= 100).collect();
    println!(
        "distance warps that save at least 100 ticks: {}",
        big_warps.len()
    );
}
