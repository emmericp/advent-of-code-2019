pub mod intcode;

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 { return b.abs(); }
    if b == 0 { return a.abs(); }
    loop {
        let h = a % b;
        a = b;
        b = h;
        if b == 0 { break; };
    }
    a.abs()
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(6, 4), 2);
    assert_eq!(gcd(-10, -5), 5);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(3, 2), 6);
    assert_eq!(lcm(3528, 3780), 52920);
}

