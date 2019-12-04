use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("./input/day3.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line1 = String::new();
    let mut line2 = String::new();
    reader.read_line(&mut line1).ok();
    reader.read_line(&mut line2).ok();
    dbg!(dbg!(find_collisions(
        draw_segments(decode_input_string(&line1)),
        draw_segments(decode_input_string(&line2)),
    )).into_iter().min_by_key(|e| e.1));
}

fn decode_input_string(input: &str) -> Vec<(&str, i64)> {
    input.trim().split(",").map(|instruction| {
        (instruction.get(0..1).unwrap(), instruction.get(1..).unwrap().parse::<i64>().unwrap())
    }).collect()
}

#[derive(Debug, PartialEq)]
enum SegmentDirection {
    HORIZONTAL,
    VERTICAL,
}

#[derive(Debug, PartialEq)]
struct LineSegment {
    direction: SegmentDirection,
    inverted: bool,
    pos: i64,
    start: i64,
    end: i64,
    steps: i64,
}

fn draw_segments(instructions: Vec<(&str, i64)>) -> Vec<LineSegment> {
    let mut result = vec![];
    let mut pos_x = 0i64;
    let mut pos_y = 0i64;
    let mut steps = 0i64;
    for (dir, length) in instructions {
        steps += length;
        let seg = match dir {
            "U" => {
                pos_y += length;
                LineSegment {
                    direction: SegmentDirection::VERTICAL,
                    inverted: false,
                    pos: pos_x,
                    start: pos_y - length,
                    end: pos_y,
                    steps,
                }
            }
            "D" => {
                pos_y -= length;
                LineSegment {
                    direction: SegmentDirection::VERTICAL,
                    inverted: true,
                    pos: pos_x,
                    start: pos_y,
                    end: pos_y + length,
                    steps,
                }
            }
            "L" => {
                pos_x -= length;
                LineSegment {
                    direction: SegmentDirection::HORIZONTAL,
                    inverted: true,
                    pos: pos_y,
                    start: pos_x,
                    end: pos_x + length,
                    steps,
                }
            }
            "R" => {
                pos_x += length;
                LineSegment {
                    direction: SegmentDirection::HORIZONTAL,
                    inverted: false,
                    pos: pos_y,
                    start: pos_x - length,
                    end: pos_x,
                    steps,
                }
            }
            _ => { panic!() }
        };
        result.push(seg);
    }
    return result;
}

fn find_collisions(line1: Vec<LineSegment>, line2: Vec<LineSegment>) -> Vec<(i64, i64)> {
    let mut collisions = vec![];
    for segment1 in &line1 {
        for segment2 in &line2 {
            // ignores the edge case that we run straight into the start or end
            if segment1.direction == SegmentDirection::HORIZONTAL &&
                segment2.direction == SegmentDirection::VERTICAL {
                let col_y = segment1.pos;
                let col_x = segment2.pos;
                if col_x >= segment1.start && col_x <= segment1.end &&
                    col_y >= segment2.start && col_y <= segment2.end {
                    collisions.push((
                        col_x.abs() + col_y.abs(),
                        segment1.steps + segment2.steps
                            - if segment1.inverted { col_x - segment1.start } else { segment1.end - col_x }
                            - if segment2.inverted { col_y - segment2.start } else { segment2.end - col_y }
                    ));
                }
            } else if segment1.direction == SegmentDirection::VERTICAL &&
                segment2.direction == SegmentDirection::HORIZONTAL {
                let col_x = segment1.pos;
                let col_y = segment2.pos;
                if col_y >= segment1.start && col_y <= segment1.end &&
                    col_x >= segment2.start && col_x <= segment2.end {
                    collisions.push((
                        col_x.abs() + col_y.abs(),
                        segment1.steps + segment2.steps
                            - if segment1.inverted { col_y - segment1.start } else { segment1.end - col_y }
                            - if segment2.inverted { col_x - segment2.start } else { segment2.end - col_x }
                    ));
                }
            }
        }
    }
    if collisions[0].0 == 0 {
        collisions.remove(0);
    }
    return collisions;
}

#[test]
fn test_find_collisions() {
    assert_eq!(find_collisions(
        draw_segments(decode_input_string("U7,R6,D4,L4")),
        draw_segments(decode_input_string("R8,U5,L5,D3")),
    ), vec![(11, 30), (6, 40)]);
    assert_eq!(dbg!(find_collisions(
        draw_segments(decode_input_string("R75,D30,R83,U83,L12,D49,R71,U7,L72")),
        draw_segments(decode_input_string("U62,R66,U55,R34,D71,R55,D58,R83")),
    )).into_iter().min_by_key(|e| e.1), Some((170, 610)));
    assert_eq!(dbg!(find_collisions(
        draw_segments(decode_input_string("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")),
        draw_segments(decode_input_string("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")),
    )).into_iter().min_by_key(|e| e.1), Some((154, 410)));
}

#[test]
fn test_draw() {
    assert_eq!(draw_segments(vec![("R", 8), ("U", 5), ("L", 5), ("D", 3)]), vec![
        LineSegment {
            direction: SegmentDirection::HORIZONTAL,
            inverted: false,
            pos: 0,
            start: 0,
            end: 8,
            steps: 8,
        },
        LineSegment {
            direction: SegmentDirection::VERTICAL,
            inverted: false,
            pos: 8,
            start: 0,
            end: 5,
            steps: 13,
        },
        LineSegment {
            direction: SegmentDirection::HORIZONTAL,
            inverted: true,
            pos: 5,
            start: 3,
            end: 8,
            steps: 18,
        },
        LineSegment {
            direction: SegmentDirection::VERTICAL,
            inverted: true,
            pos: 3,
            start: 2,
            end: 5,
            steps: 21,
        },
    ]);
}
