// day 14 - omorashi bunnies

// the input for this puzzle is a swarm of bunny robots blocking off access to the bathroom.
// simulate the bunnys for 100 steps and then do math on them please.

// for part 2, continue simulating the robots until you find their secret easter egg.

use crate::vec2::Vec2;
use regex::Regex;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Bunny {
    pos: Vec2,
    vel: Vec2,
}

impl Bunny {
    fn parse(text: &str) -> Bunny {
        let re = Regex::new(r"[-0-9]+").unwrap();

        let stats: Vec<_> = re
            .find_iter(text)
            .map(|m| m.as_str().parse::<isize>().unwrap())
            .collect();

        Bunny {
            pos: Vec2 {
                x: stats[0],
                y: stats[1],
            },
            vel: Vec2 {
                x: stats[2],
                y: stats[3],
            },
        }
    }

    fn step(&mut self, bounds: Vec2) {
        self.pos += self.vel;
        // tonight i learned rust's % operator is "remainder" and not "modulo"
        // so it works bad for negative numbers!!
        self.pos.x = self.pos.x.rem_euclid(bounds.x);
        self.pos.y = self.pos.y.rem_euclid(bounds.y);
    }
}

fn render(buns: &[Bunny], bounds: &Vec2, frame: usize) {
    std::thread::sleep(std::time::Duration::from_millis(250));
    std::process::Command::new("clear").status().unwrap();

    println!("frame {frame}");

    for y in 0..bounds.y {
        print!("  ");
        for x in 0..bounds.x {
            if let Some(_bouny) = buns.iter().find(|b| x == b.pos.x && y == b.pos.y) {
                print!("🐇");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

fn safety_tally(buns: &[Bunny]) {
    let [mut nw, mut ne, mut se, mut sw] = [0, 0, 0, 0];
    let mut centered = 0;

    for bunny in buns {
        if bunny.pos.x < 50 && bunny.pos.y < 51 {
            nw += 1;
        } else if bunny.pos.x > 50 && bunny.pos.y < 51 {
            ne += 1;
        } else if bunny.pos.x > 50 && bunny.pos.y > 51 {
            se += 1;
        } else if bunny.pos.x < 50 && bunny.pos.y > 51 {
            sw += 1;
        } else {
            centered += 1;
        }
    }

    println!("total bunnies: {}", buns.len());
    println!("{nw} * {ne} * {se} * {sw} with {centered} in center");
    let safety_factor = nw * ne * se * sw;
    println!("safety factor: {safety_factor}");
}

pub fn main(input: &str) {
    let mut bunnys: Vec<_> = input.lines().map(Bunny::parse).collect();

    let bounds = Vec2 { x: 101, y: 103 };

    for bun in &bunnys {
        println!(
            "bunny: pos {0},{1}, vel {2},{3} ",
            bun.pos.x, bun.pos.y, bun.vel.x, bun.vel.y
        );
    }

    // simulate bunnys

    for step in 1.. {
        for bunny in &mut bunnys {
            bunny.step(bounds);
        }

        render(&bunnys, &bounds, step);

        // yayyy

        if step == 100 {
            safety_tally(&bunnys);
        }

        // time to press enter approximately 8050 times ;3

        stdin().read_exact(&mut [0]).unwrap();
    }
}
