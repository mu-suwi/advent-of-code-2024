use std::collections::BinaryHeap;
use std::fs::read_to_string;

fn main() {
    //get input!!
    let lines = read_to_string("input.txt")
        .expect("file read error")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|hand| hand.parse::<u32>().expect("parse error"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    //create binary heaps!!
    let (mut left, mut right) = (BinaryHeap::new(), BinaryHeap::new());
    for line in lines {
        left.push(line[0]);
        right.push(line[1]);
    }

    //find the similarity score!!
    let mut total_similarity = 0;
    for i in &left {
        let similarity = &right.iter().filter(|x| *x == i).sum::<u32>();
        total_similarity += similarity;
    }

    //find the total distance!! (consuming the heaps)
    let mut total_distance = 0;
    for _ in 0..left.len() {
        let error = "off by one again u bimbo";
        let a = left.pop().expect(error);
        let b = right.pop().expect(error);

        total_distance += a.abs_diff(b);
    }

    println!("total distance: {}", total_distance);
    println!("similarity score: {}", total_similarity);
}
