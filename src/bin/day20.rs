use std::fs;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn main() {
    let input = fs::read_to_string("./input/day20_edited.txt").unwrap();
    let start_pos = find_start_pos(&input);
    dbg!(part1(&parse_map(&input), start_pos));
    dbg!(part2(&parse_map(&input), (start_pos.0, start_pos.1, 0)));
}

fn part1(map: &[Vec<MapTile>], start: (usize, usize)) -> usize {
    let mut todo = VecDeque::new();
    todo.push_back((start, start, 0));
    let mut visited = HashSet::new();
    while let Some((pos, come_from, steps)) = todo.pop_front() {
        for target in [
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1)
        ].iter() {
            if *target != come_from && visited.insert(*target) {
                match map[target.1][target.0] {
                    MapTile::Empty => {
                        todo.push_back((*target, pos, steps + 1))
                    }
                    MapTile::Wall => {
                        // nothing
                    }
                    MapTile::Target => {
                        return steps;
                    }
                    MapTile::Portal(_) => {
                        let dst = find_other_portal(map, *target);
                        todo.push_back((dst, dst, steps))
                    }
                }
            }
        }
    }
    panic!("target not found");
}

fn part2(map: &[Vec<MapTile>], start: (usize, usize, usize)) -> usize {
    let mut todo = VecDeque::new();
    todo.push_back((start, start, 0));
    let mut visited = HashSet::new();
    while let Some((pos, come_from, steps)) = todo.pop_front() {
        for target in [
            (pos.0, pos.1 + 1, pos.2),
            (pos.0, pos.1 - 1, pos.2),
            (pos.0 + 1, pos.1, pos.2),
            (pos.0 - 1, pos.1, pos.2)
        ].iter() {
            if *target != come_from && visited.insert(*target) {
                match map[target.1][target.0] {
                    MapTile::Empty => {
                        todo.push_back((*target, pos, steps + 1))
                    }
                    MapTile::Wall => {
                        // nothing
                    }
                    MapTile::Target => {
                        if target.2 == 0 {
                            return steps;
                        }
                    }
                    MapTile::Portal(_) => {
                        if target.1 == 1 || target.1 == 121 || target.0 == 1 || target.0 == 127 {
                            // outer portal
                            if target.2 != 0 {
                                let dst = find_other_portal(map, (target.0, target.1));
                                let dst = (dst.0, dst.1, target.2 - 1);
                                todo.push_back((dst, dst, steps))
                            }
                        } else {
                            let dst = find_other_portal(map, (target.0, target.1));
                            let dst = (dst.0, dst.1, target.2 + 1);
                            todo.push_back((dst, dst, steps))
                        }
                    }
                }
            }
        }
    }
    panic!("target not found");
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum MapTile {
    Empty,
    Wall,
    Target,
    Portal(char),
}

impl From<char> for MapTile {
    fn from(c: char) -> Self {
        match c {
            '.' | '@' => MapTile::Empty,
            '#' | ' ' => MapTile::Wall,
            'a'..='z' => MapTile::Portal(c),
            '0'..='9' => MapTile::Portal(c),
            '-' => MapTile::Target,
            _ => panic!("bad map tile {}", c)
        }
    }
}

fn find_other_portal(map: &[Vec<MapTile>], pos: (usize, usize)) -> (usize, usize) {
    if let MapTile::Portal(portal_type) = map[pos.1][pos.0] {
        for (y, line) in map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if let MapTile::Portal(c) = tile {
                    if *c == portal_type && (x, y) != pos {
                        return (x, y);
                    }
                }
            }
        }
    }
    panic!("other portal not found");
}

fn parse_map(input: &str) -> Vec<Vec<MapTile>> {
    input
        .lines()
        .map(|line|
            line.chars().map(MapTile::from).collect()
        ).collect()
}

fn find_start_pos(input: &str) -> (usize, usize) {
    input
        .lines()
        .find_position(|line| line.contains('@'))
        .map(|(y, line)|
            (line.chars().find_position(|c| *c == '@').unwrap().0, y)
        ).unwrap()
}

#[test]
fn test_find_other_portal() {
    let input = fs::read_to_string("./input/day20_edited.txt").unwrap();
    let map = parse_map(&input);
    assert_eq!(find_other_portal(&map, (41, 1)), (85, 35))
}

