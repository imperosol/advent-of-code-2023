use itertools::Itertools;
use std::cmp::{max, min};
use std::str::FromStr;
advent_of_code::solution!(11);

#[derive(Debug, PartialEq, Clone)]
struct Star {
    row: usize,
    col: usize,
}

impl Star {
    fn distance_from(&self, other: &Star) -> usize {
        let nb_rows = max(self.row, other.row) - min(self.row, other.row);
        let nb_cols = max(self.col, other.col) - min(self.col, other.col);
        nb_rows + nb_cols
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Galaxy(Vec<Star>);

impl FromStr for Galaxy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stars = s
            .lines()
            .enumerate()
            .flat_map(|(row_ind, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'#')
                    .map(move |(col_ind, _)| Star {
                        row: row_ind,
                        col: col_ind,
                    })
            })
            .collect();
        Ok(Self(stars))
    }
}

impl Galaxy {
    /// Expand the galaxy according to the expansion factor
    /// If expansion = 2, each empty row/column will be twice as large.
    /// If expansion = 100, each empty row/col will 100x larger
    fn expand(&mut self, expansion: usize) {
        // expand rows
        self.0.sort_by_key(|star| star.row);
        let mut empty_rows = 0;
        let mut current_row = self.0.first().unwrap().row;
        for star in self.0.iter_mut() {
            if star.row > current_row {
                empty_rows += star.row - current_row - 1;
                current_row = star.row;
            }
            star.row += empty_rows * (expansion - 1);
        }

        // expand cols
        self.0.sort_by_key(|star| star.col);
        let mut empty_cols = 0;
        let mut current_col = self.0.first().unwrap().col;
        for star in self.0.iter_mut() {
            if star.col > current_col {
                empty_cols += star.col - current_col - 1;
                current_col = star.col;
            }
            star.col += empty_cols * (expansion - 1);
        }
    }

    fn distance_sum(&self) -> usize {
        self.0
            .iter()
            .tuple_combinations()
            .map(|(i, j)| i.distance_from(j))
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut galaxy = input.parse::<Galaxy>().unwrap();
    galaxy.expand(2);
    Some(galaxy.distance_sum() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut galaxy = input.parse::<Galaxy>().unwrap();
    galaxy.expand(1_000_000);
    Some(galaxy.distance_sum() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
