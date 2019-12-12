use std::fs;
use itertools::Itertools;
use advent_of_code::gcd;

fn main() {
    let input = fs::read_to_string("./input/day10.txt").unwrap();
    let map = parse_map(&input);
    let src = dbg!(find_best_asteroid(&map));
    dbg!(visible_asteroids(&map, src));
    let laser_directions: Vec<(i32, i32)> = get_directions(&map, src)
        .into_iter()
        .filter(|e| *e != (0, 0))
        .unique()
        .sorted_by(|a, b| {
            (f64::from(a.0).atan2(f64::from(a.1))).partial_cmp(&f64::from(b.0).atan2(f64::from(b.1))).unwrap().reverse()
        })
        .collect();
    let mut asteroids_by_direction = map.into_iter().map(|e| (get_direction(e, src), e)).into_group_map();
    let mut hit_count = 1;
    let mut did_hit = true;
    while did_hit {
        did_hit = false;
        laser_directions.iter().for_each(|dir| {
            if let Some(asteroids) = asteroids_by_direction.get_mut(dir) {
                if !asteroids.is_empty() {
                    let target = asteroids.remove(0);
                    if hit_count == 200 {
                        println!("Hit {}, at position ({}, {})", hit_count, target.0, target.1);
                    }
                    hit_count += 1;
                    did_hit = true;
                }
            }
        });
    }
}

fn get_direction(dst: (i32, i32), src: (i32, i32)) -> (i32, i32) {
    let x = dst.0 - src.0;
    let y = dst.1 - src.1;
    let gcd = gcd(x.into(), y.into()) as i32;
    if gcd == 0 {
        (0, 0) // direction to itself
    } else {
        (x / gcd, y / gcd)
    }
}

fn get_directions(coords: &[(i32, i32)], src: (i32, i32)) -> Vec<(i32, i32)> {
    coords.iter().map(|c| get_direction(*c, src)).collect()
}

fn visible_asteroids(coords: &[(i32, i32)], src: (i32, i32)) -> usize {
    get_directions(coords, src).iter().unique().count() - 1 // don't count self
}

fn find_best_asteroid(coords: &[(i32, i32)]) -> (i32, i32) {
    *coords.iter().max_by_key(|src| {
        visible_asteroids(coords, **src)
    }).unwrap()
}

fn parse_map(map: &str) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let mut line_counter = 0;
    map.trim().lines().for_each(|line| {
        let mut char_counter = 0;
        line.chars().for_each(|c| {
            if c == '#' {
                result.push((char_counter, line_counter));
            }
            char_counter += 1;
        });
        line_counter += 1;
    });
    result
}

#[test]
fn test_get_directions() {
    let map_5x5 = parse_map("
#####
#####
#####
#####
#####
    ");
    let directions = get_directions(&map_5x5, (2, 2));
    assert_eq!(directions.len(), 25);
    assert_eq!(directions[0], (-1, -1));
    assert_eq!(directions[4], (1, -1));
    assert_eq!(directions[20], (-1, 1));
    assert_eq!(directions[24], (1, 1));
    assert_eq!(directions[5], (-2, -1));
    assert_eq!(directions[6], (-1, -1));
}

#[test]
fn test_example1() {
    let map = parse_map("
.#..#
.....
#####
....#
...##
    ");
    assert_eq!(visible_asteroids(&map, (3, 4)), 8);
    assert_eq!(visible_asteroids(&map, (4, 4)), 7);
    assert_eq!(visible_asteroids(&map, (0, 1)), 7);
    assert_eq!(find_best_asteroid(&map), (3, 4));
}

