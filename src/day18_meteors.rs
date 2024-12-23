// day 18 - the sky is falling!

// in this puzzle, you are trapped in some sort of space that is slowly caving in and
// must pathfind to the exit while dodging the debris!

// for part 1, we just simulate 100 meteors and then pathfind through the result.
// i'm kind of dreading whatever part 2 has in store...

use crate::vec2::Vec2;

pub fn main(input: &str) {
    let meteors: Vec<_> = input
        .lines()
        .map(|a| {
            let (x, y) = a.split_once(",").unwrap();
            Vec2 {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    println!("{meteors:?}");
}
