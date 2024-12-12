// day 10 - hiking!

// the input for this puzzle represents an elevation map.
// every 0 is a trailhead - give each trailhead a score
// based on how many 9's it can reach by taking single orthogonal steps
// from one digit to an adjacent digit exactly 1 greater.
// then sum the scores of all trailheads.

pub fn main(input: &str) {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("unknown digit??") as u8)
                .collect()
        })
        .collect();
}
