use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn add_link<'a>(
    com: &'a str,
    link: &'a str,
    nodes: &mut HashMap<&'a str, Vec<&'a str>>,
) {
    let mut links = {
        if let Some(v) = nodes.get(com) {
            v.clone()
        } else {
            Vec::new()
        }
    };

    links.push(link);

    nodes.insert(com, links);
}

fn are_linked(a: &str, b: &str, nodes: &HashMap<&str, Vec<&str>>) -> bool {
    let Some(a_links) = nodes.get(a) else {
        panic!("{a:?} not found in list");
    };
    let Some(b_links) = nodes.get(b) else {
        panic!("{b:?} not found in list");
    };
    match (a_links.contains(&b), b_links.contains(&a)) {
        (true, true) => true,
        (false, false) => false,
        _ => {
            panic!("nodelist contained asymmetrical link")
        }
    }
}

fn is_linked_w_all(
    node: &str,
    party: &[&str],
    nodes: &HashMap<&str, Vec<&str>>,
) -> bool {
    for other in party {
        if !are_linked(node, other, nodes) {
            return false;
        }
    }
    true
}

fn find_one_more<'a>(
    party: &[&'a str],
    nodes: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<Vec<&'a str>> {
    let mut parties = Vec::new();

    for node in nodes.keys() {
        if party.contains(node) {
            continue;
        }

        if is_linked_w_all(node, party, nodes) {
            let mut new_party = party.to_vec();
            new_party.push(node);
            new_party.sort_unstable();
            parties.push(new_party);
        }
    }

    parties
}

fn next_size_up<'a>(
    parties: HashSet<Vec<&'a str>>,
    nodes: &HashMap<&'a str, Vec<&'a str>>,
) -> Option<HashSet<Vec<&'a str>>> {
    let p = parties
        .iter()
        .flat_map(|party| find_one_more(party, nodes))
        .collect::<HashSet<Vec<&str>>>();
    if p.is_empty() {
        return None;
    }
    Some(p)
}

pub fn main(input: &str) {
    let links: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split("-").collect())
        .collect();

    let nodes: HashMap<&str, Vec<&str>> = {
        let mut nodes = HashMap::new();
        for pair in &links {
            let (a, b) = (pair[0], pair[1]);
            add_link(a, b, &mut nodes);
            add_link(b, a, &mut nodes);
        }
        nodes
    };

    let trios: HashSet<_> = nodes
        .iter()
        .filter(|(k, _v)| k.starts_with('t'))
        .flat_map(|(k, v)| {
            v.iter()
                .combinations(2)
                .filter(|comb| are_linked(comb[0], comb[1], &nodes))
                .map(|party| {
                    vec![*k, *party[0], *party[1]]
                        .into_iter()
                        .sorted()
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>()
        })
        .collect();

    println!("parties of 3: {:?}", trios.len());

    // part 2

    let mut bumpin_parties: HashSet<Vec<&str>> =
        links.iter().cloned().collect();

    for i in 3.. {
        let Some(parties) = next_size_up(bumpin_parties.clone(), &nodes) else {
            break;
        };
        bumpin_parties = parties.into_iter().sorted().dedup().collect();
        println!("parties of {i}..... {:?}", bumpin_parties.len());
    }

    if bumpin_parties.len() == 1 {
        let party = bumpin_parties.drain().next().unwrap();
        println!("most bumpin party:");
        for (i, node) in party.iter().enumerate() {
            print!("{node}");
            if i < party.len() - 1 {
                print!(",");
            }
        }
        println!();
    }
}
