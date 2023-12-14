use itertools::Itertools;
use std::str::FromStr;
advent_of_code::solution!(13);

#[derive(Clone, Debug)]
struct Grid(Vec<u32>);

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|l| {
                let mut mask = 0u32;
                l.chars().for_each(|c| {
                    mask <<= 1;
                    if c == '#' {
                        mask |= 1
                    };
                });
                mask
            })
            .collect();
        Ok(Grid(grid))
    }
}

impl Grid {
    fn parallel_axis(&self) -> Option<usize> {
        (1..self.0.len()).find(|&ind| {
            self.0[..ind]
                .iter()
                .rev()
                .zip(self.0.iter().skip(ind))
                .all(|(a, b)| a == b)
        })
    }

    fn to_transposed(&self) -> Self {
        let len = 32 - self.0.iter().map(|l| l.leading_zeros()).min().unwrap();
        let res = (0..len)
            .rev()
            .map(|row_idx| {
                let mut res = 0u32;
                let mask = 0b1 << row_idx;
                self.0.iter().for_each(|col| {
                    res <<= 1;
                    res |= (col & mask) >> row_idx;
                });
                res
            })
            .collect_vec();
        Grid(res)
    }

    fn res(&self) -> u32 {
        if let Some(res) = self.parallel_axis() {
            res as u32 * 100
        } else {
            self.to_transposed().parallel_axis().unwrap_or(0) as u32
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .split("\n\n")
        .map(|group| group.parse::<Grid>().unwrap().res())
        .sum();
    Some(res)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
