use crate::Direction::{Left, Right};
use itertools::Itertools;
use num::Integer;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::collections::HashMap;
advent_of_code::solution!(8);

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'R' => Right,
            'L' => Left,
            _ => panic!("This shouldn't happen"),
        }
    }
}

/// Convert the textual input of AoC into a tuple of two elements
/// 1. an infinite iterator over directions to take
/// 2. a hashmap that associates each key with two ones that can follow
/// 3. The first key to take
fn parse_input(
    input: &str,
) -> (
    impl Iterator<Item = Direction> + '_ + Clone,
    HashMap<&str, [&str; 2]>,
) {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().map(Direction::from).cycle();
    lines.next(); // remove the empty line
    let map = lines
        .map(|line| (&line[0..3], [&line[7..10], &line[12..15]]))
        .collect();
    (directions, map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut directions, map) = parse_input(input);
    let mut nb_steps = 0;
    let mut key = "AAA";
    while key != "ZZZ" {
        nb_steps += 1;
        let current_line = map.get(key).unwrap();
        key = match directions.next().unwrap() {
            Left => current_line[0],
            Right => current_line[1],
        };
    }
    Some(nb_steps)
}

fn part_two_nb_steps(
    start_key: &str,
    directions: impl Iterator<Item = Direction>,
    map: &HashMap<&str, [&str; 2]>,
) -> u32 {
    let mut key = start_key;
    directions
        .map(|direction| {
            let current_line = map.get(key).unwrap();
            key = match direction {
                Left => current_line[0],
                Right => current_line[1],
            };
            key
        })
        .take_while(|k| !k.ends_with('Z'))
        .count() as u32
        + 1
}

pub fn part_two(input: &str) -> Option<u64> {
    // The answer is the common lowest multiple of the amount
    // of steps to encounter a Z-terminated key for each path.
    // I don't really understand why, but that's an interesting property
    let (directions, map) = parse_input(input);
    let keys = map.keys().filter(|key| key.ends_with('A')).collect_vec();
    let res = keys
        .par_iter() // Using a parallel iterator is absolutely not necessary, but it's faster
        .map(|key| part_two_nb_steps(key, directions.clone(), &map))
        .map(|i| i as u64)
        .collect::<Vec<_>>()
        .iter()
        .fold(1u64, |acc, n| acc.lcm(n));
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "\
            LLR\n\
            \n\
            AAA = (BBB, BBB)\n\
            BBB = (AAA, ZZZ)\n\
            ZZZ = (ZZZ, ZZZ)\n\
        ";
        let result = part_one(input);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
