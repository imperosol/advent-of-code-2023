advent_of_code::solution!(12);
// use crate::State::{Broken, Unknown, Valid};
// use itertools::Itertools;
// use rayon::prelude::*;
// use std::fmt::Error;
// use std::str::FromStr;
//
// #[derive(Debug, Clone, Eq, PartialEq)]
// enum State {
//     Valid,
//     Broken,
//     Unknown,
// }
//
// impl TryFrom<char> for State {
//     type Error = anyhow::Error;
//
//     fn try_from(value: char) -> Result<Self, Self::Error> {
//         match value {
//             '#' => Ok(Broken),
//             '.' => Ok(Valid),
//             '?' => Ok(Unknown),
//             _ => Err(Error.into()),
//         }
//     }
// }
//
// #[derive(Debug, Clone)]
// struct Row {
//     springs: Vec<State>,
//     sizes: Vec<usize>,
// }
//
// impl Row {
//     #[inline(always)]
//     fn is_valid(&self) -> bool {
//         let mut states = self.springs.iter().peekable();
//         self.sizes.iter().all(|size| {
//             while let Some(_) = states.next_if(|&c| c == &Valid) {}
//             (0..*size).all(|_| states.next() == Some(&Broken)) && (states.peek() != Some(&&Broken))
//         }) && states.all(|c| c == &Valid)
//     }
//
//     /// Recursively get the combination when replacing the first element by
//     /// a broken one then by a valid one
//     fn build_recursively(&mut self) -> usize {
//         if self.springs.is_empty() || self.sizes.is_empty() {
//             return self.nb_arrangements();
//         }
//         self.springs[0] = Broken;
//         let nb_broken = self.nb_arrangements();
//         let nb_to_remove = 1 + self
//             .springs
//             .iter()
//             .skip(1)
//             .skip_while(|&i| i == &Valid)
//             .count();
//         self.springs.drain(0..nb_to_remove);
//         let nb_ok = self.nb_arrangements();
//         nb_ok + nb_broken
//     }
//
//     #[inline(always)]
//     fn remove_trailing_point(&mut self) {
//         let count = self.springs.iter().take_while(|&i| i == &Valid).count();
//         if count > 0 {
//             self.springs.drain(..count);
//         }
//     }
//
//     fn combinations(&self) -> impl ParallelIterator<Item = Row> + '_ + Clone {
//         let indexes = self
//             .springs
//             .iter()
//             .enumerate()
//             .filter_map(|(ind, state)| match state {
//                 Unknown => Some(ind),
//                 _ => None,
//             })
//             .collect_vec();
//         (0..2_u32.pow(indexes.len() as u32))
//             .into_par_iter()
//             .map(move |mask| {
//                 let mut clone = self.clone();
//                 indexes.iter().enumerate().for_each(|(idx, &n)| {
//                     clone.springs[n] = match (mask >> idx) & 1 {
//                         0 => Valid,
//                         _ => Broken,
//                     };
//                 });
//                 clone
//             })
//     }
// }
//
// impl FromStr for Row {
//     type Err = anyhow::Error;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut parts = s.split(' ');
//         let springs = parts
//             .next()
//             .unwrap()
//             .chars()
//             .map(|c| c.try_into().unwrap())
//             .collect();
//         let sizes = parts
//             .next()
//             .unwrap()
//             .split(',')
//             .map(|n| n.parse().unwrap())
//             .collect();
//         Ok(Self { springs, sizes })
//     }
// }

pub fn part_one(_input: &str) -> Option<u32> {
    // This is correct, but takes too much time to execute
    // Fuck it, I will do it later *maybe*

    // let res = input
    //     .par_lines()
    //     .map(|line| line.parse::<Row>().unwrap())
    //     .map(|row| row.combinations().filter(|c| c.is_valid()).count() as u32)
    //     .sum::<u32>();
    // Some(res)

    None
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
