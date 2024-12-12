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

    // empty spaces will be a sequence of Nones
    // files will be a sequence of Some(file ID)
    let disk_as_blocks: Vec<Option<usize>> = {
        println!("reading disk...");
        diskmap
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
            .collect()
    };

    // part 1: just pack those blocks as tight as possible with no
    // regard to fragmentation. this seems unsanitary somehow
    let fragged: Vec<Option<usize>> = {
        println!("fragmenting disk, please wait...");
        let mut disk = disk_as_blocks.clone();

        for (i, block) in disk.clone().iter().enumerate() {
            if block.is_none() {
                let disk_scan = disk.clone();
                let candidate = disk_scan.iter().enumerate().rfind(|(_, b)| b.is_some());
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

    // part 2: like the above, but keep entire files (contiguous chunks of blocks) together.
    // take a file from the right and find the first area on the left where it will fit.
    let defragged: Vec<Option<usize>> = {
        println!("defragmenting disk, please wait...");
        let mut disk = disk_as_blocks.clone();

        // enough functional programming already... it's time for THE CLAW!!
        let mut the_claw = disk.clone().into_iter().enumerate().rev();

        loop {
            // the claw starts at far right and moves left until can't anymore
            let claw_position = the_claw.next();
            if claw_position.is_none() {
                break;
            }
            println!("claw position: {claw_position:?}");

            let (claw, data) = claw_position.unwrap();

            if data.is_none() {
                continue;
            }

            // the claw looks ahead and finds a contiguous row of Somes
            let file: Vec<_> = std::iter::once((claw, data))
                .chain(
                    the_claw
                        .clone()
                        .take_while(|(_c, b)| b.is_some() && b == &data),
                )
                .collect();
            println!("file: {file:?}");

            // starting from left, look for a window of Nones the same size
            let reverse_claw = disk.clone();
            let window = reverse_claw
                .windows(file.len())
                .enumerate()
                .take_while(|(c, _w)| c < &claw)
                .find(|(_c, w)| w.iter().all(|b| b.is_none()));
            println!("window: {window:?}");

            // if we find a match, swap into the new space...
            if window.is_some() {
                let (dest, _nones) = window.unwrap();
                if dest < claw {
                    for (i, (origin, some)) in file.iter().enumerate() {
                        let (a, b) = (origin, dest + i);
                        assert!(disk[b].is_none());
                        assert!(disk[*a].is_some());
                        disk.swap(*a, b);
                        println!("swapping {a}:{some:?} with {b}");
                    }
                }
            }

            for _i in 0..file.len() - 1 {
                the_claw.next();
            }
        }

        disk
    };

    // print the whole thing, for fun:
    defragged.iter().for_each(|b| match b {
        Some(id) => {
            let digit = id % 10;
            print!("{digit}");
        }
        None => {
            print!(".");
        }
    });

    // the actual puzzle output
    println!("\ngenerating checksums...");
    {
        fn checksum(disk: Vec<Option<usize>>) -> usize {
            disk.iter()
                .enumerate()
                .filter(|(_, b)| b.is_some())
                .map(|(index, block)| index * block.unwrap())
                .sum()
        }

        println!("filesystem checksum 1: {}", checksum(fragged));
        println!("filesystem checksum 2: {}", checksum(defragged));
    }
}
