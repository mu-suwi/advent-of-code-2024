use std::cmp::Ordering;

pub fn main(input: &str) {
    let lists: Vec<_> = input
        .split("\n\n")
        .map(|s| s.lines().collect::<Vec<_>>())
        .collect();

    let rules: Vec<_> = lists[0]
        .iter()
        .map(|rule| {
            rule.split('|')
                .map(|s| s.parse::<u32>().expect("parse int error"))
                .collect::<Vec<_>>()
        })
        .map(|rule| (rule[0], rule[1]))
        .collect();

    let updates = lists[1].iter().map(|upd8| {
        upd8.split(',')
            .map(|s| s.parse::<u32>().expect("parse int error"))
            .collect::<Vec<_>>()
    });

    let (correct, incorrect): (Vec<_>, Vec<_>) = updates.partition(|upd8| {
        for rule in &rules {
            for (i, value) in upd8.iter().enumerate() {
                if value == &rule.0 && upd8.split_at(i).0.contains(&rule.1) {
                    return false;
                }
            }
        }
        true
    });

    let sum_correct: u32 = correct.iter().map(|upd8| upd8[(upd8.len()) / 2]).sum();

    let sum_incorrect: u32 = incorrect
        .iter()
        .map(|upd8| {
            let mut u = upd8.clone();

            // did u know u can just Lie to rust about which of two numbers is bigger
            u.sort_by(|a, b| {
                for rule in &rules {
                    if (*a, *b) == *rule {
                        return Ordering::Greater;
                    }
                }
                Ordering::Less
            });

            // prove to me that this is freaking sorted
            println!("{upd8:?}");
            println!("{u:?}");
            println!("-----------------------------------");
            u
        })
        .map(|upd8| upd8[(upd8.len()) / 2])
        .sum();

    println!("total correct: {}", sum_correct);
    println!("total incorrect (corrected): {}", sum_incorrect);
}
