use nom::{character::complete::one_of, combinator::map_res, multi::many1, IResult};
use std::fs;

#[derive(Debug)]
enum FloorChange {
    Up,
    Down,
}

fn from_char(c: char) -> Result<FloorChange, ()> {
    match c {
        '(' => Ok(FloorChange::Up),
        _ => Ok(FloorChange::Down),
    }
}

fn parse_up(input: &str) -> IResult<&str, FloorChange> {
    map_res(one_of("()"), |c| match c {
        '(' => Ok(FloorChange::Up),
        _ => Ok(FloorChange::Down),
    })(input)
}

fn parse_floor_change(input: &str) -> Vec<FloorChange> {
    many1(parse_up)(input).unwrap().1
}

fn sum_instructions(is: &[FloorChange]) -> i32 {
    is.iter()
        .map(|g| match g {
            FloorChange::Up => 1,
            FloorChange::Down => -1,
        })
        .sum()
}

fn find_negative_step(is: &[FloorChange]) -> u64 {
    is.iter()
        .map(|g| match g {
            FloorChange::Up => 1,
            FloorChange::Down => -1,
        })
        .fold((0, 0), |acc, step| {
            if acc.0 < 0 {
                acc
            } else {
                (acc.0 + step, acc.1 + 1)
            }
        })
        .1
}

// 280
pub fn part1() {
    let input = fs::read_to_string("data/2015/day1.input").unwrap();
    let instructions = parse_floor_change(&input);
    println!("Solution 1: {}", sum_instructions(&instructions))
}

// 1797
pub fn part2() {
    let input = fs::read_to_string("data/2015/day1.input").unwrap();
    let instructions = parse_floor_change(&input);
    println!("Solution 2: {}", find_negative_step(&instructions))
}

#[test]
fn part1_example() {
    let instructions = parse_floor_change(")())())");
    assert_eq!(sum_instructions(&instructions), -3)
}

#[test]
fn part2_example() {
    let instructions = parse_floor_change("()())");
    assert_eq!(find_negative_step(&instructions), 5)
}
