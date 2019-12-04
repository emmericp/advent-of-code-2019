#![allow(clippy::unreadable_literal)]

fn main() {
    dbg!(count_valid_passwords(245318, 765747, is_valid_password_part1));
    dbg!(count_valid_passwords(245318, 765747, is_valid_password_part2));
}

fn pass_to_digits(password: u32) -> Vec<u32> {
    return vec![
        password / 100_000,
        password / 10_000 % 10,
        password / 1_000 % 10,
        password / 100 % 10,
        password / 10 % 10,
        password % 10,
    ];
}

fn is_valid_password_part1(password: u32) -> bool {
    let digits = pass_to_digits(password);
    let mut found_duplicate = false;
    for (digit, next) in digits.iter().zip(digits.iter().skip(1)) {
        if next < digit {
            return false;
        }
        if digit == next {
            found_duplicate = true;
        }
    }
    found_duplicate
}

fn is_valid_password_part2(password: u32) -> bool {
    let digits = pass_to_digits(password);
    for (digit, next) in digits.iter().zip(digits.iter().skip(1)) {
        if next < digit {
            return false;
        }
    }
    let mut current_run = -1;
    let mut current_run_count = 1;
    for digit in digits {
        if digit as i32 == current_run {
            current_run_count += 1;
        } else {
            current_run = digit as i32;
            if current_run_count == 2 {
                return true;
            }
            current_run_count = 1;
        }
    }
    current_run_count == 2
}

fn count_valid_passwords(start: u32, end: u32, validator: fn(u32) -> bool) -> usize {
    (start..=end).filter(|pass| validator(*pass)).count()
}

#[test]
fn test_password_checker_part1() {
    assert!(!is_valid_password_part1(123456));
    assert!(!is_valid_password_part1(223450));
    assert!(!is_valid_password_part1(123789));
    assert!(is_valid_password_part1(111111));
}

#[test]
fn test_password_checker_part2() {
    assert!(!is_valid_password_part2(123456));
    assert!(!is_valid_password_part2(223450));
    assert!(!is_valid_password_part2(123789));
    assert!(!is_valid_password_part2(111111));
    assert!(!is_valid_password_part2(123444));
    assert!(is_valid_password_part2(112222));
}
