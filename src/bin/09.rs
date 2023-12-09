use itertools::Itertools;
use std::str::FromStr;
advent_of_code::solution!(9);

#[derive(Debug)]
struct Line(Vec<i32>);

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s.split(' ').map(|i| i.parse().unwrap()).collect();
        Ok(Line(res))
    }
}

impl Line {
    #[inline(always)]
    fn child(&self) -> Self {
        let res = self.0.iter().tuple_windows().map(|(i, j)| j - i).collect();
        Line(res)
    }

    fn next_element(&self) -> i32 {
        // It could be faster using an iterative algorithm,
        // but man, the recursive solution is just so much cleaner
        match self.0.iter().all_equal() {
            true => self.0[0], // f(n) = f(0) for all n
            false => self.0.last().unwrap() + self.child().next_element(),
        }
    }

    fn previous_element(&self) -> i32 {
        match self.0.iter().all_equal() {
            true => self.0[0],
            false => self.0[0] - self.child().previous_element(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|line| line.parse::<Line>().unwrap().next_element())
        .sum::<i32>();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|line| line.parse::<Line>().unwrap().previous_element())
        .sum::<i32>();
    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
