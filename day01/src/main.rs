use std::io;
use std::io::Read;
use std::error::Error;

fn main() {
    if let Err(e) = run() {
        eprintln!("FATAL ERROR: {}", e);
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let floor: Floor = From::from(input.as_str());
    println!("Floor #{}", floor.0);

    let first_basement = first_basement(input.as_str());
    println!("First time in basement at step #{}", first_basement);
    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Floor(i32);

fn is_allowed(c: &char) -> bool {
    *c == ')' || *c == '('
}

impl From<&str> for Floor {
    fn from(s: &str) -> Self {
        let number = s.chars()
            .filter(is_allowed)
            .fold(0, _next_floor);
            Floor(number)
    }
}

fn _next_floor(current: i32, direction: char) -> i32 {
    if direction == '(' { current + 1 } else { current - 1 }
}

fn first_basement(s: &str) -> usize {
    let mut i = 0;
    let mut floor = 0;
    for direction in s.chars().filter(is_allowed) {
        i += 1;
        floor = _next_floor(floor, direction);
        if floor == -1 {
            break
        }
    }
    i
}

#[test]
fn test_floor_3() {
    assert_eq!(Floor(3), From::from("((("));
    assert_eq!(Floor(3), From::from("(()(()("));
    assert_eq!(Floor(3), From::from("))((((("));
}

#[test]
fn test_floor_neg_1() {
    assert_eq!(Floor(-1), From::from("())"));
    assert_eq!(Floor(-1), From::from("))("));
}

#[test]
fn test_floor_neg_3() {
    assert_eq!(Floor(-3), From::from(")))"));
    assert_eq!(Floor(-3), From::from(")())())"));
}

#[test]
fn test_floor_0() {
    assert_eq!(Floor(0), From::from("(())"));
    assert_eq!(Floor(0), From::from("()()"));
}

#[test]
fn test_first_basement() {
    assert_eq!(1, first_basement(")"));
    assert_eq!(5, first_basement("()())"));
}