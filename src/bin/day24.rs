use std::fs;
use std::collections::{HashSet, HashMap};
use itertools::{Itertools, MinMaxResult};

fn main() {
    let input = fs::read_to_string("./input/day24.txt").unwrap();
    dbg!(find_cycle(parse(&input)));
    let input = parse(&input);
    let mut state = HashMap::new();
    state.insert(0, input);
    for _ in 0..200 {
        state = step_part2(state);
    }
    dbg!(state.values().map(|state| {
        let mut count = 0;
        for i in 0..25 {
            count += (*state & (1 << i) != 0) as i32;
        }
        count
    }).sum::<i32>());
}

fn parse(input: &str) -> i32 {
    let mut result = 0;
    for (i, _) in input.chars().filter(|c| !c.is_whitespace()).enumerate().filter(|(_, c)| *c == '#') {
        result |= 1 << i;
    }
    result as i32
}

#[allow(clippy::identity_op)]
fn step_part2(states: HashMap<i32, i32>) -> HashMap<i32, i32> {
    let mut result = HashMap::new();
    let mut states_with_extra_layers = states.clone();
    match states.keys().minmax() {
        MinMaxResult::NoElements => panic!("empty state"),
        MinMaxResult::OneElement(layer) => {
            states_with_extra_layers.insert(layer - 1, 0);
            states_with_extra_layers.insert(layer + 1, 0);
        }
        MinMaxResult::MinMax(min, max) => {
            states_with_extra_layers.insert(min - 1, 0);
            states_with_extra_layers.insert(max + 1, 0);
        }
    }
    for (layer, state) in states_with_extra_layers.iter() {
        let mut new_layer: i32 = 0;
        let upper_layer = *states.get(&(layer - 1)).unwrap_or(&0);
        let lower_layer = *states.get(&(layer + 1)).unwrap_or(&0);
        let upper_layer_top = (upper_layer & (1 << 7) != 0) as i32;
        let upper_layer_left = (upper_layer & (1 << 11) != 0) as i32;
        let upper_layer_right = (upper_layer & (1 << 13) != 0) as i32;
        let upper_layer_bottom = (upper_layer & (1 << 17) != 0) as i32;
        let lower_layer_top = (lower_layer & (1 << 0) != 0) as i32
            + (lower_layer & (1 << 1) != 0) as i32
            + (lower_layer & (1 << 2) != 0) as i32
            + (lower_layer & (1 << 3) != 0) as i32
            + (lower_layer & (1 << 4) != 0) as i32;
        let lower_layer_left = (lower_layer & (1 << 0)) as i32
            + (lower_layer & (1 << 5) != 0) as i32
            + (lower_layer & (1 << 10) != 0) as i32
            + (lower_layer & (1 << 15) != 0) as i32
            + (lower_layer & (1 << 20) != 0) as i32;
        let lower_layer_right = (lower_layer & (1 << 4) != 0) as i32
            + (lower_layer & (1 << 9) != 0) as i32
            + (lower_layer & (1 << 14) != 0) as i32
            + (lower_layer & (1 << 19) != 0) as i32
            + (lower_layer & (1 << 24) != 0) as i32;
        let lower_layer_bottom = (lower_layer & (1 << 20) != 0) as i32
            + (lower_layer & (1 << 21) != 0) as i32
            + (lower_layer & (1 << 22) != 0) as i32
            + (lower_layer & (1 << 23) != 0) as i32
            + (lower_layer & (1 << 24) != 0) as i32;
        let mut fields = Vec::new();
        for i in 0..25 {
            fields.push((state & (1 << i) != 0) as i32);
        }
        for i in 0..25 {
            let cnt = match i {
                0 => upper_layer_left + upper_layer_top + fields[1] + fields[5],
                1 => upper_layer_top + fields[0] + fields[2] + fields[6],
                2 => upper_layer_top + fields[1] + fields[3] + fields[7],
                3 => upper_layer_top + fields[2] + fields[4] + fields[8],
                4 => upper_layer_top + upper_layer_right + fields[3] + fields[9],
                5 => upper_layer_left + fields[0] + fields[6] + fields[10],
                6 => fields[5] + fields[1] + fields[7] + fields[11],
                7 => fields[2] + fields[6] + fields[8] + lower_layer_top,
                8 => fields[7] + fields[3] + fields[9] + fields[13],
                9 => upper_layer_right + fields[4] + fields[8] + fields[14],
                10 => upper_layer_left + fields[5] + fields[11] + fields[15],
                11 => fields[6] + fields[10] + fields[16] + lower_layer_left,
                13 => fields[8] + fields[14] + fields[18] + lower_layer_right,
                14 => upper_layer_right + fields[9] + fields[13] + fields[19],
                15 => upper_layer_left + fields[10] + fields[16] + fields[20],
                16 => fields[15] + fields[11] + fields[17] + fields[21],
                17 => fields[22] + fields[16] + fields[18] + lower_layer_bottom,
                18 => fields[17] + fields[13] + fields[19] + fields[23],
                19 => upper_layer_right + fields[14] + fields[18] + fields[24],
                20 => upper_layer_left + upper_layer_bottom + fields[21] + fields[15],
                21 => upper_layer_bottom + fields[20] + fields[22] + fields[16],
                22 => upper_layer_bottom + fields[21] + fields[23] + fields[17],
                23 => upper_layer_bottom + fields[22] + fields[24] + fields[18],
                24 => upper_layer_bottom + upper_layer_right + fields[23] + fields[19],
                _ => 0,
            };
            if i != 12 && (fields[i] == 0 && cnt == 2 || cnt == 1) {
                new_layer |= 1 << i as i32;
            }
        }
        result.insert(*layer, new_layer);
    }
    result
}

fn step(state: i32) -> i32 {
    let mut result = 0;
    for i in 0..25 {
        let cnt = (i % 5 != 0 && state & (1 << (i - 1)) != 0) as i32
            + (i % 5 != 4 && state & (1 << (i + 1)) != 0) as i32
            + (i >= 5 && state & (1 << (i - 5)) != 0) as i32
            + (i < 20 && state & (1 << (i + 5)) != 0) as i32;
        if state & (1 << i) == 0 && cnt == 2 || cnt == 1 {
            result |= 1 << i;
        }
    }
    result
}

fn find_cycle(mut state: i32) -> i32 {
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(state) {
            return state;
        }
        state = step(state);
    }
}

#[test]
fn test_parse() {
    let input = "
        .....
        .....
        .....
        #....
        .#...
    ";
    assert_eq!(parse(input), 2_129_920);
}


#[test]
fn test_step() {
    let input = "
        ....#
        #..#.
        #..##
        ..#..
        #....
    ";
    let result = "
        #..#.
        ####.
        ###.#
        ##.##
        .##..
    ";
    assert_eq!(step(parse(input)), parse(result));
}

#[test]
fn test_find_cycle() {
    let input = "
        ....#
        #..#.
        #..##
        ..#..
        #....
    ";
    assert_eq!(find_cycle(parse(input)), 2_129_920);
}

