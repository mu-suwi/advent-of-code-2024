use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn relative(&self, delta: (i32, i32)) -> Self {
        Vec2 {
            x: self.x + delta.0,
            y: self.y + delta.1,
        }
    }

    fn to_tile<'a>(&'a self, field: &'a [Vec<char>]) -> Option<&char> {
        // pastbin doesn't like it
        let x = self.x as usize;
        let y = self.y as usize;
        if let Some(row) = field.get(y) {
            if let Some(tile) = row.get(x) {
                Some(tile)
            } else {
                None
            }
        } else {
            None
        }
    }
}

enum MouseFate {
    Escaped(HashSet<Vec2>),
    Trapped,
    Lost,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Chuchu {
    coords: Vec2,
    direction: (i32, i32),
}

impl Chuchu {
    fn forward(&self) -> Vec2 {
        self.coords.relative(self.direction)
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            (0, -1) => (1, 0),  // east
            (1, 0) => (0, 1),   // south
            (0, 1) => (-1, 0),  // west
            (-1, 0) => (0, -1), // north
            _ => panic!("direction not properly initialized"),
        };
    }

    fn navigate(&self, field: &[Vec<char>]) -> MouseFate {
        let mut mouse = self.clone();
        let mut route: HashSet<Vec2> = HashSet::new();

        let mut turn_points: HashSet<Chuchu> = HashSet::new();

        let mut lifetime_turns = 0;

        loop {
            route.insert(mouse.coords.clone());

            let next = mouse.forward();

            match next.to_tile(field) {
                Some('.') | Some('^') => {
                    // forward!
                    mouse.coords = next;
                }
                Some('#') => {
                    // turn right!
                    if lifetime_turns > 1000 {
                        // we're not getting out of this one...
                        return MouseFate::Lost;
                    } else {
                        lifetime_turns += 1;
                    }

                    let here = mouse.clone();
                    if turn_points.contains(&here) {
                        // we made the same turn twice, we're cooked
                        return MouseFate::Trapped;
                    } else {
                        turn_points.insert(here);
                    }

                    mouse.turn_right();
                }
                None => {
                    return MouseFate::Escaped(route);
                }
                _ => {
                    panic!("encountered unknown character!");
                }
            }
        }
    }

    fn simulate_loop<'a>(&'a self, field: &[Vec<char>], new_block: &'a Vec2) -> Option<&Vec2> {
        // thank's pastebin i know you don't understand lifetimes i don't either
        let mut simulation = field.to_owned();
        let ghostmouse = self.clone();

        if new_block.to_tile(&simulation) == Some(&'.') {
            let x = new_block.x as usize;
            let y = new_block.y as usize;
            if (y > simulation.len()) | (x > simulation[y].len()) {
                return None;
            }
            simulation[y][x] = '#';

            match ghostmouse.navigate(&simulation) {
                MouseFate::Trapped => Some(new_block),
                _ => {
                    // mouse escapes; therefore no loop found
                    None
                }
            }
        } else {
            None
        }
    }
}

fn find_carrot_in(field: &[Vec<char>]) -> Option<Vec2> {
    for (y_index, row) in field.iter().enumerate() {
        if let Some(x_index) = row.iter().position(|&tile| tile == '^') {
            let x = x_index as i32;
            let y = y_index as i32;
            let coords = Some(Vec2 { x, y });
            println!(
                "found carrot: {1:?} at {0:?}",
                coords.as_ref()?,
                coords.as_ref()?.to_tile(field)?
            );
            return coords;
        }
    }
    None
}

pub fn main(input: &str) {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // find the ^
    let carrot = find_carrot_in(&input);

    // spawn the mice
    let mouse = Chuchu {
        coords: carrot.expect("couldn't find ^ in input"),
        direction: (0, -1), // carrot always facing north
    };

    println!("navigating...");

    let MouseFate::Escaped(escape_route) = mouse.navigate(&input) else {
        panic!("the mouse got lost in the maze...");
    };

    let mut total_traps: HashSet<Vec2> = HashSet::new();

    println!("simulating traps...");

    for vec in &escape_route {
        if let Some(block) = mouse.simulate_loop(&input, vec) {
            total_traps.insert(block.clone());
        }
    }

    println!("\n~~~ done!! ~~~");

    // visualization

    let mut output = input.clone();

    for vec in &escape_route {
        let x = vec.x as usize;
        let y = vec.y as usize;
        output[y][x] = '+';
    }

    for vec in &total_traps {
        let x = vec.x as usize;
        let y = vec.y as usize;
        output[y][x] = 'O';
    }

    #[allow(clippy::print_with_newline)]
    for row in output {
        for ch in row {
            print!("{}", ch);
        }
        print!("\n");
    }

    println!("~~~ mouse escaped!!! ~~~");
    println!("escape route: {} tiles", escape_route.len());
    println!("traps evaded: {}", total_traps.len());
}
