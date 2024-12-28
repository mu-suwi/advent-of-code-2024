// day 22 - stalk market

// in this puzzle you are reimplementing a pseudorandom number generator
// to predict price changes in a video game stock market.

use itertools::Itertools;
use std::iter::repeat_n;

fn prune(x: u64) -> u64 {
    x % 2u64.pow(24) // in other words, x % 16777216
}

// super secret monkey prng hash function
fn secret(x: u64) -> u64 {
    let y = prune(x ^ (x << 6));
    let z = prune(y ^ (y >> 5));
    prune(z ^ (z << 11))
}

// every sequence of four price changes (from -9 to +9)
// that can occur given the price stays between 0 and 9
fn generate_quads() -> Vec<Vec<i8>> {
    repeat_n(0..=9, 5).multi_cartesian_product().map(|v| {
        v.windows(2).map(|win| {
            win[1] - win[0]
        }).collect()
    }).sorted().dedup().collect()
}

// search a buyer's history for the given price changes and sell at that price
fn find_quad(quad: &[i8], monkey: &[(i8, i8)]) -> Option<u64> {
    if let Some(arr) = monkey.windows(4).find(|win| {
        let (a, _) = win[0];
        let (b, _) = win[1];
        let (c, _) = win[2];
        let (d, _) = win[3];
        [a, b, c, d] == *quad
    }) {
        let (_, sell_price) = arr[3];
        Some(sell_price as u64)
    } else { None }
}

pub fn main(input: &str) {
    let secrets: Vec<u64> = input.lines().map(|n| n.parse().unwrap()).collect();

    let mut secret_sum = 0;

    for s in secrets.clone() {
        let mut x = s;
        for _ in 0..2000 {
            x = secret(x);
        }
        secret_sum += x;
    }
    println!("{secret_sum}: secret sum");

    // part 2

    println!("calculating deltas...");

    let mut digit_deltas: Vec<Vec<(i8, i8)>> = Vec::new();
    for s in secrets.clone() {
        let mut x = s;
        let mut deltas = Vec::new();
        for _i in 0..2000 {
            let a = (x % 10) as i8;
            x = secret(x);
            let b = (x % 10) as i8;
            let d = b - a;
            deltas.push((d, b));
        }
        digit_deltas.push(deltas);
    }

    println!("finding banana offers...");

    let all_quads = generate_quads();

    let mut banana_sums: Vec<u64> = Vec::new();

    for q in all_quads {
        let mut bananas = 0;
        for monkey in digit_deltas.clone() {
            bananas += find_quad(&q, &monkey).unwrap_or(0);
        }
        banana_sums.push(bananas);

        println!("{q:?} - {bananas}");
    }

    let banana_total = banana_sums.iter().max().unwrap();

    println!("total bananas: {banana_total}");
}
