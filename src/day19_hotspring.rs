// day 19 - linen layout

// this puzzle involves a limited palette of "blocks" and determining whether
// a larger pattern can be arranged out of a combination of those blocks.

use std::collections::HashMap;

fn all_found_in_string<'a>(sentence: &str, words: &'a [&str]) -> Vec<&'a str> {
    words
        .iter()
        .filter(|w| sentence.contains(**w))
        .copied()
        .collect()
}

fn valid_compositions<'a>(
    sentence: &'a str,
    words: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(valids) = cache.get(sentence) {
        *valids
    } else {
        if sentence.is_empty() {
            return 1;
        }

        let mut acc = 0;
        for word in words {
            if let Some(sentence_out) = sentence.strip_prefix(word) {
                let valids = valid_compositions(sentence_out, words, cache);
                cache.insert(sentence_out, valids);
                acc += valids;
            }
        }

        acc
    }
}

pub fn main(input: &str) {
    let (towels, arrangements) = input.split_once("\n\n").unwrap();

    let mut towels: Vec<&str> = towels.split(", ").collect(); // shadowing for type change
    towels.sort_by_key(|a| usize::MAX - a.len());
    let towels = towels;

    let arrangements: Vec<&str> = arrangements.lines().collect();

    let mut valid_combos = 0;
    let mut valid_count = 0;
    let mut cache: HashMap<&str, usize> = HashMap::new();

    for arrange in arrangements {
        let words = all_found_in_string(arrange, &towels);
        let comps = valid_compositions(arrange, &words, &mut cache);
        println!("{arrange}: {comps}");
        if comps > 0 {
            valid_count += 1;
        }
        valid_combos += comps;
    }

    println!("strings with valid configurations: {valid_count}");
    println!("valid configurations total: {valid_combos}");
}
