// day 8 - resonant collinearity

// the input for this puzzle represents a map of a city
// populated with antennas emitting a mind-control signal.
// if an antenna shares a frequency (char) with another,
// it creates a "resonant antinode" the same distance away
// on its opposite side.

// for part 2, each pair of antennas generates an entire line
// of antinodes at regular intervals, all the way out to
// the edge of the map.

// use crate::tools2d::Vec2;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    // adding two vectors together is the same as
    // applying an offset to some absolute coordinates.
    fn offset_by(&self, offset: &Self) -> Self {
        let x = self.x + offset.x;
        let y = self.y + offset.y;
        Vec2 { x, y }
    }

    // subtracting someone else's coordinates from your own
    // returns position of other relative to self.
    fn get_offset(&self, other: &Self) -> Self {
        let x = other.x - self.x;
        let y = other.y - self.y;
        Vec2 { x, y }
    }

    // multiplying each number in a vector by -1
    // is the same as rotating 180 degrees.
    fn invert(&self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }

    // part 2: multiplying all numbers in a vector by the same number
    // is the same as multiplying its magnitude.
    fn multiply(&self, factor: isize) -> Self {
        let x = self.x * factor;
        let y = self.y * factor;
        Vec2 { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Antenna {
    location: Vec2,
    frequency: char,
}

pub fn main(input: &str) {
    // this shits ALL in main baby
    // welcome to iterator hell
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let range = Vec2 {
        x: grid[0].len() as isize,
        y: grid.len() as isize,
    };

    // the set of all frequencies antennas can have.
    let frequencies: HashSet<char> = grid
        .iter()
        .flatten()
        .filter(|c| **c != '.')
        .map(|c| c.to_owned())
        .collect();

    // every character on this map is an antenna with a location and a frequency
    let antennas: Vec<Antenna> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c != '.')
                .map(move |(j, c)| {
                    let location = Vec2 {
                        y: i as isize,
                        x: j as isize,
                    };
                    let frequency = *c;
                    Antenna {
                        location,
                        frequency,
                    }
                })
        })
        .collect();

    // every antenna finds the antinode for every other antenna sharing its frequency
    // my extremely evil iterator !!!!!!!
    let nodes: HashSet<Vec2> = frequencies
        .iter()
        .flat_map(|freq| {
            antennas
                .iter()
                .filter(|antenna| antenna.frequency == *freq)
                .flat_map(|this| {
                    antennas
                        .iter()
                        .filter(move |other| this.frequency == other.frequency && other != &this)
                        .flat_map(move |other| {
                            // part 1: antinodes appear opposite from each {other} antenna
                            let offset: Vec2 = this.location.get_offset(&other.location).invert();
                            // part 2: antinodes appear at regular intervals along the same line
                            [1, -1].iter().flat_map(move |sign| {
                                (1..)
                                    .map(move |i| {
                                        this.location.offset_by(&offset.multiply(i * sign))
                                    })
                                    .take_while(move |i| {
                                        (0..range.x).contains(&i.x) && (0..range.y).contains(&i.y)
                                    })
                            })
                        })
                })
        })
        .collect();

    // map of the grid with antinodes added
    let grid_nodes: Vec<Vec<char>> = grid
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, c)| {
                    let v = Vec2 {
                        x: j as isize,
                        y: i as isize,
                    };
                    if nodes.contains(&v) {
                        '#'
                    } else {
                        *c
                    }
                })
                .collect()
        })
        .collect();

    // print the whole map... as a cute blinky radar screen
    loop {
        [&grid, &grid_nodes].iter().for_each(|gridmap| {
            println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            print!(" ");
            gridmap.iter().for_each(|line| {
                line.iter().for_each(|c| {
                    print!(" {c}");
                });
                print!("\n ");
            });

            println!("\nnumber of antinodes: {}", nodes.len());

            std::thread::sleep(std::time::Duration::from_secs(1));
        })
    }
}
