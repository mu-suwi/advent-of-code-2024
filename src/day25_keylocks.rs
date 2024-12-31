// day 25 - code chronicle

// in this puzzle we're parsing keys and locks and seeing how many keys
// can fit in each lock by comparing their pin heights.
// sounds a little too easy..............

type Grid = Vec<Vec<char>>;

fn pin_sizes(grid: Grid) -> [u8; 5] {
    let mut pins = [0, 0, 0, 0, 0];
    for i in 0..5 {
        pins[i] = grid
            .iter()
            .skip(1)
            .take(5)
            .filter(|line| line[i] == '#')
            .count() as u8;
    }
    pins
}

fn fits(key: [u8; 5], lock: [u8; 5]) -> bool {
    for i in 0..5 {
        if key[i] + lock[i] > 5 {
            println!("lock {lock:?}\nkey  {key:?} does NOT fit");
            return false;
        }
    }
    println!("lock {lock:?}\nkey  {key:?} fits!");
    true
}

pub fn main(input: &str) {
    // first ever Vec<Vec<Vec<char>>>... merry xmas!
    let grids: Vec<Grid> = input
        .split("\n\n")
        .map(|text| text.lines().map(|line| line.chars().collect()).collect())
        .collect();

    let (locks, keys): (Vec<[u8; 5]>, Vec<[u8; 5]>) = {
        let (locks, keys): (Vec<Grid>, Vec<Grid>) = grids
            .into_iter()
            .partition(|grid| grid[0] == ['#', '#', '#', '#', '#']);
        let [locks, keys] =
            [locks, keys].map(|c| c.into_iter().map(pin_sizes).collect());
        (locks, keys)
    };

    let mut fits_total = 0;
    for lock in locks {
        fits_total += keys.iter().filter(|key| fits(**key, lock)).count();
    }

    println!("total fitting combinations: {fits_total}");

    // part 2

    // shhhhh... ***** ***** ***** ***** ***** ***** ***** ***** ***** *****
}
