#![allow(clippy::type_complexity)]

use std::fs;
use itertools::Itertools;
use std::collections::{HashSet, HashMap, VecDeque};
use std::fmt::{Display, Formatter, Error, Write};

fn main() {
    let input = fs::read_to_string("./input/day18.txt").unwrap();
    dbg!(find_all_keys(&parse_map(&input), find_start_pos(&input)));
    let input = fs::read_to_string("./input/day18_part2.txt").unwrap();
    dbg!(find_all_keys_with_4_robots(&parse_map(&input), ((39, 39), (41, 41), (39, 41), (41, 39))));
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum MapTile {
    Empty,
    Wall,
    Door(char),
    Key(char),
}

impl From<char> for MapTile {
    fn from(c: char) -> Self {
        match c {
            '.' | '@' => MapTile::Empty,
            '#' => MapTile::Wall,
            'A'..='Z' => MapTile::Door(c),
            'a'..='z' => MapTile::Key(c),
            _ => panic!("bad map tile {}", c)
        }
    }
}

fn find_possible_moves(
    map: &[Vec<MapTile>],
    from: (usize, usize),
    keys: BitmapKey,
) -> HashMap<char, ((usize, usize), usize)> {
    let mut possible_moves = HashMap::new();
    struct Position {
        pos: (usize, usize),
        from: (usize, usize),
        steps: usize,
    }
    let mut todo = VecDeque::new();
    todo.push_back(
        Position {
            pos: from,
            from,
            steps: 0,
        }
    );
    let mut visited = HashSet::new();
    while let Some(pos) = todo.pop_front() {
        for target in [
            (pos.pos.0, pos.pos.1 + 1),
            (pos.pos.0, pos.pos.1 - 1),
            (pos.pos.0 + 1, pos.pos.1),
            (pos.pos.0 - 1, pos.pos.1)
        ].iter() {
            let tile = map[target.1][target.0]; // in bounds because map is surrounded by walls
            if *target != pos.from && visited.insert(*target) {
                match tile {
                    MapTile::Empty => {
                        todo.push_back(Position {
                            pos: *target,
                            from: pos.pos,
                            steps: pos.steps + 1,
                        });
                    }
                    MapTile::Wall => {}
                    MapTile::Door(c) => {
                        if keys.contains(c.to_ascii_lowercase()) {
                            todo.push_back(Position {
                                pos: *target,
                                from: pos.pos,
                                steps: pos.steps + 1,
                            });
                        }
                    }
                    MapTile::Key(c) => {
                        if keys.contains(c.to_ascii_lowercase()) {
                            todo.push_back(Position {
                                pos: *target,
                                from: pos.pos,
                                steps: pos.steps + 1,
                            });
                        } else {
                            let entry = possible_moves.entry(c).or_insert((*target, pos.steps + 1));
                            if entry.1 > pos.steps + 1 {
                                *entry = (entry.0, pos.steps + 1);
                            }
                        }
                    }
                }
            }
        }
    }
    possible_moves
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct BitmapKey {
    key: u64
}

impl Display for BitmapKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("Keys (")?;
        for i in 0..=26 {
            if self.key & (1 << i) != 0 {
                f.write_char(char::from(b'a' + i as u8))?;
            }
        }
        f.write_char(')')?;
        Ok(())
    }
}

impl BitmapKey {
    fn bitmap_index_for(key: char) -> u64 {
        1 << (key as u32 - b'a' as u32) as u64
    }

    fn contains(self, key: char) -> bool {
        self.key & Self::bitmap_index_for(key) != 0
    }

    fn insert(&mut self, key: char) -> bool {
        let did_have_entry = self.contains(key);
        self.key |= Self::bitmap_index_for(key);
        !did_have_entry
    }

    fn new() -> Self {
        Self {
            key: 0
        }
    }
}

