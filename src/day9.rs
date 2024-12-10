// day 9 - disk fragmenter

// the input of this puzzle (a very very long string of digits) represents a hard drive.
// each (xy) pair of digits represents a file with (x) blocks of data,
// followed by (y) blocks of free space.
// the puzzle involves taking each file (starting at the end,)
// chopping it up into individual blocks, and fitting those blocks
// into the "free space" starting from the left.
// each block should contain the index [i] of the file it came from.

pub fn main(input: &str) {
    // the entire disk, sorted into (file, free space) and indexed!
    let diskmap: Vec<(u8, u8)> = {
        let digits = input
            .chars()
            .map(|c| c.to_digit(10).expect("encountered unknown digit??") as u8)
            .chain(std::iter::once(0u8));
        digits
            .clone()
            .step_by(2)
            .zip(digits.skip(1).step_by(2))
            .collect()
    };

    println!("reading disk...");

    let blocks: Vec<Option<usize>> = diskmap
        .iter()
        .enumerate()
        .flat_map(|(index, (data, space))| {
            // i'm suffering iterator fatigue... let's do this imperatively please
            let mut file: Vec<Option<usize>> = Vec::new();
            for _j in 0..*data {
                file.push(Some(index));
            }
            for _k in 0..*space {
                file.push(None);
            }
            file
        })
        .collect();

    println!("fragmenting disk, please wait...");

    let fragged: Vec<Option<usize>> = {
        let mut disk = blocks.clone();

        for (i, block) in disk.clone().iter().enumerate() {
            if block.is_none() {
                let gertrude = disk.clone();
                let candidate = gertrude.iter().enumerate().rfind(|(_, b)| b.is_some());
                let s = match candidate {
                    Some((j, _)) => j,
                    None => {
                        break;
                    }
                };
                if i < s {
                    disk.swap(i, s);
                }
            }
        }

        disk
    };

    println!("generating checksum...");

    let checksum: usize = fragged
        .iter()
        .enumerate()
        .filter(|(_, b)| b.is_some())
        .map(|(index, block)| index * block.unwrap())
        .sum();

    println!("filesystem checksum: {checksum}");

    // // print the whole thing, for fun:

    // fragged.iter().for_each(|b| match b {
    //     Some(digit) => {
    //         print!("{digit}");
    //     }
    //     None => {
    //         print!(".");
    //     }
    // });
}
