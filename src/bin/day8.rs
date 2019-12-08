use std::fs;
use itertools::Itertools;
use std::fs::File;
use image::gif::{Encoder, Frame};

fn main() {
    let input = fs::read_to_string("./input/day8.txt").unwrap();
    dbg!(part1(&input.trim(), 25 * 6));
    part2(&input.trim(), 25, 6);
}

fn part1(data: &str, frame_size: usize) -> u64 {
    let min_frame = data.chars().chunks(frame_size).into_iter().map(|frame| {
        let mut count0 = 0;
        let mut count1 = 0;
        let mut count2 = 0;
        for char in frame {
            match char {
                '0' => count0 += 1,
                '1' => count1 += 1,
                '2' => count2 += 1,
                _ => panic!()
            }
        }
        (count0, count1, count2)
    }).min_by_key(|(count0, _, _)| *count0).unwrap();
    min_frame.1 * min_frame.2
}

fn part2(data: &str, width: usize, height: usize) {
    let file_out = File::create("day8.gif").unwrap();
    let mut encoder = Encoder::new(file_out);
    let frames: Vec<Frame> = data.chars().chunks(width * height).into_iter().map(|frame| {
        let mut pixels: Vec<u8> = frame.flat_map(|e| match e {
            '0' => [255u8, 255u8, 255u8, 255u8].iter(),
            '1' => [0u8, 0u8, 0u8, 255u8].iter(),
            '2' => [0u8, 0u8, 0u8, 0u8].iter(),
            _ => panic!()
        }).copied().collect();
        let mut frame = Frame::from_rgba(width as u16, height as u16, &mut pixels);
        frame.delay = 25;
        frame
    }).collect();
    frames.into_iter().rev().for_each(|frame| encoder.encode(&frame).unwrap());
}

#[test]
fn test_part1() {
    assert_eq!(part1("01201122", 4), 4);
}
