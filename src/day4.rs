const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

const DIRS: [(isize, isize); 8] = [
    (-1, 1),  //NW
    (0, 1),   //N
    (1, 1),   //NE
    (-1, 0),  //W
    (1, 0),   //E
    (-1, -1), //SW
    (0, -1),  //S
    (1, -1),  //SE
];

const DIAGONALS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

fn get_relative(
    grid: &[Vec<char>],
    coords: (usize, usize),
    offset: (isize, isize),
) -> Option<&char> {
    let (x, y) = coords;
    let (dx, dy) = offset;

    let new_x: isize = x as isize + dx;
    let new_y: isize = y as isize + dy;

    if let Some(line) = grid.get(new_y as usize) {
        if let Some(letter) = line.get(new_x as usize) {
            Some(letter)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn main(input: &str) {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total_xmas = 0;
    let mut total_x_mas = 0;

    // four! count 'em! FOUR nested for loops!!
    for (y, line) in input.iter().enumerate() {
        for (x, &letter) in line.iter().enumerate() {
            match letter {
                // XMAS
                'X' => {
                    for (dx, dy) in DIRS {
                        let mut word: Vec<char> = vec![letter];
                        for i in 1..4 {
                            if let Some(c) = get_relative(&input, (x, y), (dx * i, dy * i)) {
                                word.push(*c);
                            };
                        }
                        if word.as_slice() == XMAS {
                            total_xmas += 1;
                        }
                    }
                }

                //X-MAS
                'A' => {
                    let mut mas = 0;
                    for (dx, dy) in DIAGONALS {
                        if let Some('M') = get_relative(&input, (x, y), (dx, dy)) {
                            if let Some('S') = get_relative(&input, (x, y), (-dx, -dy)) {
                                mas += 1;
                            };
                        };
                    }
                    if mas == 2 {
                        total_x_mas += 1;
                    }
                }

                _ => {}
            }
        }
    }

    println!("total instances of xmas: {}", total_xmas);
    println!("total instances of x-mas: {}", total_x_mas);
}