fn find_all_keys(
    map: &[Vec<MapTile>],
    from: (usize, usize),
) -> usize {
    let mut todo = VecDeque::new();
    todo.push_back((from, 0, BitmapKey::new()));
    let mut result = std::usize::MAX;
    let mut visited = HashMap::new();
    while let Some((pos, steps, keys)) = todo.pop_front() {
        let moves = find_possible_moves(map, pos, keys);
        if moves.is_empty() && steps < result {
            result = steps;
        }
        for (key, (pos, steps_to_pos)) in &moves {
            let mut new_keys = keys;
            new_keys.insert(*key);
            if let Some(did_visit) = visited.get_mut(&(new_keys, *pos)) {
                if steps + *steps_to_pos < *did_visit {
                    todo.push_back((*pos, steps + *steps_to_pos, new_keys));
                    *did_visit = steps + *steps_to_pos;
                }
            } else {
                visited.insert((new_keys, *pos), steps + *steps_to_pos);
                todo.push_back((*pos, steps + *steps_to_pos, new_keys));
            }
        }
    }
    result
}

fn find_all_keys_with_4_robots(
    map: &[Vec<MapTile>],
    from: ((usize, usize), (usize, usize), (usize, usize), (usize, usize)),
) -> usize {
    let mut todo = VecDeque::new();
    todo.push_back((from, 0, BitmapKey::new()));
    let mut result = std::usize::MAX;
    let mut visited = HashMap::new();
    while let Some((pos, steps, keys)) = todo.pop_front() {
        let moves0 = find_possible_moves(map, pos.0, keys);
        let moves1 = find_possible_moves(map, pos.1, keys);
        let moves2 = find_possible_moves(map, pos.2, keys);
        let moves3 = find_possible_moves(map, pos.3, keys);
        if moves0.is_empty() && moves1.is_empty() && moves2.is_empty() && moves3.is_empty() && steps < result {
            result = steps;
        }
        for (key, (new_pos, steps_to_pos)) in &moves0 {
            let mut new_keys = keys;
            new_keys.insert(*key);
            if let Some(did_visit) = visited.get_mut(&(new_keys, (*new_pos, pos.1, pos.2, pos.3))) {
                if steps + *steps_to_pos < *did_visit {
                    todo.push_back(((*new_pos, pos.1, pos.2, pos.3), steps + *steps_to_pos, new_keys));
                    *did_visit = steps + *steps_to_pos;
                }
            } else {
                visited.insert((new_keys, (*new_pos, pos.1, pos.2, pos.3)), steps + *steps_to_pos);
                todo.push_back(((*new_pos, pos.1, pos.2, pos.3), steps + *steps_to_pos, new_keys));
            }
        }
        for (key, (new_pos, steps_to_pos)) in &moves1 {
            let mut new_keys = keys;
            new_keys.insert(*key);
            if let Some(did_visit) = visited.get_mut(&(new_keys, (pos.0, *new_pos, pos.2, pos.3))) {
                if steps + *steps_to_pos < *did_visit {
                    todo.push_back(((pos.0, *new_pos, pos.2, pos.3), steps + *steps_to_pos, new_keys));
                    *did_visit = steps + *steps_to_pos;
                }
            } else {
                visited.insert((new_keys, (pos.0, *new_pos, pos.2, pos.3)), steps + *steps_to_pos);
                todo.push_back(((pos.0, *new_pos, pos.2, pos.3), steps + *steps_to_pos, new_keys));
            }
        }
        for (key, (new_pos, steps_to_pos)) in &moves2 {
            let mut new_keys = keys;
            new_keys.insert(*key);
            if let Some(did_visit) = visited.get_mut(&(new_keys, (pos.0, pos.1, *new_pos, pos.3))) {
                if steps + *steps_to_pos < *did_visit {
                    todo.push_back(((pos.0, pos.1, *new_pos, pos.3), steps + *steps_to_pos, new_keys));
                    *did_visit = steps + *steps_to_pos;
                }
            } else {
                visited.insert((new_keys, (pos.0, pos.1, *new_pos, pos.3)), steps + *steps_to_pos);
                todo.push_back(((pos.0, pos.1, *new_pos, pos.3), steps + *steps_to_pos, new_keys));
            }
        }
        for (key, (new_pos, steps_to_pos)) in &moves3 {
            let mut new_keys = keys;
            new_keys.insert(*key);
            if let Some(did_visit) = visited.get_mut(&(new_keys, (pos.0, pos.1, pos.2, *new_pos))) {
                if steps + *steps_to_pos < *did_visit {
                    todo.push_back(((pos.0, pos.1, pos.2, *new_pos), steps + *steps_to_pos, new_keys));
                    *did_visit = steps + *steps_to_pos;
                }
            } else {
                visited.insert((new_keys, (pos.0, pos.1, pos.2, *new_pos)), steps + *steps_to_pos);
                todo.push_back(((pos.0, pos.1, pos.2, *new_pos), steps + *steps_to_pos, new_keys));
            }
        }
    }
    result
}


