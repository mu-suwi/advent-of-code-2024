// day 15 - sokoban bot

// the input for this puzzle is a map of a warehouse filled with pushable boulders,
// followed by a series of instructions for the hapless robot who's trapped inside.

// for part 2, the warehouse and all the booulders are now twice as wide.

const PART: isize = 2; // maybe we can avoid mangling the code too much for part 2

use crate::tools2d::Vec2;
use std::collections::{HashSet, VecDeque};

// storing the world not as a 2D array, but as a list of entities with coordinates

#[derive(Debug, Copy, Clone)]
struct Entity {
    coords: Vec2,
    tile: Tile,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Wall,
    Movable,
    Alive,
}

// recursion is fun! but it got too complicated for part 2

fn recursive_move(world: &mut Vec<Entity>, move_from: Vec2, direction: Vec2) -> Option<Vec2> {
    let (mover_id, _) = world
        .iter()
        .enumerate()
        .find(|(_i, e)| e.coords == move_from)
        .expect("called recursive_move with no mover");

    let move_to = move_from + direction;

    if let Some(obstruction) = world.iter().find(|e| e.coords == move_to) {
        if obstruction.tile == Tile::Wall {
            return None;
        }
        if obstruction.tile == Tile::Movable {
            recursive_move(world, move_to, direction)?;
        }
    }

    world[mover_id].coords = move_to;

    Some(move_to)
}

fn render_warehouse(world: &[Entity]) {
    std::process::Command::new("clear").status().unwrap();
    for y in 0..50 {
        println!();
        for x in 0..50 {
            if let Some(ent) = world.iter().find(|v| v.coords == Vec2 { x, y }) {
                match ent.tile {
                    Tile::Wall => print!(" #"),
                    Tile::Movable => print!(" O"),
                    Tile::Alive => print!(" @"),
                }
            } else {
                print!(" .");
            }
        }
    }
    println!();
}

pub fn main(input: &str) {
    // parsing the input is half of every advent of code puzzle

    let (warehouse_map, anglebrackets) = input.split_once("\n\n").expect("error parsing input");

    let mut warehouse: Vec<Entity> = warehouse_map
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| {
                let coords = Vec2 {
                    x: j as isize * PART,
                    y: i as isize,
                };
                match c {
                    '#' => Some(Entity {
                        coords,
                        tile: Tile::Wall,
                    }),
                    'O' => Some(Entity {
                        coords,
                        tile: Tile::Movable,
                    }),
                    '@' => Some(Entity {
                        coords,
                        tile: Tile::Alive,
                    }),
                    _ => None,
                }
            })
        })
        .collect();

    let steps: Vec<_> = anglebrackets.chars().filter(|c| *c != '\n').collect();

    // action starts here

    for step in steps {
        let dir = match step {
            'v' => Vec2::DOWN,
            '^' => Vec2::UP,
            '>' => Vec2::RIGHT,
            '<' => Vec2::LEFT,
            _ => Vec2 { x: 0, y: 0 },
        };

        let roomba = warehouse
            .iter()
            .find(|e| e.tile == Tile::Alive)
            .expect("robot not found")
            .coords;

        big_move(&mut warehouse, roomba, dir, PART);

        // // uncomment this to view step-by-step:

        // render_big_warehouse(&warehouse, PART);
        // println!("roomba coords: {roomba:?}");
        // use std::io::{stdin, Read};
        // stdin().read_exact(&mut [0]).unwrap();
    }

    render_big_warehouse(&warehouse, PART);

    println!();

    let mut gps_total = 0;

    for ent in warehouse.iter().filter(|e| e.tile == Tile::Movable) {
        gps_total += ent.coords.y * 100 + ent.coords.x;
    }

    println!("coordinate sum: {gps_total}");
}

// part 2 logic beyond this point!

fn big_move(world: &mut Vec<Entity>, roomba: Vec2, direction: Vec2, part: isize) -> Option<Vec2> {
    // if this is part 1, use the old function instead ok. that's backwards compatbility
    if part == 1 {
        return recursive_move(world, roomba, direction);
    }

    let mut queue: VecDeque<_> = VecDeque::new();
    queue.push_back(roomba);

    let mut to_move: HashSet<usize> = HashSet::new();

    while let Some(move_from) = queue.pop_front() {
        let (mover_id, _) = world
            .iter()
            .enumerate()
            .find(|(_i, e)| e.coords == move_from)
            .expect(
                "pushed movable obstruction coords to queue, but no object found at those coords",
            );

        let move_to = move_from + direction;

        let wide = matches!(world[mover_id].tile, Tile::Movable);

        let obstructions: Vec<Entity> = world
            .iter()
            .filter(|e| {
                (e.coords == move_to)
                    | (e.coords + Vec2::RIGHT == move_to)
                    | (wide && e.coords + Vec2::LEFT == move_to)
            })
            .filter(|e| e.coords != move_from)
            .copied()
            .collect();

        // if this list of entities ever includes a wall, nobody goes anywhere

        for obstruction in obstructions {
            if obstruction.tile == Tile::Wall {
                return None;
            }

            if obstruction.tile == Tile::Movable {
                queue.push_back(obstruction.coords);
            }
        }

        to_move.insert(mover_id);
    }

    for id in to_move {
        world[id].coords += direction;
    }

    Some(roomba + direction)
}

fn render_big_warehouse(world: &[Entity], part: isize) {
    if part == 1 {
        render_warehouse(world);
        return;
    }

    let mut double_wide = false;
    let mut render_buffer: String = String::new();

    for y in 0..50 {
        render_buffer.push_str("\n ");
        for x in 0..100 {
            if double_wide {
                double_wide = false;
                continue;
            }

            if let Some(ent) = world.iter().find(|v| v.coords == Vec2 { x, y }) {
                match ent.tile {
                    Tile::Wall => {
                        render_buffer.push_str("##");
                        double_wide = true;
                    }
                    Tile::Movable => {
                        render_buffer.push_str("[]");
                        double_wide = true;
                    }
                    Tile::Alive => render_buffer.push('@'),
                }
            } else {
                render_buffer.push('.');
            }
        }
    }
    render_buffer.push('\n');

    // render!
    std::process::Command::new("clear").status().unwrap();
    println!("{render_buffer}");
}
