use std::{cmp::Ordering, fs::read_to_string};

fn is_safe(report: &[u32]) -> bool {
    let mut flag_unsafe = false;
    let pairs: Vec<&[u32]> = report.windows(2).collect();
    let mut contours: Vec<Ordering> = Vec::new();

    for pair in pairs {
        let (a, b) = (pair[0], pair[1]);
        if !(1..=3).contains(&a.abs_diff(b)) {
            flag_unsafe = true;
        }

        contours.push(a.cmp(&b));
    }

    let c = &contours[0];
    if !(contours.iter().all(|x| x == c)) {
        flag_unsafe = true;
    }

    !flag_unsafe
}

fn main() {
    let mut total_safe = 0;

    let reports: Vec<Vec<u32>> = read_to_string("input.txt")
        .expect("file read error")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().expect("parse error"))
                .collect()
        })
        .collect();

    for report in reports {
        let mut flag_safe = false;

        if is_safe(&report) {
            flag_safe = true;
        } else {
            // part 2 problem dampener
            for bad_level in 0..report.len() {
                let mut damp_report = report.clone();
                damp_report.remove(bad_level);
                if is_safe(&damp_report) {
                    flag_safe = true;
                }
            }
        }

        if flag_safe {
            total_safe += 1;
        }
    }

    println!("total safe: {}", total_safe);
}
