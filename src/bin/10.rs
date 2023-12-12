use crate::Pipe::{Empty, Horizontal, NorthEast, NorthWest, SouthEast, SouthWest, Start, Vertical};
use std::fmt::Debug;
use std::str::FromStr;
advent_of_code::solution!(10);

#[derive(Debug, PartialEq)]
enum Pipe {
    Horizontal,
    Vertical,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Start,
    Empty,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Coords {
    row: usize,
    col: usize,
}

impl From<(usize, usize)> for Coords {
    fn from(value: (usize, usize)) -> Self {
        Self {
            row: value.0,
            col: value.1,
        }
    }
}

#[derive(Debug)]
struct NoNextPipeError(Coords);

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '-' => Horizontal,
            '|' => Vertical,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            'S' => Start,
            _ => Empty,
        }
    }
}

struct MainLoop<'a> {
    grid: &'a Grid,
    current_pos: Coords,
    next_pos: Coords,
}

impl<'a> From<&'a Grid> for MainLoop<'a> {
    fn from(value: &'a Grid) -> Self {
        let start = value.first_cell();
        let next_cell = if start.row > 0
            && [Vertical, SouthEast, SouthWest].contains(&value.0[start.row - 1][start.col])
        {
            Coords::from((start.row - 1, start.col))
        } else if start.col > 0
            && [Horizontal, SouthWest, NorthWest].contains(&value.0[start.row][start.col - 1])
        {
            Coords::from((start.row, start.col - 1))
        } else if start.row < value.0.len()
            && [Vertical, NorthWest, NorthEast].contains(&value.0[start.row + 1][start.col])
        {
            Coords::from((start.row + 1, start.col))
        } else if start.col < value.0[0].len()
            && [Horizontal, SouthEast, NorthEast].contains(&value.0[start.row][start.col + 1])
        {
            Coords::from((start.row, start.col + 1))
        } else {
            // If here, there is no pipe that is connected to the start,
            // which is stated as impossible by the problem's description
            unreachable!()
        };
        Self {
            grid: value,
            current_pos: start,
            next_pos: next_cell,
        }
    }
}

impl Iterator for MainLoop<'_> {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        let next_pipe = self.grid.next_pipe(&self.next_pos, &self.current_pos);
        match next_pipe {
            Ok(pipe) => {
                self.current_pos = self.next_pos.clone();
                self.next_pos = pipe.clone();
                Some(pipe)
            }
            Err(_) => None,
        }
    }
}

struct Grid(Vec<Vec<Pipe>>);

impl Grid {
    fn first_cell(&self) -> Coords {
        self.0
            .iter()
            .enumerate()
            .find_map(|(row_ind, row)| {
                row.iter()
                    .position(|col| *col == Start)
                    .map(|col_ind| (row_ind, col_ind).into())
            })
            .unwrap()
    }

    #[inline(always)]
    fn get(&self, coords: &Coords) -> &Pipe {
        &self.0[coords.row][coords.col]
    }

    fn next_pipe(&self, pos: &Coords, previous_pos: &Coords) -> Result<Coords, NoNextPipeError> {
        let pair = match self.get(pos) {
            Horizontal => Ok((
                Coords::from((pos.row, pos.col - 1)),
                Coords::from((pos.row, pos.col + 1)),
            )),
            Vertical => Ok((
                Coords::from((pos.row - 1, pos.col)),
                Coords::from((pos.row + 1, pos.col)),
            )),
            NorthWest => Ok((
                Coords::from((pos.row - 1, pos.col)),
                Coords::from((pos.row, pos.col - 1)),
            )),
            NorthEast => Ok((
                Coords::from((pos.row - 1, pos.col)),
                Coords::from((pos.row, pos.col + 1)),
            )),
            SouthWest => Ok((
                Coords::from((pos.row + 1, pos.col)),
                Coords::from((pos.row, pos.col - 1)),
            )),
            SouthEast => Ok((
                Coords::from((pos.row + 1, pos.col)),
                Coords::from((pos.row, pos.col + 1)),
            )),
            _ => Err(NoNextPipeError(pos.clone())),
        };
        let (a, b) = pair?;
        // Either a or b is the previous pos.
        // Thus the next pipe has the coords of the one that isn't previous_pos
        if a == *previous_pos {
            Ok(b)
        } else {
            Ok(a)
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|l| l.chars().map(Pipe::from).collect())
                .collect(),
        ))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.parse::<Grid>().unwrap();
    // The +1 is to include the starting pipe twice in the loop
    let loop_len = MainLoop::from(&grid).count() + 1;

    Some(loop_len as u32 / 2)
}

pub fn part_two(_input: &str) -> Option<u32> {
    // I'm late and I heard this part is hard to solve
    // I will catch up with the AoC schedule, then I will go back to this
    None
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
        assert_eq!(result, None);
    }
}
