use regex::Regex;

pub fn main(input: &str) {
    let enabled: Vec<&str> = input
        .split_inclusive("do()")
        .map(|substring| {
            let (until_dont, _) = substring.split_once("don't()").unwrap_or((substring, "_"));
            until_dont
        })
        .collect();

    let reg = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("typed it wrong");
    // matches "mul(1,2)", "mul(123,45)" etc.

    let mut total = 0;

    for block in enabled {
        for (_, [a_str, b_str]) in reg.captures_iter(block).map(|capture| capture.extract()) {
            let (a, b) = (
                a_str.parse::<isize>().unwrap(),
                b_str.parse::<isize>().unwrap(),
            );
            total += a * b;
        }
    }

    println!("total: {}", total)
}
