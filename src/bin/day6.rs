use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("./input/day6.txt").unwrap();
    let edges: Vec<(&str, &str)> = input.split('\n').filter(|e| !e.is_empty()).map(|e| {
        let parts: Vec<&str> = e.split(')').collect();
        (parts[0], parts[1])
    }).collect();
    dbg!(part1(&edges));
    dbg!(part2(&edges));
}

fn part1(edges: &[(&str, &str)]) -> u64 {
    let mut parents: HashMap<&str, &str> = HashMap::new();
    for (parent, node) in edges {
        parents.insert(node, parent);
    }
    let mut count = 0;
    for (_, node) in edges {
        let mut parent = node;
        while parents.contains_key(parent) {
            count += 1;
            parent = parents.get(parent).unwrap();
        }
    }
    count
}

fn part2(edges: &[(&str, &str)]) -> u64 {
    let mut parents: HashMap<&str, &str> = HashMap::new();
    for (parent, node) in edges {
        parents.insert(node, parent);
    }
    let mut visited: HashMap<&str, u64> = HashMap::new();
    let mut node = parents.get("YOU").unwrap();
    let mut count = 0;
    while parents.contains_key(node) {
        count += 1;
        node = parents.get(node).unwrap();
        visited.insert(node, count);
    }
    node = parents.get("SAN").unwrap();
    count = 0;
    while parents.contains_key(node) {
        count += 1;
        node = parents.get(node).unwrap();
        if let Some(other_path) = visited.get(node) {
            return count + *other_path;
        }
    }
    panic!("no route");
}

#[test]
fn test_part1() {
    assert_eq!(part1(&[
        ("COM", "B"),
        ("B", "C"),
        ("C", "D"),
        ("D", "E"),
        ("E", "F"),
        ("B", "G"),
        ("G", "H"),
        ("D", "I"),
        ("E", "J"),
        ("J", "K"),
        ("K", "L"),
    ]), 42);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&[
        ("COM", "B"),
        ("B", "C"),
        ("C", "D"),
        ("D", "E"),
        ("E", "F"),
        ("B", "G"),
        ("G", "H"),
        ("D", "I"),
        ("E", "J"),
        ("J", "K"),
        ("K", "L"),
        ("K", "YOU"),
        ("I", "SAN")
    ]), 4);
}
