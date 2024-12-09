use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn offset_by(&self, offset: &Self) -> Self {
        let x = self.x + offset.x;
        let y = self.y + offset.y;
        Vec2 { x, y }
    }

    // returns position of other relative to self
    fn get_offset(&self, other: &Self) -> Self {
        let x = other.x - self.x;
        let y = other.y - self.y;
        Vec2 { x, y }
    }

    fn invert(&self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Antenna {
    location: Vec2,
    frequency: char,
}

pub fn main(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let frequencies: HashSet<char> = grid
        .iter()
        .flatten()
        .filter(|c| **c != '.')
        .map(|c| c.to_owned())
        .collect();

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
                        .map(|other| {
                            let offset: Vec2 = this.location.get_offset(&other.location).invert();
                            let node: Vec2 = this.location.offset_by(&offset);
                            node
                        })
                        .filter(|node| {
                            let range_x = 0..grid[0].len() as isize;
                            let range_y = 0..grid.len() as isize;
                            range_x.contains(&node.x) && range_y.contains(&node.y)
                        })
                })
        })
        .collect();

    // map of the grid with nodes added
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
        for i in [&grid, &grid_nodes] {
            println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            print!(" ");
            i.iter().for_each(|line| {
                line.iter().for_each(|c| {
                    print!(" {c}");
                });
                print!("\n ");
            });

            println!("\nnumber of antinodes: {}", nodes.len());

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
