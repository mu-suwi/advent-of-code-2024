// day 15 - sokoban bot

// the input for this puzzle is a map of a warehouse filled with pushable boulders,
// followed by a series of instructions for the hapless robot who's trapped inside.

// for part 2, the warehouse and all the boulders are now twice as wide.

use crate::tools2d::Vec2;
// use std::io::{stdin, Read};

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
    let (warehouse_map, anglebrackets) = input.split_once("\n\n").expect("error parsing input");

    let mut warehouse: Vec<Entity> = warehouse_map
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| {
                let coords = Vec2 {
                    x: j as isize,
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
        recursive_move(&mut warehouse, roomba, dir);

        // render_warehouse(&warehouse);
        // stdin().read_exact(&mut [0]).unwrap();
    }

    render_warehouse(&warehouse);
    println!();

    let mut gps_total = 0;

    for ent in warehouse.iter().filter(|e| e.tile == Tile::Movable) {
        gps_total += ent.coords.y * 100 + ent.coords.x;
    }

    println!("coordinate sum: {gps_total}");
}
