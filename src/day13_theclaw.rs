use crate::vec2::Vec2;
use regex::Regex;

// the input for this puzzle is a series of arcade machines.
// each machine has two buttons which move the claw by different amounts.
// pressing A costs 3 coins and pressing B costs 1.
// for each machine, find the minimum number of coins needed to reach that machine's prize.

struct Machine {
    a: Vec2,
    b: Vec2,
    prize: Vec2,
}

fn parse_machine(text: &str) -> Machine {
    let re = Regex::new(r"[0-9]+").unwrap();

    let stats: Vec<_> = re
        .find_iter(text)
        .map(|m| m.as_str().parse::<isize>().unwrap())
        .collect();

    Machine {
        a: Vec2 {
            x: stats[0],
            y: stats[1],
        },
        b: Vec2 {
            x: stats[2],
            y: stats[3],
        },
        prize: Vec2 {
            x: stats[4] + 10_000_000_000_000,
            y: stats[5] + 10_000_000_000_000,
        },
        // i accidentally put "*" instead of "+" here at first
        // and for a second i thought i had to invent a 128-bit vec2
        // to avoid overflow during arithmetic
    }
}

pub fn main(input: &str) {
    // parsing this one is gonna be a real doozy
    let machines: Vec<_> = input.split("\n\n").collect();

    let mut token_total = 0;

    for machine in machines {
        let m = parse_machine(machine);
        let (a, b, prize) = (m.a, m.b, m.prize);

        // i learned about Cramer's Rule from reddit today. thanks reddit
        // uhhhh welp. what am i gonna do, complain about spoilers for how math works?
        // it came out 4 billion years ago, c'mon

        let determinant = a.x * b.y - a.y * b.x;
        let a_times = (prize.x * b.y - prize.y * b.x) / determinant;
        let b_times = (a.x * prize.y - a.y * prize.x) / determinant;

        if a * a_times + b * b_times == prize {
            println!("prize found! {prize:?}");
            token_total += a_times * 3;
            token_total += b_times;
        }
    }

    println!("token total: {token_total}");
}
