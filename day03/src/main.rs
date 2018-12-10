use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::Read;

fn main() {
    if let Err(e) = run() {
        eprintln!("FATAL ERROR: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let directions: Vec<_> = input.chars().filter_map(Direction::from_char).collect();

    let distinct_houses = count_distinct(directions.as_slice());
    println!("Santa visited {} distinct houses", distinct_houses);

    let robot_distinct_houses = count_robot_distinct(directions.as_slice());
    println!(
        "Santa and Robot visited {} distinct houses",
        robot_distinct_houses
    );

    Ok(())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }
}

type Position = (i32, i32);
fn step(current: Position, direction: &Direction) -> Position {
    let (x, y) = current;
    match direction {
        Direction::Up => (x + 1, y),
        Direction::Down => (x - 1, y),
        Direction::Right => (x, y + 1),
        Direction::Left => (x, y - 1),
    }
}

fn count_distinct(directions: &[Direction]) -> usize {
    let mut position = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(position);

    for direction in directions.iter() {
        position = step(position, &direction);
        visited.insert(position);
    }
    visited.len()
}

// Now Santa and a Robot santa take turns (even santa, odd robot turns)
fn count_robot_distinct(directions: &[Direction]) -> usize {
    let mut santa = (0, 0);
    let mut robot = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(santa);

    let mut santurn = false;
    for direction in directions.iter() {
        let current = if santurn { &mut santa } else { &mut robot };

        *current = step(*current, direction);
        visited.insert(*current);

        santurn = !santurn
    }
    visited.len()
}

#[test]
fn test_parse_direction_other() {
    let input = ">asdf>";
    let directions: Vec<Direction> = input
        .chars()
        .filter_map(|c| Direction::from_char(c))
        .collect();

    assert_eq!(&[Direction::Right, Direction::Right], directions.as_slice())
}

#[test]
fn test_parse_direction() {
    let input = "<^v>";
    let directions: Vec<Direction> = input
        .chars()
        .filter_map(|c| Direction::from_char(c))
        .collect();

    assert_eq!(
        &[
            Direction::Left,
            Direction::Up,
            Direction::Down,
            Direction::Right
        ],
        directions.as_slice()
    )
}

#[test]
fn test_count_visited_start() {
    let directions = &[Direction::Right];
    assert_eq!(2, count_distinct(directions));
}

#[test]
fn test_count_visited_all() {
    let directions = &[
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    assert_eq!(4, count_distinct(directions));
}

#[test]
fn test_count_visited_same() {
    let directions = &[
        Direction::Up,
        Direction::Down,
        Direction::Up,
        Direction::Down,
        Direction::Up,
        Direction::Down,
        Direction::Up,
        Direction::Down,
        Direction::Up,
        Direction::Down,
    ];
    assert_eq!(2, count_distinct(directions));
}

#[test]
fn test_robot_count_visited_start() {
    let directions = &[Direction::Up, Direction::Down];
    assert_eq!(3, count_robot_distinct(directions));
}

#[test]
fn test_robot_count_visited_all() {
    let directions = &[
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    assert_eq!(3, count_robot_distinct(directions));
}

#[test]
fn test_robot_count_visited_same() {
    let directions = &[
        Direction::Up,
        Direction::Down,
        Direction::Up,
        Direction::Down,
        Direction::Up,
        Direction::Down,
        Direction::Up,
        Direction::Down,
        Direction::Up,
        Direction::Down,
    ];
    assert_eq!(11, count_robot_distinct(directions));
}