fn parse_map(input: &str) -> Vec<Vec<MapTile>> {
    input
        .lines()
        .map(|line|
            line.trim().chars().map(MapTile::from).collect()
        ).collect()
}

fn find_start_pos(input: &str) -> (usize, usize) {
    input
        .lines()
        .find_position(|line| line.contains('@'))
        .map(|(y, line)|
            (line.trim().chars().find_position(|c| *c == '@').unwrap().0, y)
        ).unwrap()
}

#[cfg(test)]
mod test {
    use itertools::repeat_n;
    use super::*;

    #[test]
    fn test_bitmap_key() {
        let mut key = BitmapKey::new();
        assert!(!key.contains('a'));
        assert!(!key.contains('z'));
        assert!(key.insert('a'));
        assert!(!key.insert('a'));
        assert!(key.insert('z'));
        assert!(key.contains('a'));
        assert!(key.contains('z'));
    }

    #[test]
    fn test_parse_map() {
        let map =
            "#########
             #b.A.@.a#
             #########";
        assert_eq!(parse_map(map), vec![
            repeat_n(MapTile::Wall, 9).collect(),
            vec![MapTile::Wall, MapTile::Key('b'), MapTile::Empty, MapTile::Door('A'), MapTile::Empty, MapTile::Empty, MapTile::Empty, MapTile::Key('a'), MapTile::Wall],
            repeat_n(MapTile::Wall, 9).collect()
        ]);
        assert_eq!(find_start_pos(map), (5, 1));
    }

    #[test]
    fn test_find_possible_moves() {
        let map =
            "########################
             #f.D.E.e.C.b.A.@.a.B.c.#
             ######################.#
             #d.....................#
             ########################";
        let moves = find_possible_moves(&parse_map(map), find_start_pos(map), BitmapKey::new());
        assert_eq!(moves.len(), 1);
        assert_eq!(*moves.get(&'a').unwrap(), ((17, 1), 2));
        let mut keys = BitmapKey::new();
        keys.insert('a');
        keys.insert('b');
        keys.insert('c');
        let moves = find_possible_moves(&parse_map(map), (21, 1), keys);
        assert_eq!(moves.len(), 2);
        assert_eq!(*moves.get(&'d').unwrap(), ((1, 3), 24));
        assert_eq!(*moves.get(&'e').unwrap(), ((7, 1), 14));
    }

    #[test]
    fn test_find_all_keys() {
        let map =
            "########################
             #f.D.E.e.C.b.A.@.a.B.c.#
             ######################.#
             #d.....................#
             ########################";
        assert_eq!(find_all_keys(&parse_map(map), find_start_pos(map)), 86);
        let map =
            "#################
             #i.G..c...e..H.p#
             ########.########
             #j.A..b...f..D.o#
             ########@########
             #k.E..a...g..B.n#
             ########.########
             #l.F..d...h..C.m#
             #################";
        assert_eq!(find_all_keys(&parse_map(map), find_start_pos(map)), 136);
    }
}