use std::fs;
use advent_of_code::intcode::IntCodeCpu;
use std::collections::BTreeMap;
use std::fs::File;
use image::gif::{Encoder, Frame};

fn main() {
    let input = fs::read_to_string("./input/day11.txt").unwrap();
    let mut cpu = IntCodeCpu::from_code(&input);
    let mut map: Map = BTreeMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::Up;
    let mut visited_fields_count = 0;
    set_color(&mut map, 0, 0, Color::White); // part 2
    loop {
        let color = get_color(&map, x, y);
        cpu.input.push_back(color.get_value());
        if let Some(new_color) = cpu.run_until_out() {
            if let Some(turn_direction) = cpu.run_until_out() {
                if set_color(&mut map, x, y, Color::from(new_color)).is_none() {
                    visited_fields_count += 1;
                }
                if turn_direction != 0 && turn_direction != 1 {
                    panic!("bad turn direction {}", turn_direction);
                }
                direction = direction.turn(&Turn::from(turn_direction));
                match direction {
                    Direction::Up => y -= 1,
                    Direction::Right => x += 1,
                    Direction::Down => y += 1,
                    Direction::Left => x -= 1,
                };
            } else {
                break;
            }
        } else {
            break;
        }
    }
    dbg!(visited_fields_count);
    let mut x_iter = map.iter();
    x_iter.next();
    let min_x = *map.iter().next().unwrap().0;
    let max_x = *map.iter().next_back().unwrap().0;
    let min_y = map.iter()
        .min_by_key(|e| *e.1.iter().next().unwrap().0).unwrap().0;
    let max_y = map.iter()
        .max_by_key(|e| *e.1.iter().next_back().unwrap().0).unwrap().0;
    let mut pixels: Vec<u8> = Vec::new();
    pixels.resize(((max_x - min_x) * (max_y - min_y) * 3) as usize, 0);
    for (x, ys) in &map {
        for (y, color) in ys {
            let x_adjusted = x - min_x;
            let y_adjusted = y - min_y;
            pixels[((x_adjusted + y_adjusted * (max_x - min_x)) * 3) as usize] = color.get_grayscale();
            pixels[((x_adjusted + y_adjusted * (max_x - min_x)) * 3) as usize + 1] = color.get_grayscale();
            pixels[((x_adjusted + y_adjusted * (max_x - min_x)) * 3) as usize + 2] = color.get_grayscale();
        }
    }
    let file_out = File::create("day11.gif").unwrap();
    let mut encoder = Encoder::new(file_out);
    let frame = Frame::from_rgb((max_x - min_x) as u16, (max_y - min_y) as u16, &pixels);
    encoder.encode(&frame).ok().unwrap();
}

type Map = BTreeMap<i32, BTreeMap<i32, Color>>;

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
}

impl Color {
    fn get_value(self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
    fn get_grayscale(self) -> u8 {
        match self {
            Color::Black => 0,
            Color::White => 255,
        }
    }
}

impl From<i64> for Color {
    fn from(val: i64) -> Self {
        match val {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("bad color {}", val),
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self, turn: &Turn) -> Self {
        match self {
            Direction::Up => match turn {
                Turn::Left => Direction::Left,
                Turn::Right => Direction::Right,
            },
            Direction::Right => match turn {
                Turn::Left => Direction::Up,
                Turn::Right => Direction::Down,
            },
            Direction::Down => match turn {
                Turn::Left => Direction::Right,
                Turn::Right => Direction::Left,
            },
            Direction::Left => match turn {
                Turn::Left => Direction::Down,
                Turn::Right => Direction::Up,
            },
        }
    }
}

enum Turn {
    Left,
    Right,
}

impl From<i64> for Turn {
    fn from(val: i64) -> Self {
        match val {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("bad turn direction {}", val),
        }
    }
}


fn get_color(map: &Map, x: i32, y: i32) -> &Color {
    map.get(&x).map_or(&Color::Black, |e| e.get(&y).unwrap_or(&Color::Black))
}

fn set_color(map: &mut Map, x: i32, y: i32, color: Color) -> Option<Color> {
    map.entry(x).or_insert_with(BTreeMap::new).insert(y, color)
}
