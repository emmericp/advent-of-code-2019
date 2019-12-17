use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./input/day16.txt").unwrap();
    dbg!(&digits_to_str(&part1(&input))[0..8]);
    dbg!(&digits_to_str(&part2(&input))[0..8]);
}

fn part1(input: &str) -> Vec<i32> {
    let mut data = parse_digits(input.trim());
    for _ in 0..100 {
        data = phase(&data);
    }
    data
}

// takes a few minutes but way faster than a naive brute force solution
fn part2(input: &str) -> Vec<i32> {
    let offset: u32 = input[0..7].parse().unwrap();
    let full_input = input.trim().repeat(10_000);
    let mut data = parse_digits(&full_input[offset as usize..]);
    for _ in 0..100 {
        data = data.iter().enumerate().map(|(i, _)| {
            data.iter().skip(i).sum::<i32>().abs() % 10
        }).collect();
    }
    data
}

struct FftParams {
    digit: i32,
    state: i32,
}

fn fft_params(digit: i32) -> FftParams {
    FftParams {
        digit,
        state: 0,
    }
}

impl Iterator for FftParams {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.state += 1;
        Some([0, 1, 0, -1][(self.state / self.digit % 4) as usize])
    }
}

fn phase(digits: &[i32]) -> Vec<i32> {
    digits.iter().enumerate().map(|(i, _)| {
        digits.iter().zip(fft_params(i as i32 + 1)).map(|e| e.0 * e.1).sum::<i32>().abs() % 10
    }).collect()
}

fn parse_digits(digits: &str) -> Vec<i32> {
    digits.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
}

fn digits_to_str(digits: &[i32]) -> String {
    digits.iter().map(|d| d.to_string()).join("")
}

#[test]
fn test_fft_params() {
    assert_eq!(
        fft_params(1).take(7).collect_vec(),
        vec![1, 0, -1, 0, 1, 0, -1]
    );
    assert_eq!(
        fft_params(2).take(15).collect_vec(),
        vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1]
    );
}

#[test]
fn test_parse_digits() {
    assert_eq!(parse_digits("01234"), vec![0, 1, 2, 3, 4]);
    assert_eq!(digits_to_str(&parse_digits("01234")), "01234");
}

#[test]
fn test_phases() {
    assert_eq!(digits_to_str(&phase(&parse_digits("12345678"))), "48226158");
}

#[test]
fn test_part2() {
    // too slow in debug mode, commented out for CI
    //assert_eq!(&digits_to_str(&part2("03036732577212944063491565474664"))[0..8], "84462026");
    //assert_eq!(&digits_to_str(&part2("02935109699940807407585447034323"))[0..8], "78725270");
}
