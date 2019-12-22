#![allow(clippy::unreadable_literal)]
use std::fs;
use itertools::Itertools;
use modinverse::modinverse;
use mod_exp::mod_exp;

fn main() {
    let input = fs::read_to_string("./input/day22.txt").unwrap();
    // part 1
    let mut cards = (0..10007).collect::<Vec<i32>>();
    for line in input.lines() {
        ShuffleTechnique::from(line).apply(&mut cards);
    }
    dbg!(cards.into_iter().find_position(|c| *c == 2019));
    // part 2, meh. needed hints on reddit :(
    let num_cards = 119315717514047;
    let target_pos = 2020;
    let mut come_from = target_pos;
    for line in input.lines().rev() {
        come_from = ShuffleTechnique::from(line).come_from(come_from, num_cards);
    }
    let mut come_from_2_shuffles = come_from;
    for line in input.lines().rev() {
        come_from_2_shuffles = ShuffleTechnique::from(line).come_from(come_from_2_shuffles, num_cards);
    }
    let count = 101741582076661;
    // f = reverse shuffle on full input
    // f(2020) = come_from, running reverse 1 time
    // f(f(2020)) = come_from_2_shuffles, running reverse 2 times
    // f(x) = a * x + b mod N; find a and b based on the two reverse runs
    let a = (come_from - come_from_2_shuffles) * modinverse(2020 - come_from as i128 + num_cards as i128, num_cards as i128).unwrap() % num_cards;
    let b = (come_from - a * target_pos) % num_cards;
    // f^n(x) = a^n * x + a^(n - 1) * b + ... + a^0 * b mod C
    //        = a^n * x + (a^(n - 1) + ... + a^0) * b mod C
    //        = a^n * x + a^(n - 1) / (a - 1) * b mod C
    dbg!((mod_exp(a, count, num_cards) * target_pos + (mod_exp(a, count, num_cards) - 1) * modinverse(a - 1, num_cards).unwrap() % num_cards * b % num_cards) % num_cards);
}

enum ShuffleTechnique {
    NewStack,
    Cut(i32),
    DealIncrement(i32),
}

impl ShuffleTechnique {
    fn apply(self, cards: &mut [i32]) {
        match self {
            ShuffleTechnique::NewStack => {
                cards.reverse()
            }
            ShuffleTechnique::Cut(n) => {
                if n >= 0 {
                    cards.rotate_left(n as usize)
                } else {
                    cards.rotate_right(-n as usize)
                }
            }
            ShuffleTechnique::DealIncrement(n) => {
                let mut new_array = Vec::new();
                new_array.resize(cards.len(), 0);
                for (i, card) in cards.iter().enumerate() {
                    new_array[i * n as usize % cards.len()] = *card;
                }
                cards.copy_from_slice(&new_array);
            }
        }
    }

    fn come_from(self, pos: i128, num_cards: i128) -> i128 {
        match self {
            ShuffleTechnique::NewStack => num_cards - pos - 1,
            ShuffleTechnique::Cut(n) => (i128::from(n) + pos + num_cards) % num_cards,
            ShuffleTechnique::DealIncrement(n) => modinverse(i128::from(n), num_cards).unwrap() * pos % num_cards,
        }
    }
}

impl From<&str> for ShuffleTechnique {
    fn from(string: &str) -> Self {
        if string == "deal into new stack" {
            ShuffleTechnique::NewStack
        } else if string.starts_with("cut") {
            ShuffleTechnique::Cut(string.split_whitespace().last().unwrap().parse().unwrap())
        } else if string.starts_with("deal with increment") {
            ShuffleTechnique::DealIncrement(string.split_whitespace().last().unwrap().parse().unwrap())
        } else {
            panic!("bad shuffle technique");
        }
    }
}

#[test]
fn test_new_stack() {
    let mut cards = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ShuffleTechnique::NewStack.apply(&mut cards);
    assert_eq!(cards, [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
}

#[test]
fn test_cut() {
    let mut cards = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ShuffleTechnique::Cut(3).apply(&mut cards);
    assert_eq!(cards, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

    let mut cards = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ShuffleTechnique::Cut(-4).apply(&mut cards);
    assert_eq!(cards, [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_deal_increment() {
    let mut cards = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ShuffleTechnique::DealIncrement(3).apply(&mut cards);
    assert_eq!(cards, [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
}
