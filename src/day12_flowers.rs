// day 12 - flower fields

// the input for this puzzle is a map of a garden with irregularly-shaped plots.
// find the area of each contiguous plot in tiles, and also find the area's perimeter.
// multiply each plot's area by its perimeter and sum the results.

// for part 2, find all continuous walls in the perimeter
// and multiply the area by the number of walls instead.

use crate::tools2d::{Vec2, COMPASS};
use std::collections::{HashSet, VecDeque};

struct Region {
    area: HashSet<Vec2>,
    perimeter: HashSet<(Vec2, Vec2)>,
}

fn get_tile_in(world: &[Vec<char>], index: Vec2) -> Option<char> {
    let (x, y) = (index.x as usize, index.y as usize);
    world.get(y)?.get(x).copied()
}

// i thought my recursive floodfill was sooo clever but all it did was blow the stack!
// here's a queue based floodfill. thanks wikipedia
fn floodfill(world: &[Vec<char>], mouse: Vec2) -> HashSet<Vec2> {
    let mut painted: HashSet<Vec2> = HashSet::new();
    let root_tile = get_tile_in(world, mouse).expect("tried to call floodfill out of bounds?");
    let mut queue = VecDeque::new();
    queue.push_back(mouse);
    while let Some(here) = queue.pop_front() {
        if !painted.contains(&here) {
            painted.insert(here);
            for dir in COMPASS {
                let there = here + dir;
                let Some(that_tile) = get_tile_in(world, there) else {
                    continue;
                };
                if that_tile == root_tile {
                    queue.push_back(there);
                }
            }
        }
    }
    painted
}

fn find_areas(world: &[Vec<char>]) -> Vec<HashSet<Vec2>> {
    let mut areas: Vec<HashSet<Vec2>> = Vec::new();
    let mut seen: HashSet<Vec2> = HashSet::new();
    for y in 0..world.len() {
        for x in 0..world[0].len() {
            let here = Vec2 {
                x: x as isize,
                y: y as isize,
            };

            if seen.contains(&here) {
                continue;
            }

            let painted = floodfill(world, here);
            painted.clone().iter().for_each(|v| {
                seen.insert(*v);
            });
            areas.push(painted);
        }
    }
    areas
}

// storing each border as a pair of tiles which claim to have a border between them
fn find_perimeter(area: &HashSet<Vec2>) -> HashSet<(Vec2, Vec2)> {
    let mut borders: HashSet<(Vec2, Vec2)> = HashSet::new();
    for tile in area {
        for dir in COMPASS {
            let other_tile = *tile + dir;
            if !area.contains(&other_tile) {
                borders.insert((*tile, other_tile));
            }
        }
    }
    borders
}

// returns false if you are in a line of borders and are not the top-most/left-most border
fn is_wall_leader(perimeter: &HashSet<(Vec2, Vec2)>, wall: (Vec2, Vec2)) -> bool {
    let (a, b) = wall;
    let north = Vec2 { x: 0, y: -1 };
    let west = Vec2 { x: -1, y: 0 };
    if a.x == b.x {
        // horizontal wall
        let one_west = (a + west, b + west);
        !perimeter.contains(&one_west)
    } else if a.y == b.y {
        // vertical wall
        let one_north = (a + north, b + north);
        !perimeter.contains(&one_north)
    } else {
        panic!("wall contained {0:?}, {1:?}", a, b);
    }
}

pub fn main(input: &str) {
    let field: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let areas = find_areas(&field);

    let regions: Vec<_> = areas
        .iter()
        .map(|area| Region {
            area: area.clone(),
            perimeter: find_perimeter(area),
        })
        .collect();

    let mut total = 0;
    for (i, region) in regions.iter().enumerate() {
        //part 2
        let walls = region
            .perimeter
            .iter()
            .filter(|w| is_wall_leader(&region.perimeter, **w))
            .count();

        let fence = region.area.len() * walls;
        total += fence;

        println!("region {i}");
        println!("     area: {}", region.area.len());
        println!("perimeter: {}", region.perimeter.len());
        println!("    walls: {}", walls);
        println!("    -- fence: {fence}");
        println!();
    }

    println!("total fence: {total}");
}
