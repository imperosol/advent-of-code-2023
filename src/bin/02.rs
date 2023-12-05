use anyhow::Error;
use std::cmp::max;
use std::str::FromStr;
advent_of_code::solution!(2);

/// Convert a line into an iterator of tuples
/// of the form `[(1, "red"), (6, "blue")]`
fn split_line(s: &str) -> impl Iterator<Item = (u8, &str)> {
    s.split(": ")
        .nth(1)
        .unwrap() // remove "Game <n>: "
        .split(&[',', ';'][..])
        .map(|i| i.trim())
        .map(|i| {
            let mut split = i.split(' ');
            let ball_amount = split.next().unwrap().parse::<u8>().unwrap();
            (ball_amount, split.next().unwrap())
        })
}

fn line_valid(line: &str) -> bool {
    split_line(line).all(|(n, colour)| match colour {
        "red" => n <= 12,
        "green" => n <= 13,
        "blue" => n <= 14,
        _ => false,
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .enumerate()
        .filter_map(|(id, line)| {
            if line_valid(line) {
                Some(id as u32 + 1)
            } else {
                None
            }
        })
        .sum();
    Some(res)
}

#[derive(Debug, Default)]
struct MinRequired {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for MinRequired {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = MinRequired::default();
        split_line(s).for_each(|(n, colour)| match colour {
            "red" => res.red = max(res.red, n),
            "green" => res.green = max(res.green, n),
            "blue" => res.blue = max(res.blue, n),
            _ => {}
        });
        Ok(res)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|line| MinRequired::from_str(line).unwrap())
        .map(|MinRequired { red, green, blue }| red as u32 * green as u32 * blue as u32)
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
