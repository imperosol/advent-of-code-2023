use itertools::Itertools;
use std::cmp::{max, min};
advent_of_code::solution!(3);

#[derive(Debug)]
struct GridNumber {
    value: u32,
    row: usize,
    col: usize,
    len: usize,
}

impl GridNumber {
    fn is_adjacent(&self, row: usize, col: usize) -> bool {
        if max(self.row, row) - min(self.row, row) == 1 {
            // row up or row bottom
            max(self.col, 1) - 1 <= col && col <= (self.col + self.len)
        } else if self.row == row {
            // same row
            col == max(self.col, 1) - 1 || col == (self.col + self.len)
        } else {
            // not on the same row nor an adjacent one
            false
        }
    }
}

fn numbers_in_line(line: &str, row_ind: usize) -> Vec<GridNumber> {
    let mut n = 0;
    let mut nbr_ind = 0;
    let mut length = 0;
    let mut res = Vec::new();

    // Add a "." at the end of each line to make sure
    // a number can be found even if it is at the end of the line
    for (col_ind, c) in line.chars().chain(".".chars()).enumerate() {
        if c.is_ascii_digit() {
            if n == 0 {
                nbr_ind = col_ind;
            }
            length += 1;
            n = n * 10 + c.to_digit(10).unwrap();
        } else if n != 0 {
            // This cell is just after a digit
            res.push(GridNumber {
                value: n,
                row: row_ind,
                col: nbr_ind,
                len: length,
            });
            n = 0;
            length = 0;
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let symbols = input
        .lines()
        .enumerate()
        .map(|(row_ind, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_ascii_digit() && *c != '.')
                .map(move |(col_ind, _)| (row_ind, col_ind))
                .collect_vec()
        })
        .collect_vec();
    let max_len = symbols.len();
    let res = input
        .lines()
        .enumerate()
        .flat_map(|(row_ind, row)| numbers_in_line(row, row_ind))
        .filter(|nbr| {
            // get the symbols that are in the nearby row
            // checking against symbols on further rows would be a waste of time
            let row_idx = (max(nbr.row, 1) - 1)..min(nbr.row + 2, max_len);
            symbols[row_idx]
                .iter()
                .flatten()
                .any(|(row, col)| nbr.is_adjacent(*row, *col))
        })
        .map(|nbr| nbr.value)
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let gears = input.lines().enumerate().flat_map(|(row_ind, row)| {
        row.chars()
            .enumerate()
            .filter(|(_, c)| *c == '*')
            .map(move |(col_ind, _)| (row_ind, col_ind))
    });
    let numbers = input
        .lines()
        .enumerate()
        .map(|(row_ind, row)| numbers_in_line(row, row_ind))
        .collect_vec();
    let max_len = numbers.len();
    let res = gears
        .map(|(row, col)| {
            let row_idx = (max(row, 1) - 1)..min(row + 2, max_len);
            numbers[row_idx]
                .iter()
                .flatten()
                .filter(|nbr| nbr.is_adjacent(row, col))
                .map(|nbr| nbr.value)
                .collect_vec()
        })
        .filter(|vec| vec.len() > 1)
        .map(|vec| vec.iter().product::<u32>())
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
