advent_of_code::solution!(12);
// use anyhow::Result;
// use itertools::Itertools;
// use std::cmp::min;
// use std::str::FromStr;
//
// #[derive(Debug, Clone)]
// struct Row {
//     springs: String,
//     sizes: Vec<usize>,
// }
//
// impl Row {
//     fn is_valid(&self) -> bool {
//         let mut chars = self.springs.chars().peekable();
//         let sizes_ok = self.sizes.iter().all(|size| {
//             while let Some(_) = chars.next_if(|&c| c == '.') {}
//             (0..*size).all(|_| chars.next() == Some('#')) && (chars.peek() != Some(&'#'))
//         });
//         let no_remaining_sharp = chars.all(|c| c == '.');
//         sizes_ok && no_remaining_sharp
//     }
//
//     fn test_sharp_and_point(&mut self) -> usize {
//         if self.springs.is_empty() || self.sizes.is_empty() {
//             return self.nb_arrangements();
//         }
//         self.springs.replace_range(0..1, "#");
//         let nb_with_sharp = self.nb_arrangements();
//         self.springs = self
//             .springs
//             .chars()
//             .skip(1)
//             .skip_while(|&c| c == '.')
//             .collect();
//         let nb_with_point = self.nb_arrangements();
//         nb_with_point + nb_with_sharp
//     }
//
//     fn remove_trailing_point(&mut self) {
//         if self.springs.starts_with('.') {
//             self.springs = self.springs.chars().skip_while(|&c| c == '.').collect();
//         }
//     }
//
//     fn nb_arrangements(&self) -> usize {
//         if self.is_valid() {
//             return 1;
//         } else if self.springs.chars().all(|c| c != '?') {
//             // everything is known, but it is incorrect
//             return 0;
//         }
//         if self.sizes.is_empty() {
//             return if self.springs.chars().any(|c| c == '#') {
//                 0
//             } else {
//                 1
//             };
//         } else if self.springs.len() < self.sizes.iter().sum() {
//             return 0;
//         }
//
//         let nb_sharps = self.springs.chars().take_while(|&c| c == '#').count();
//         if nb_sharps == 0 {
//             let mut cloned = self.clone();
//             cloned.test_sharp_and_point()
//         } else if (1..self.sizes[0]).contains(&nb_sharps) {
//             if self.springs.chars().take(self.sizes[0]).any(|c| c == '.') {
//                 0
//             } else {
//                 // if here, the next char to explore must be a '#'
//                 let mut cloned = self.clone();
//                 cloned
//                     .springs
//                     .replace_range(0..=min(cloned.sizes[0], cloned.springs.len() - 1), "");
//                 cloned.remove_trailing_point();
//                 cloned.sizes.remove(0);
//                 cloned.test_sharp_and_point()
//             }
//         } else if self.sizes[0] == nb_sharps {
//             let mut cloned = self.clone();
//             cloned
//                 .springs
//                 .replace_range(0..=min(cloned.sizes[0], cloned.springs.len() - 1), "");
//             cloned.remove_trailing_point();
//             cloned.sizes.remove(0);
//             cloned.test_sharp_and_point()
//         } else {
//             0
//         }
//     }
//
//     fn combinations(&self) -> impl Iterator<Item = Row> + '_ + Clone {
//         let indexes = self
//             .springs
//             .chars()
//             .enumerate()
//             .filter_map(|(ind, c)| match c {
//                 '?' => Some(ind),
//                 _ => None,
//             })
//             .collect_vec();
//         (0..2_u32.pow(indexes.len() as u32)).map(move |mask| {
//             let mut clone = self.clone();
//             indexes.iter().enumerate().for_each(|(idx, &n)| {
//                 let s = match (mask >> idx) & 1 {
//                     0 => ".",
//                     _ => "#",
//                 };
//                 clone.springs.replace_range(n..=n, s);
//             });
//             clone
//         })
//     }
// }
//
// impl FromStr for Row {
//     type Err = anyhow::Error;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut parts = s.split(' ');
//         let springs = parts.next().unwrap().to_string();
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
    //     .lines()
    //     .map(|line| line.parse::<Row>().unwrap())
    //     .map(|row| {
    //         row.combinations() /*.filter(|c| c.is_valid())*/
    //             .count() as u32
    //     })
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
